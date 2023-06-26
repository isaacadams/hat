use super::{Endpoint, HttpClient, UtilError};
use reqwest::header::HeaderMap;

#[derive(Debug)]
pub struct RequestBuilder {
    endpoint: Endpoint,
    headers: HeaderMap,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new(method: &str, url: &str) -> Result<Self, UtilError> {
        Ok(Self::from_endpoint(Endpoint::new(url, method)?))
    }

    pub fn from_endpoint(endpoint: Endpoint) -> Self {
        Self {
            endpoint,
            headers: HeaderMap::new(),
            body: None,
        }
    }

    pub fn add_header(&mut self, name: &str, value: &str) -> Result<(), UtilError> {
        let name = reqwest::header::HeaderName::from_lowercase(name.to_lowercase().as_bytes())?;
        let value = value.parse()?;
        self.headers.insert(name, value);

        Ok(())
    }

    pub fn add_body(&mut self, body: String) {
        self.body = Some(body);
    }

    #[allow(dead_code)]
    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    pub fn has_headers(&self) -> bool {
        !self.headers.is_empty()
    }

    #[allow(dead_code)]
    pub fn into_body(self) -> Option<String> {
        self.body
    }

    pub fn build(self, client: &HttpClient) -> reqwest::blocking::RequestBuilder {
        let has_headers = self.has_headers();
        let mut builder = self.endpoint.builder(client);

        if has_headers {
            builder = builder.headers(self.headers);
        }

        if let Some(b) = self.body {
            builder.body(b)
        } else {
            builder
        }
    }
}
