mod parser;

#[derive(thiserror::Error, Debug)]
pub enum HttpLexerError {
    #[error(".http rule violated: {0}")]
    MalformedHttpFile(&'static str),
    #[error("\n[Ln {row}, Col {col}] => '{content}' is invalid\n{reason}")]
    MalformedLine {
        row: usize,
        col: usize,
        content: String,
        reason: std::borrow::Cow<'static, str>,
    },
    #[error("{0:#?}")]
    UtilError(#[from] crate::hat_util::UtilError),
    #[error("{0}")]
    IO(#[from] std::io::Error),
}

pub use parser::{parse, parse_from_path, parse_from_utf8};

pub fn get_contents(input: String) -> Result<String, HttpLexerError> {
    Ok(if input.ends_with(".http") {
        std::fs::read_to_string(input.as_str())?
    } else {
        input
    })
}
