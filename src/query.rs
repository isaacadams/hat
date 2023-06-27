pub trait Queryable {
    fn query(&self, filter: &str) -> Option<String>;
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum BodyContent {
    Json(String),
    Xml(String),
    Plaintext(String),
}

impl Queryable for BodyContent {
    // pass in arbitrary filter to extract data from body
    // e.g. Json -> filter = ".posts.[0]"
    // e.g. Plaintext -> filter = "/\w+/g"
    fn query(&self, filter: &str) -> Option<String> {
        match self {
            BodyContent::Json(json) => Some(gjson::get(json, filter).str().to_string()),
            BodyContent::Xml(_) => todo!(),
            BodyContent::Plaintext(text) => Some(text.to_owned()),
        }
    }
}

impl BodyContent {
    pub fn new(content: String) -> Self {
        if gjson::valid(&content) {
            return BodyContent::Json(content);
        }

        log::debug!("{}", content);

        BodyContent::Plaintext(content)
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
        let content = BodyContent::new(test.to_string());
        assert_eq!(content.query("message"), Some("Hello World".to_string()));
    }

    #[test]
    fn nested_json_field_query() {
        let content = BodyContent::new(test.to_string());
        assert_eq!(content.query("user.name"), Some("Isaac Adams".to_string()));
        assert_eq!(content.query("user.id"), Some("0".to_string()));
    }

    #[test]
    fn nested_json_array_query() {
        let content = BodyContent::new(test.to_string());
        assert_eq!(content.query("user.name"), Some("Isaac Adams".to_string()));
        assert_eq!(
            content.query("user.languages.1"),
            Some("typescript".to_string())
        );
    }
}
