use super::{error::UtilError, UtilResult};
use std::str::FromStr;

#[derive(Debug)]
pub struct Endpoint {
    url: reqwest::Url,
    method: reqwest::Method,
}

impl Endpoint {
    fn parse_method(method: &str) -> UtilResult<reqwest::Method> {
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
            _ => return Err(UtilError::InvalidRestMethod(method)),
        };

        Ok(method)
    }

    fn parse_url(url: &str) -> UtilResult<reqwest::Url> {
        match reqwest::Url::from_str(url) {
            Ok(u) => Ok(u),
            Err(e) => Err(UtilError::InvalidUrl(format!(
                "url failed: '{}'\nreason: {}",
                url, e
            ))),
        }
    }

    pub fn new(url: &str, method: &str) -> UtilResult<Self> {
        let method = Self::parse_method(method)?;
        let url = Self::parse_url(url)?;
        Ok(Self { url, method })
    }

    pub fn builder(self, client: &reqwest::blocking::Client) -> reqwest::blocking::RequestBuilder {
        client.request(self.method, self.url)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn endpoint_parsing_works() -> UtilResult<()> {
        let url = "https://google.com";
        for m in vec!["get", "post", "put", "delete", "patch", "head", "options"] {
            assert!(Endpoint::new(url, m).is_ok());
            assert!(Endpoint::new(url, m.to_uppercase().as_ref()).is_ok());
        }

        assert_eq!(Endpoint::parse_method("get")?, reqwest::Method::GET);

        assert!(matches!(
            Endpoint::new(url, "invalid"),
            Err(UtilError::InvalidRestMethod(_))
        ));

        assert!(matches!(
            Endpoint::new("not-a-valid-url", "get"),
            Err(UtilError::InvalidUrl(_))
        ));

        Ok(())
    }
}
