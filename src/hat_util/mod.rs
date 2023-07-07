mod endpoint;
mod error;
mod request_builder;
mod store;

pub use {
    endpoint::Endpoint,
    error::UtilError,
    request_builder::RequestBuilder,
    store::{Store, StoreComposed, StoreUnion},
};

pub trait Assert {
    fn assert(&self, buffer: &mut String) -> bool;
}

type UtilResult<T> = Result<T, UtilError>;
