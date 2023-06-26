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

use reqwest::blocking::{Client, Response};
pub type HttpClient = Client;
pub type HttpResponse = Response;
pub type HttpError = reqwest::Error;

pub trait RequestExecutor {
    fn execute(&self, request: RequestBuilder) -> Result<HttpResponse, HttpError>;
}

type UtilResult<T> = Result<T, UtilError>;
