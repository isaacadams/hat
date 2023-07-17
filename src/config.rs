use crate::{
    assertion,
    error::HatError,
    factory,
    runner::{HatTestBuilder, HatTestOutput, RequestExecutor},
    store::Store,
};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

pub fn read<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let buffer = std::fs::read_to_string(path.as_ref()).with_context(|| {
        let path = path.as_ref().to_string_lossy();
        match std::env::current_dir() {
            Ok(cwd) => format!(
                "could not find {}\ncurrent working directory: {}\n",
                path,
                cwd.to_string_lossy(),
            ),
            Err(e) => format!(
                "could not find {}\nfailed to resolve current working directory\ncause: {}",
                path, e
            ),
        }
    })?;

    let toml: Config = toml::de::from_str(&buffer).with_context(|| {
        let path = path.as_ref().to_string_lossy();
        format!("{} has invalid schema", path)
    })?;

    Ok(toml)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub environment: HashMap<String, String>,
    pub tests: Vec<TestConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestConfig {
    description: Option<String>,
    http: String,
    assertions: String,
    outputs: Option<HashMap<String, String>>,
}

impl HatTestBuilder for TestConfig {
    fn build<T: Store + RequestExecutor>(
        self,
        hat: &T,
        buffer: &mut String,
    ) -> anyhow::Result<HatTestOutput> {
        match build(self, hat, buffer) {
            Ok(t) => Ok(t),
            Err(e) => Err(HatError::TestFailedToBuild(e.to_string()).into()),
        }
    }
}

fn build<T: Store + RequestExecutor>(
    hat_test_config: TestConfig,
    hat: &T,
    _: &mut String,
) -> Result<HatTestOutput, HatError> {
    // extract the raw http request from config
    // can either be a path to an .http file or the raw http request
    let http_contents = crate::http_file::get_contents(hat_test_config.http)?;
    // replace variables in raw http request
    let http_contents = hat.match_and_replace(http_contents.as_str(), |v| v.as_value());
    log::debug!("HTTP: {}", &http_contents);

    // parses the raw http request into something the http client can use
    let request = crate::http_file::parse(http_contents.as_str())?;
    let method = request.get_method().to_string();
    let response = hat.execute(request)?;
    let response_info = format!(
        "{} {} {} {} {}",
        response.status(),
        response.status_text(),
        method,
        response.get_url(),
        response.http_version()
    );

    log::info!("{}", &response_info);
    log::debug!("{:#?}", &response);

    // these stores contain the data from the response headers and body
    // these should not persist across other tests unless specified in the `output` config
    // any persistent store data gets handled at the end in `factory::outputs(...)`
    let response_store = factory::response(response)?;
    let store_composed = hat.compose(&response_store);

    let assertions =
        store_composed.match_and_replace(&hat_test_config.assertions, |v| v.as_literal());
    let assert = assertion::new(response_info, hat_test_config.description, assertions);

    let outputs = match hat_test_config.outputs {
        Some(o) => Some(factory::outputs(&store_composed, o)?),
        None => None,
    };

    Ok((assert, outputs))
}
