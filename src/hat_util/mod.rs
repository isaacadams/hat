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

type UtilResult<T> = Result<T, UtilError>;
