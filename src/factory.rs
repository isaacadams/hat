use crate::error::HatError;
use crate::hat_util::{Store, StoreMap, StoreUnion};
use std::collections::HashMap;

pub fn outputs<S: Store>(
    store: &S,
    outputs_from_config: HashMap<String, String>,
) -> Result<StoreUnion, HatError> {
    // create map for outputs
    // ensure variables used in outputs are hydrated from the latest store
    // ?? evaluate the hydrated output content?
    // finally, assign it to the output key
    let mut evaluated_outputs = StoreMap::default();
    for (key, value) in outputs_from_config.into_iter() {
        let value = store.match_and_replace(&value);
        evaluated_outputs.insert(key, serde_json::to_value(&value)?);
    }

    log::info!("{:#?}", evaluated_outputs);

    Ok(StoreUnion::MapStringToJsonValue(evaluated_outputs))
}

pub fn response(response: reqwest::blocking::Response) -> Result<StoreUnion, HatError> {
    let mut store = StoreMap::default();

    let _ = internal::store_from_response(&mut store, &response);
    let _ = internal::store_from_response_body(&mut store, response);

    Ok(StoreUnion::MapStringToJsonValue(store))
}

mod internal {
    use crate::error::HatError;
    use crate::hat_util::StoreMap;

    pub fn store_from_response(
        buffer: &mut StoreMap,
        response: &reqwest::blocking::Response,
    ) -> Result<(), HatError> {
        buffer.insert(
            "r.status".to_string(),
            serde_json::to_value(response.status().as_u16()).unwrap(),
        );

        let headers = response.headers();

        if headers.is_empty() {
            return Ok(());
        }

        for (key, value) in headers.iter().filter(|(_, v)| !v.is_empty()) {
            let key = format!("r.headers.{}", key);
            let value = serde_json::to_value(
                value
                    .to_str()
                    .map_err(|e| HatError::ParsingError(e.to_string()))?,
            )?;

            let mut parse_buffer = Vec::new();
            self::parse(&mut parse_buffer, key, value);
            for (k, v) in parse_buffer {
                buffer.insert(k, v);
            }
        }

        Ok(())
    }

    pub fn store_from_response_body(
        buffer: &mut StoreMap,
        response: reqwest::blocking::Response,
    ) -> Result<(), HatError> {
        let text = response.text().map(|text| {
            log::info!("BODY: {}", &text);
            buffer.insert(
                String::from("r.body"),
                serde_json::Value::from(text.clone()),
            );
            text
        })?;

        let _ = text.parse().map(|json| {
            let mut parse_buffer = Vec::new();
            self::parse(&mut parse_buffer, String::from("r.body"), json);
            for v in parse_buffer {
                buffer.insert(v.0, v.1);
            }
        });

        log::info!("STORE: {:#?}", &buffer);

        Ok(())
    }

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
