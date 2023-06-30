pub trait Queryable {
    fn query(&self, filter: &str) -> Option<String>;
}

impl<Q: Queryable> crate::hat_util::Store for Q {
    fn fetch_value(&self, key: &str) -> Option<String> {
        self.query(key)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Content {
    Json(String),
    Xml(String),
    Plaintext(String),
}

impl Queryable for Content {
    // pass in arbitrary filter to extract data from body
    // e.g. Json -> filter = ".posts.[0]"
    // e.g. Plaintext -> filter = "/\w+/g"
    fn query(&self, filter: &str) -> Option<String> {
        log::debug!("{:#?}", &self);
        match self {
            Content::Json(json) => {
                let value = gjson::get(json, filter);
                Some(match value.kind() {
                    gjson::Kind::Null => return None,
                    gjson::Kind::String => value.to_string(),
                    gjson::Kind::False => todo!(),
                    gjson::Kind::True => todo!(),
                    gjson::Kind::Number => value.str().to_string(),
                    gjson::Kind::Array => todo!(),
                    gjson::Kind::Object => todo!(),
                })
            }
            Content::Xml(_) => todo!(),
            Content::Plaintext(text) => Some(text.to_owned()),
        }
    }
}

impl Content {
    pub fn new(content: String) -> Self {
        if gjson::valid(&content) {
            return Content::Json(content);
        }

        log::debug!("{}", content);

        Content::Plaintext(content)
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

    const test: &str = r#"{ 
        "message": "Hello World",
        "user": { 
            "id": 0, 
            "name": "Isaac Adams", 
            "username": "iadams",
            "languages": ["rust", "typescript", "csharp"]
        } 
    }"#;

    #[test]
    fn basic_json_field_query() {
        let content = Content::new(test.to_string());
        assert_eq!(content.query("message"), Some("Hello World".to_string()));
    }

    #[test]
    fn nested_json_field_query() {
        let content = Content::new(test.to_string());
        assert_eq!(content.query("user.name"), Some("Isaac Adams".to_string()));
        assert_eq!(content.query("user.id"), Some("0".to_string()));
    }

    #[test]
    fn nested_json_array_query() {
        let content = Content::new(test.to_string());
        assert_eq!(content.query("user.name"), Some("Isaac Adams".to_string()));
        assert_eq!(
            content.query("user.languages.1"),
            Some("typescript".to_string())
        );
    }
}
