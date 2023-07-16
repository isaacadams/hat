use super::{error::UtilError, UtilResult};
use http::Method as HttpMethod;
use std::str::FromStr;

#[derive(Debug)]
pub struct Endpoint {
    url: url::Url,
    method: (String, HttpMethod),
}

impl Endpoint {
    fn parse_method(method: &str) -> UtilResult<(String, HttpMethod)> {
        let method = method.to_uppercase();
        let http_method = match method.as_ref() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "PATCH" => HttpMethod::PATCH,
            "DELETE" => HttpMethod::DELETE,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            _ => return Err(UtilError::InvalidRestMethod(method)),
        };

        Ok((method, http_method))
    }

    fn parse_url(url: &str) -> UtilResult<url::Url> {
        match url::Url::from_str(url) {
            Ok(u) => Ok(u),
            Err(e) => Err(UtilError::InvalidUrl(format!(
                "url failed: '{}'\nreason: {}",
                url, e
            ))),
        }
    }

    pub fn get_url_as_str(&self) -> &str {
        self.url.as_str()
    }

    #[allow(dead_code)]
    pub fn get_url(&self) -> &url::Url {
        &self.url
    }

    pub fn get_method(&self) -> &str {
        &self.method.0
    }

    pub fn new(url: &str, method: &str) -> UtilResult<Self> {
        let method = Self::parse_method(method)?;
        let url = Self::parse_url(url)?;
        Ok(Self { url, method })
    }

    pub fn builder(&self, builder: http::request::Builder) -> http::request::Builder {
        builder.method(&self.method.1).uri(&self.url.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn endpoint_parsing_works() -> UtilResult<()> {
        let url = "https://google.com";
        for m in vec!["get", "post", "put", "delete", "patch", "head", "options"] {
            let endpoint = Endpoint::new(url, m)?;
            assert_eq!(endpoint.method.0, m.to_uppercase());
            assert!(Endpoint::new(url, m.to_uppercase().as_ref()).is_ok());
        }

        assert_eq!(Endpoint::parse_method("get")?.1, HttpMethod::GET);

        assert!(matches!(
            Endpoint::new(url, "invalid-method"),
            Err(UtilError::InvalidRestMethod(_))
        ));

        assert!(matches!(
            Endpoint::new("not-a-valid-url", "GET"),
            Err(UtilError::InvalidUrl(_))
        ));

        Ok(())
    }
}
