#[derive(thiserror::Error, Debug)]
pub enum HatError {
    #[error("{0:#}")]
    IO(#[from] std::io::Error),
    #[error("{0:#}")]
    HttpLexer(#[from] crate::http_file_parser::HttpLexerError),
    #[error("{0:#}")]
    TomlError(#[from] toml::de::Error),
    #[error("{0:#}")]
    Anyhow(#[from] anyhow::Error),
    #[error("test failed to build before execution, cause: {0}")]
    TestFailedToBuild(String),
    #[error("failed to build request")]
    RequestBuilder,
    #[error("response failed: {0}")]
    HttpResponse(String),
}
