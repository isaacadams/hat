use std::str::FromStr;

mod config;

#[derive(thiserror::Error, Debug)]
enum RatError {
    #[error("{0} is not a valid rest method")]
    InvalidRestMethod(String),
    #[error("selector was not found: {0}")]
    InvalidSelector(String),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    InvalidUrl(String),
}

fn main() -> Result<(), RatError> {
    let contents = std::fs::read("config.json").unwrap();
    let json: serde_json::Value = serde_json::from_slice(&contents).unwrap();

    let selected = parse(String::from("environment.base"), json)?;
    println!("{:?}", selected);

    let response = reqwest::blocking::get(selected)?;
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    println!("{:#?}", response.headers().get("content-type"));

    Ok(())
}

fn parse(selector: String, json: serde_json::Value) -> Result<String, RatError> {
    let parts = selector.split(".");
    let mut selected = &json;
    for p in parts {
        selected = &selected[p];
    }

    match selected.as_str() {
        Some(v) => Ok(v.to_string()),
        None => Err(RatError::InvalidSelector(selector)),
    }
}

struct Test {
    client: reqwest::blocking::Client,
}

struct Endpoint {
    url: reqwest::Url,
    method: reqwest::Method,
}

impl Endpoint {
    fn parse_method(method: &str) -> Result<reqwest::Method, RatError> {
        use reqwest::Method;
        let method = method.to_lowercase();
        let method = match method.as_ref() {
            "get" => Method::GET,
            "post" => Method::POST,
            "put" => Method::PUT,
            "patch" => Method::PATCH,
            "delete" => Method::DELETE,
            "head" => Method::HEAD,
            "options" => Method::OPTIONS,
            _ => return Err(RatError::InvalidRestMethod(method)),
        };

        Ok(method)
    }

    fn parse_url(url: &str) -> Result<reqwest::Url, RatError> {
        match reqwest::Url::from_str(url) {
            Ok(u) => Ok(u),
            Err(e) => return Err(RatError::InvalidUrl(e.to_string())),
        }
    }

    fn new(url: &str, method: &str) -> Result<Self, RatError> {
        let method = Self::parse_method(method)?;
        let url = Self::parse_url(url)?;
        Ok(Self { url, method })
    }

    fn build(self) -> reqwest::blocking::Request {
        reqwest::blocking::Request::new(self.method, self.url)
    }
}

impl Test {
    fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }

    fn run(
        self,
        request: reqwest::blocking::Request,
    ) -> Result<reqwest::blocking::Response, crate::RatError> {
        let r = self.client.execute(request)?;

        Ok(r)
    }
}

#[cfg(test)]
mod test {
    use crate::{Endpoint, RatError};

    #[test]
    fn endpoint_parsing_works() -> Result<(), RatError> {
        let url = "https://google.com";
        for m in vec!["get", "post", "put", "delete", "patch", "head", "options"] {
            assert!(Endpoint::new(url, m).is_ok());
            assert!(Endpoint::new(url, m.to_uppercase().as_ref()).is_ok());
        }

        assert_eq!(Endpoint::parse_method("get")?, reqwest::Method::GET);

        assert!(matches!(
            Endpoint::new(url, "invalid"),
            Err(RatError::InvalidRestMethod(_))
        ));

        assert!(matches!(
            Endpoint::new("not-a-valid-url", "get"),
            Err(RatError::InvalidUrl(_))
        ));

        Ok(())
    }

    #[test]
    fn config_parsing_works() {
        let contents = std::fs::read("./config.json").unwrap();
        let config: crate::config::Config = serde_json::from_slice(&contents).unwrap();
        println!("{:#?}", config);
    }
}
