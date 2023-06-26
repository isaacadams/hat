#[derive(thiserror::Error, Debug)]
pub enum HatError {
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("{0}")]
    ParsingError(String),
    #[error("{0}")]
    HttpLexer(#[from] crate::http_file_parser::HttpLexerError),
    #[error("{0}")]
    TomlError(#[from] toml::de::Error),
    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("test failed before execution, cause: {0}")]
    TestFailed(String),
}