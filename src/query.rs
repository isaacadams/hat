pub enum Variable<'a> {
    Json(gjson::Value<'a>),
    Text(String),
}

impl Variable<'_> {
    pub fn as_value(&self) -> String {
        match self {
            Variable::Json(value) => match value.kind() {
                gjson::Kind::Null => "null",
                _ => value.str(),
            },
            Variable::Text(x) => x,
        }
        .to_string()
    }

    pub fn as_literal(&self) -> String {
        match self {
            Variable::Json(value) => match value.kind() {
                gjson::Kind::String => format!("\"{}\"", value.str()),
                _ => self.as_value(),
            },
            Variable::Text(x) => format!("\"{}\"", x),
        }
    }
}

#[derive(Debug)]
pub enum Content {
    Json(String),
    #[allow(dead_code)]
    Xml(String),
    Plaintext(String),
}

impl Content {
    pub fn new(content: String) -> Self {
        if gjson::valid(&content) {
            return Content::Json(content);
        }

        log::debug!("{}", content);

        Content::Plaintext(content)
    }

    // pass in arbitrary filter to extract data from body
    // e.g. Json -> filter = ".posts.[0]"
    // e.g. Plaintext -> filter = "/\w+/g"
    pub fn query<'a>(&'a self, filter: &'a str) -> Option<Variable<'a>> {
        log::debug!("{:#?}", &self);

        match self {
            Content::Json(json) => {
                let value = gjson::get(json, filter);
                Some(Variable::Json(value))
            }
            Content::Xml(_) => todo!(),
            Content::Plaintext(text) => Some(Variable::Text(text.to_string())),
        }
    }

    pub fn value(&self) -> Option<Variable<'_>> {
        log::debug!("{:#?}", &self);

        match self {
            Content::Json(json) => {
                let value = gjson::parse(json);
                Some(Variable::Json(value))
            }
            Content::Xml(_) => todo!(),
            Content::Plaintext(text) => Some(Variable::Text(text.to_string())),
        }
    }

    #[allow(dead_code)]
    pub fn parse_filter<F: FnOnce(&str) -> ()>(
        key: &str,
        found_key: F,
        found_filter: F,
    ) -> Option<()> {
        let mut iter = key.split("|");

        let key = iter.next()?.trim();
        found_key(key);

        let filter = iter.next()?.trim();
        found_filter(filter);

        Some(())
    }
}

/// query a json value
/// if the query hits a path that does not exist, this function returns "null"
#[allow(dead_code)]
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

    const TEST: &str = r#"{ 
        "message": "Hello World",
        "user": { 
            "id": 0, 
            "name": "Isaac Adams", 
            "username": "iadams",
            "languages": ["rust", "typescript", "csharp"]
        } 
    }"#;

    #[test]
    fn basic_json_field_query() -> Result<(), String> {
        let content = Content::new(TEST.to_string());
        let query = content.query("message").ok_or("failed")?;

        assert_eq!(query.as_value(), "Hello World");

        Ok(())
    }

    #[test]
    fn nested_json_field_query() -> Result<(), String> {
        let content = Content::new(TEST.to_string());
        let query1 = content.query("user.name").ok_or("failed")?;
        let query2 = content.query("user.id").ok_or("failed")?;

        assert_eq!(query1.as_value(), "Isaac Adams");
        assert_eq!(query2.as_value(), "0");

        Ok(())
    }

    #[test]
    fn nested_json_array_query() -> Result<(), String> {
        let content = Content::new(TEST.to_string());
        let query1 = content.query("user.name").ok_or("failed")?;
        let query2 = content.query("user.languages.1").ok_or("failed")?;

        assert_eq!(query1.as_value(), "Isaac Adams");
        assert_eq!(query2.as_value(), "typescript");

        Ok(())
    }

    #[test]
    fn status_codes_as_json() -> Result<(), String> {
        let content = Content::Json("200".to_string());
        let query = content.value().ok_or("failed")?;

        assert_eq!(query.as_value(), "200");

        Ok(())
    }

    #[test]
    fn headers_as_json() -> Result<(), String> {
        let content = Content::Json(
            "{\"content-length\":\"175\",\"date\":\"Thu, 06 Jul 2023 22:07:07 GMT\"}".to_string(),
        );
        let query = content.query("content-length").ok_or("failed")?;

        assert_eq!(query.as_value(), "175");

        Ok(())
    }
}
