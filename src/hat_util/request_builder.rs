use super::{Endpoint, UtilError};

#[derive(Debug)]
pub struct RequestBuilder {
    endpoint: Endpoint,
    body: Option<String>,
    inner: http::request::Builder,
}

impl RequestBuilder {
    pub fn new(method: &str, url: &str) -> Result<Self, UtilError> {
        Ok(Self::from_endpoint(Endpoint::new(url, method)?))
    }

    pub fn from_endpoint(endpoint: Endpoint) -> Self {
        Self {
            endpoint,
            body: None,
            inner: http::request::Builder::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.inner
            .headers_ref()
            .and_then(|h| h.get(key))
            .and_then(|v| v.to_str().ok())
    }

    #[allow(dead_code)]
    pub fn get_url(&self) -> &str {
        self.endpoint.get_url()
    }

    pub fn get_method(&self) -> &str {
        self.endpoint.get_method()
    }

    pub fn add_header(mut self, name: &str, value: &str) -> Self {
        self.inner = self.inner.header(name, value);
        self
    }

    pub fn add_body(&mut self, body: String) {
        self.body = Some(body);
    }

    #[allow(dead_code)]
    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    #[allow(dead_code)]
    pub fn has_headers(&self) -> bool {
        match self.inner.headers_ref() {
            Some(headers) => !headers.is_empty(),
            None => false,
        }
    }

    #[allow(dead_code)]
    pub fn into_body(self) -> Option<String> {
        self.body
    }

    pub fn split(self) -> (http::request::Builder, Endpoint, Option<String>) {
        (self.inner, self.endpoint, self.body)
    }

    pub fn build(
        mut builder: http::request::Builder,
        endpoint: Endpoint,
        client: &ureq::Agent,
    ) -> Option<ureq::Request> {
        builder = endpoint.builder(builder);

        let mut ureq_request = client.request(
            builder.method_ref().map(|m| m.as_str())?,
            &builder.uri_ref().map(|u| u.to_string())?,
        );

        if let Some(headers) = builder.headers_ref() {
            ureq_request = headers
                .iter()
                .filter_map(|header| {
                    header
                        .1
                        .to_str()
                        .ok()
                        .map(|str_value| (header.0.as_str(), str_value))
                })
                .fold(ureq_request, |request, header| {
                    request.set(header.0, header.1)
                });
        }

        Some(ureq_request)
    }
}
