use crate::hat_util::RequestBuilder;
use std::path::Path;

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

pub fn get_contents(input: String) -> Result<String, HttpLexerError> {
    Ok(if input.ends_with(".http") {
        std::fs::read_to_string(input.as_str())?
    } else {
        input
    })
}

pub fn parse(input: &str) -> Result<RequestBuilder, HttpLexerError> {
    if input.ends_with(".http") {
        parse_from_path(input)
    } else {
        parse_from_utf8(input)
    }
}

pub fn parse_from_path<P: AsRef<Path>>(path: P) -> Result<RequestBuilder, HttpLexerError> {
    let buffer = std::fs::read(path)?;
    let builder = parse_from_utf8(buffer)?;
    Ok(builder)
}

/// .http specification
/// follows RFC9110 https://www.rfc-editor.org/rfc/rfc9110.html#section-3.9
///
///```http
///request  (required)  | <METHOD> <URL>
///```
/// OR
///```http
///request  (required)  | <METHOD> <URL>
///headers  (optional)  | <HEADER_NAME>: <HEADER_VALUE>
///                     | ...
///                     | <HEADER_NAME>: <HEADER_VALUE>
///newline  (required)  |
///body     (optional)  | <BODY>
///```
pub fn parse_from_utf8<T: AsRef<[u8]>>(
    http_file_buffer: T,
) -> Result<RequestBuilder, HttpLexerError> {
    let contents = String::from_utf8_lossy(http_file_buffer.as_ref());

    // eventually create HttpTokens enum to do something like the following
    // lines.next()
    // if index == 0, then HttpToken::Request(..)
    // if index > 0, then HttpToken::Header(..)
    // until line == "\n", then HttpToken::Body(..)
    let builder = parser::request(&contents)?;

    Ok(builder)
}
