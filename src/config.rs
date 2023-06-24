use crate::hat_util::{RequestExecutor, Store};
use crate::{
    assertion,
    error::HatError,
    factory,
    runner::{HatTestBuilder, HatTestOutput},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

pub fn read<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let buffer = std::fs::read(path.as_ref())?; //"./tests/config.toml"
    let toml: Config = toml::from_str(String::from_utf8_lossy(&buffer).as_ref())?;

    Ok(toml)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub environment: HashMap<String, serde_json::Value>,
    pub tests: Vec<TestConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestConfig {
    name: String,
    http: String,
    assertions: String,
    outputs: Option<HashMap<String, String>>,
}

impl HatTestBuilder for TestConfig {
    fn build<T: Store + RequestExecutor>(self, hat: &T) -> anyhow::Result<HatTestOutput> {
        match build(self, hat) {
            Ok(t) => Ok(t),
            Err(e) => Err(HatError::TestFailed(e.to_string()).into()),
        }
    }
}

fn build<T: Store + RequestExecutor>(
    hat_test_config: TestConfig,
    hat: &T,
) -> Result<HatTestOutput, HatError> {
    // extract the raw http request from config
    // can either be a path to an .http file or the raw http request
    let http_contents = crate::http_file_parser::get_contents(hat_test_config.http)?;
    // replace variables in raw http request
    let http_contents = hat.match_and_replace(http_contents.as_str());
    log::debug!("{}", &http_contents);

    // parses the raw http request into something the http client can use
    let request = crate::http_file_parser::parse(http_contents.as_str())?;
    let response = hat.execute(request)?;
    log::info!("{} {}", response.status(), response.url());
    log::debug!("{:#?}", &response);

    // these stores contain the data from the response headers and body
    // these should not persist across other tests unless specified in the `output` config
    // any persistent store data gets handled at the end in `factory::outputs(...)`
    let response_store = factory::response(response)?;
    let store_composed = hat.compose(&response_store);

    let assertions = store_composed.match_and_replace(&hat_test_config.assertions);
    let assert = assertion::new(hat_test_config.name, assertions);

    let outputs = match hat_test_config.outputs {
        Some(o) => Some(factory::outputs(&store_composed, o)?),
        None => None,
    };

    Ok((Box::new(assert), outputs))
}
