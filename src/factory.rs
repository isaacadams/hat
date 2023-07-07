use crate::error::HatError;
use crate::hat_util::{Store, StoreUnion};
use crate::query::Content;
use std::collections::HashMap;

pub fn outputs<S: Store>(
    store: &S,
    outputs_from_config: HashMap<String, String>,
) -> Result<StoreUnion, HatError> {
    // create map for outputs
    // ensure variables used in outputs are hydrated from the latest store
    // ?? evaluate the hydrated output content?
    // finally, assign it to the output key
    let mut evaluated_outputs = HashMap::<String, Content>::default();
    for (key, value) in outputs_from_config.into_iter() {
        let value = store.match_and_replace(&value, |v| v.as_value().to_string());
        evaluated_outputs.insert(key, Content::new(value));
    }

    log::info!("OUTPUTS: {:#?}", evaluated_outputs);

    Ok(StoreUnion::MapStringToContent(evaluated_outputs))
}

pub fn response(response: reqwest::blocking::Response) -> Result<StoreUnion, HatError> {
    let mut store = HashMap::<String, Content>::default();

    let response_header = internal::store_from_response(&mut store, &response);
    if response_header.is_err() {
        log::error!("{:?}", response_header);
    }
    let response_body = store_from_response_body(&mut store, response);
    if response_body.is_err() {
        log::error!("{:?}", response_header);
    }

    Ok(StoreUnion::MapStringToContent(store))
}

pub fn store_from_response_body(
    buffer: &mut HashMap<String, Content>,
    response: reqwest::blocking::Response,
) -> Result<(), HatError> {
    let text = response.text()?;
    log::info!("BODY: {}", &text);

    if text.is_empty() {
        return Ok(());
    }

    let content = Content::new(text);
    log::info!("STORE: {:#?}", &content);

    buffer.insert("body".to_string(), content);

    Ok(())
}

mod internal {
    use crate::query::Content;
    use anyhow::Context;
    use std::collections::HashMap;

    pub fn store_from_response(
        buffer: &mut HashMap<String, Content>,
        response: &reqwest::blocking::Response,
    ) -> anyhow::Result<()> {
        buffer.insert(
            "status".to_string(),
            Content::Json(response.status().as_u16().to_string()),
        );

        let headers = response.headers();

        if headers.is_empty() {
            return Ok(());
        }

        let mut json = json::JsonValue::new_object();

        for (key, value) in headers.iter().filter(|(_, v)| !v.is_empty()) {
            let value = value
                .to_str()
                .with_context(|| format!("failed to stringify '{:?}'", value))?;

            // json::parse(...) will convert a "100" -> 100 in json
            // but will fail to parse a "hello world" -> \"hello world\" because it expects the double quote to be contained within the value
            // so first try json::parse(..) to correctly parse booleans, numbers, etc., then treat everything else like a string.
            let value = json::parse(value).unwrap_or(value.into());

            json[key.to_string()] = value;
        }

        let json = json.dump();
        buffer.insert("headers".to_string(), Content::Json(json));

        log::debug!("HEADERS: {:#?}", &buffer);

        Ok(())
    }

    #[allow(dead_code)]
    pub fn parse(
        buffer: &mut Vec<(String, serde_json::Value)>,
        key: String,
        value: serde_json::Value,
    ) {
        match value {
            serde_json::Value::Null => buffer.push((key, value)),
            serde_json::Value::Bool(_) => buffer.push((key, value)),
            serde_json::Value::Number(_) => buffer.push((key, value)),
            serde_json::Value::String(_) => buffer.push((key, value)),
            serde_json::Value::Array(x) => {
                for (i, item) in x.into_iter().enumerate() {
                    let key = format!("{}.[{}]", key, i);
                    self::parse(buffer, key, item);
                }
            }
            serde_json::Value::Object(x) => {
                for (name, field) in x {
                    let key = format!("{}.{}", key, name);
                    self::parse(buffer, key, field);
                }
            }
        };
    }
}
