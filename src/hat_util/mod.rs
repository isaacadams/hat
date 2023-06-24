mod endpoint;
mod error;
mod request_builder;
mod store;

pub use {
    endpoint::Endpoint,
    error::UtilError,
    request_builder::RequestBuilder,
    store::{Store, StoreComposed, StoreMap, StoreUnion},
};

pub trait Assert {
    fn assert(&self, buffer: &mut String) -> bool;
}

use reqwest::blocking::{Client, Request, Response};
pub type HttpClient = Client;
pub type HttpRequest = Request;
pub type HttpResponse = Response;
pub type HttpError = reqwest::Error;

pub trait RequestExecutor {
    fn execute(&self, request: RequestBuilder) -> Result<HttpResponse, HttpError>;
}

type UtilResult<T> = Result<T, UtilError>;

pub fn parse(selector: String, json: serde_json::Value) -> Result<String, UtilError> {
    let parts = selector.split('.');
    let mut selected = &json;
    for p in parts {
        selected = &selected[p];
    }

    match selected.as_str() {
        Some(v) => Ok(v.to_string()),
        None => Err(UtilError::InvalidSelector(selector)),
    }
}
