use crate::hat_util::RequestBuilder;
use std::path::Path;

use super::HttpLexerError;

pub fn parse(input: &str) -> Result<RequestBuilder, HttpLexerError> {
    if input.contains(".http") {
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
/// request  (required)  | <METHOD> <URL>
/// headers  (optional)  | <HEADER_NAME> <HEADER_VALUE>
///                      | ...
///                      | <HEADER_NAME> <HEADER_VALUE>
///                      |
/// body     (optional)  | <BODY>
pub fn parse_from_utf8<T: AsRef<[u8]>>(
    http_file_buffer: T,
) -> Result<RequestBuilder, HttpLexerError> {
    let contents = String::from_utf8_lossy(http_file_buffer.as_ref());

    // instead of doing this, I could call .next() on everything
    // if index == 0, then HttpToken::Request(..)
    // if index > 0, then HttpToken::Header(..)
    // until line == "\n\n", then HttpToken::Body(..)
    let mut lines = contents.split("\n\n");

    let mut builder = if let Some(request) = lines.next() {
        parser::request(request)?
    } else {
        return Err(HttpLexerError::MalformedHttpFile(
            "request line is missing\nmust have at least one line with a <method> <ur>",
        ));
    };

    if let Some(body) = lines.next() {
        builder.add_body(body.to_string());
    }

    Ok(builder)
}

pub mod parser {
    use super::*;

    pub fn line(line: &str) -> Option<[&str; 2]> {
        let content = line.split(' ').collect::<Vec<&str>>();
        match content[..] {
            [first, second] => Some([first, second]),
            [..] => None,
        }
    }

    pub fn request(first_half_of_http_file: &str) -> Result<RequestBuilder, HttpLexerError> {
        let mut lines = first_half_of_http_file.lines();

        let first = lines.next();
        let first = if let Some(x) = first {
            x
        } else {
            return Err(HttpLexerError::MalformedHttpFile(
                "request line is missing\nmust have at least one line with a <method> <ur>",
            ));
        };

        let [method, url] = self::line(first).ok_or(HttpLexerError::MalformedLine {
            row: 0,
            col: 0,
            content: first.to_string(),
            reason: std::borrow::Cow::Borrowed(
                "a method and a url path is expected\n<METHOD> <URL>",
            ),
        })?;

        let mut builder = RequestBuilder::new(method, url)?;

        for (mut row, line) in lines.enumerate() {
            row += 1;
            let [header_name, header_value] =
                self::line(line).ok_or(HttpLexerError::MalformedLine {
                    row,
                    col: 0,
                    content: line.to_string(),
                    reason: std::borrow::Cow::Borrowed(
                        "a header name and value is expected\n<NAME> <VALUE>",
                    ),
                })?;

            builder.add_header(header_name, header_value).map_err(|e| {
                HttpLexerError::MalformedLine {
                    row,
                    col: 0,
                    content: line.to_string(),
                    reason: std::borrow::Cow::Owned(e.to_string()),
                }
            })?;
        }

        Ok(builder)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn should_receive_ok() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1"#;
        let result = self::parse_from_utf8(http);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.has_headers());
        assert!(!result.has_body());
    }

    #[test]
    pub fn should_receive_ok_with_single_header() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
Content-Type application/json"#;
        let result = self::parse_from_utf8(http);
        assert!(result.is_ok());
        assert!(result.unwrap().has_headers());
    }

    #[test]
    pub fn should_receive_ok_with_many_headers() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
Content-Type application/json
Accept application/json"#;
        let result = self::parse_from_utf8(http);
        assert!(result.is_ok());
        assert!(result.unwrap().has_headers());
    }

    #[test]
    pub fn should_receive_ok_with_many_headers_and_body() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
Content-Type application/json
Accept application/json

{
    "message": "hello world",
    "count": 10
}"#;
        let result = self::parse_from_utf8(http);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.has_headers());
        assert!(result.has_body());
        assert_eq!(
            result.to_body(),
            Some(String::from(
                r#"{
    "message": "hello world",
    "count": 10
}"#
            ))
        );
    }

    #[test]
    pub fn throws_error_for_invalid_header() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
Content-Type: application/json"#;
        assert!(matches!(
            self::parse_from_utf8(http),
            Err(HttpLexerError::MalformedLine {
                row: 1,
                col: 0,
                content: _,
                reason: _
            })
        ));
    }

    #[test]
    pub fn throws_error_for_malformed_line() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
Content-Type application/json
     "#;
        assert!(matches!(
            self::parse_from_utf8(http),
            Err(HttpLexerError::MalformedLine {
                row: 2,
                col: 0,
                content: _,
                reason: _
            })
        ));
    }

    #[test]
    pub fn throws_error_on_empty() {
        let http = r#""#;
        assert!(matches!(
            self::parse_from_utf8(http),
            Err(HttpLexerError::MalformedHttpFile(_))
        ));
    }

    /* #[test]
    pub fn throws_error_on_missing_url() {
        let http = r#"GET "#;
        let result = execute(http);
        assert!(matches!(
            result,
            Err(HttpLexerError::MalformedLine {
                col: _,
                content: _,
                expected: _,
                row: _
            })
        ));
    } */
}
