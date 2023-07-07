#[derive(thiserror::Error, Debug)]
pub enum UtilError {
    #[error("{0} is not a valid rest method")]
    InvalidRestMethod(String),
    #[error("{0}")]
    InvalidUrl(String),
    #[error("{0}")]
    IO(#[from] std::io::Error),
}
