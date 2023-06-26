pub trait Queryable {
    fn query(&self, filter: &str) -> Option<String>;
}

#[allow(dead_code)]
pub enum BodyContent {
    Json(serde_json::Value),
    Xml(String),
    Plaintext(String),
}

impl Queryable for BodyContent {
    // pass in arbitrary filter to extract data from body
    // e.g. Json -> filter = ".posts.[0]"
    // e.g. Plaintext -> filter = "/\w+/g"
    fn query(&self, filter: &str) -> Option<String> {
        match self {
            BodyContent::Json(json) => {
                let mut filter = filter.split('.');
                let value = parse(&mut filter, &json);

                if &serde_json::Value::Null == value {
                    None
                } else {
                    Some(value.to_string())
                }
            }
            BodyContent::Xml(_) => todo!(),
            BodyContent::Plaintext(_) => todo!(),
        }
    }
}

/// query a json value
/// if the query hits a path that does not exist, this function returns "null"
pub fn parse<'a, 'b, I: Iterator<Item = &'a str>>(
    selector: &mut I,
    json: &'b serde_json::Value,
) -> &'b serde_json::Value {
    if let Some(key) = selector.next() {
        if let Ok(index) = key.parse::<usize>() {
            parse(selector, &json[index])
        } else {
            parse(selector, &json[key])
        }
    } else {
        json
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_json_field_query() {
        let content = BodyContent::Json(serde_json::json!({ "message": "Hello World" }));
        assert_eq!(
            content.query("message"),
            Some("\"Hello World\"".to_string())
        );
    }

    #[test]
    fn nested_json_field_query() {
        let content = BodyContent::Json(
            serde_json::json!({ "user": { "name": "Isaac Adams", "id": 0, "username": "iadams" } }),
        );
        assert_eq!(
            content.query("user.name"),
            Some("\"Isaac Adams\"".to_string())
        );
        assert_eq!(content.query("user.id"), Some("0".to_string()));
    }

    #[test]
    fn nested_json_array_query() {
        let content = BodyContent::Json(serde_json::json!({
            "user": {
                "name": "Isaac Adams",
                "id": 0,
                "username": "iadams",
                "languages": ["rust", "typescript", "csharp"]
            }
        }));
        assert_eq!(
            content.query("user.name"),
            Some("\"Isaac Adams\"".to_string())
        );
        assert_eq!(
            content.query("user.languages.1"),
            Some("\"typescript\"".to_string())
        );
    }

    #[test]
    fn query_json_parser() {
        let content =
            serde_json::json!({ "user": { "name": "Isaac Adams", "id": 0, "username": "iadams" } });

        let mut filter = "user.name".split('.');
        assert_eq!(parse(&mut filter, &content).to_string(), "\"Isaac Adams\"");
    }

    #[test]
    fn query_nonexistent_json_parser() {
        let content =
            serde_json::json!({ "user": { "name": "Isaac Adams", "id": 0, "username": "iadams" } });

        let mut filter = "address".split('.');
        assert_eq!(parse(&mut filter, &content).to_string(), "null");
    }
}
