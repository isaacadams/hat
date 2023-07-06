pub trait Queryable {
    fn query<'a>(&'a self, filter: &'a str) -> Option<Variable<'a>>;
}

pub enum Variable<'a> {
    Json(gjson::Value<'a>),
    Text(String),
}

impl Variable<'_> {
    pub fn as_value(&self) -> &str {
        match self {
            Variable::Json(value) => match value.kind() {
                gjson::Kind::Null => "null",
                gjson::Kind::String => value.str(),
                gjson::Kind::False => todo!(),
                gjson::Kind::True => todo!(),
                gjson::Kind::Number => value.str(),
                gjson::Kind::Array => todo!(),
                gjson::Kind::Object => todo!(),
            },
            Variable::Text(x) => x,
        }
    }

    pub fn as_literal(&self) -> &str {
        match self {
            Variable::Json(value) => match value.kind() {
                gjson::Kind::Null => todo!(),
                gjson::Kind::False => todo!(),
                gjson::Kind::Number => todo!(),
                gjson::Kind::String => todo!(),
                gjson::Kind::True => todo!(),
                gjson::Kind::Array => todo!(),
                gjson::Kind::Object => todo!(),
            },
            Variable::Text(x) => x,
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

impl Queryable for Content {
    // pass in arbitrary filter to extract data from body
    // e.g. Json -> filter = ".posts.[0]"
    // e.g. Plaintext -> filter = "/\w+/g"
    fn query<'a>(&'a self, filter: &'a str) -> Option<Variable<'a>> {
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
}

impl Content {
    pub fn new(content: String) -> Self {
        if gjson::valid(&content) {
            return Content::Json(content);
        }

        log::debug!("{}", content);

        Content::Plaintext(content)
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
}
