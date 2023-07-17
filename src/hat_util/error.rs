#[derive(thiserror::Error, Debug)]
pub enum UtilError {
    #[error("{0}")]
    IO(#[from] std::io::Error),
}
