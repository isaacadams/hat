use super::{HttpLexerError, RequestBuilder};

/// <METHOD> <URL>
/// METHOD = letters only
/// URL    = https://www.rfc-editor.org/rfc/rfc9110.html#section-4.1
pub fn parse_request_line(line: &str) -> Option<[&str; 2]> {
    let mut line = line.split(' ');
    let method = line.next()?;
    let url = line.next()?;

    if method.is_empty() || url.is_empty() {
        return None;
    }

    Some([method, url])
}

/// <HEADER>: <VALUES>
/// HEADER = letters + '-'
/// VALUES = <VALUE> ... <VALUE>
/// VALUE = alphanumerics
pub fn parse_header_line(line: &str) -> Option<[&str; 2]> {
    let mut line = line.split(": ");
    let header_name = line.next()?;
    let header_value = line.next()?;

    if header_name.is_empty() || header_value.is_empty() {
        return None;
    }

    Some([header_name, header_value])
}

pub fn request(contents: &str) -> Result<RequestBuilder, HttpLexerError> {
    let mut lines = contents.lines();
    let mut row = 1;
    let col = 1;

    let first_line = if let Some(x) = lines.next() {
        log::debug!("[{} _request]: {}", row, x);
        x
    } else {
        return Err(HttpLexerError::MalformedHttpFile(
            "request line is missing\nmust have at least one line with a <METHOD> <URL>",
        ));
    };

    let [method, url] =
        self::parse_request_line(first_line).ok_or(HttpLexerError::MalformedLine {
            row,
            col,
            content: first_line.to_string(),
            reason: std::borrow::Cow::Borrowed(
                "a method and a url path is expected\n<METHOD> <URL>",
            ),
        })?;

    let mut builder = RequestBuilder::new(method, url)?;

    // parse headers
    let mut current = lines.next();
    while let Some(line) = current {
        row += 1;

        if line.is_empty() {
            log::debug!("[{} _newline]: {}", row, line);
            break;
        }

        log::debug!("[{} __header]: {}", row, line);

        let [header_name, header_value] =
            self::parse_header_line(line).ok_or(HttpLexerError::MalformedLine {
                row,
                col,
                content: line.to_string(),
                reason: std::borrow::Cow::Borrowed(
                    r#"
does not conform to header formatting rule(s)
<NAME>: <VALUE>"#,
                ),
            })?;

        builder = builder.add_header(header_name, header_value);
        current = lines.next();
    }

    // eventually make this configurable?
    // likely always want to load the unaltered request body

    // should use .remainder() here, but its not stabilized
    // https://doc.rust-lang.org/std/str/struct.Split.html#method.remainder
    let preserve_newlines = true;
    let remaining: String = if preserve_newlines {
        lines.fold(String::new(), |mut p, c| {
            p.push_str(c);
            p.push('\n');
            p
        })
    } else {
        lines.collect()
    };

    if !remaining.is_empty() {
        let mut remaining = remaining;
        // remove extra '\n'
        remaining.pop();
        log::debug!("[{}+ ___body]: {}", row + 1, &remaining);
        builder.add_body(remaining);
    }

    Ok(builder)
}

#[cfg(test)]
mod test {
    use super::super::parse_from_utf8;
    use super::*;

    #[test]
    pub fn http_single_line() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1"#;
        let result = self::parse_from_utf8(http);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.has_headers());
        assert!(!result.has_body());
        assert_eq!(result.get_method(), "GET");
        assert_eq!(
            result.get_url(),
            "https://jsonplaceholder.typicode.com/todos/1"
        );
    }

    #[test]
    pub fn http_with_single_header() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
Content-Type: application/json
"#;
        let result = self::parse_from_utf8(http);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get_method(), "GET");
        assert_eq!(
            result.get_url(),
            "https://jsonplaceholder.typicode.com/todos/1"
        );
        assert!(result.has_headers());
        assert_eq!(result.get_header("Content-Type"), Some("application/json"));
    }

    #[test]
    pub fn http_with_many_headers() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
Content-Type: application/json
Accept: application/json
"#;
        let result = self::parse_from_utf8(http);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get_method(), "GET");
        assert_eq!(
            result.get_url(),
            "https://jsonplaceholder.typicode.com/todos/1"
        );
        assert!(result.has_headers());
        assert_eq!(result.get_header("Content-Type"), Some("application/json"));
        assert_eq!(result.get_header("Accept"), Some("application/json"));
    }

    #[test]
    pub fn http_with_many_headers_and_body() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
Content-Type: application/json
Accept: application/json

{
    "message": "hello world",
    "count": 10
}"#;
        let result = self::parse_from_utf8(http);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get_method(), "GET");
        assert_eq!(
            result.get_url(),
            "https://jsonplaceholder.typicode.com/todos/1"
        );
        assert!(result.has_headers());
        assert!(result.has_body());
        assert_eq!(result.get_header("Content-Type"), Some("application/json"));
        assert_eq!(result.get_header("Accept"), Some("application/json"));
        assert_eq!(
            result.into_body(),
            Some(String::from(
                r#"{
    "message": "hello world",
    "count": 10
}"#
            ))
        );
    }

    #[test]
    pub fn http_with_body_only() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1

{
    "message": "hello world",
    "count": 10
}"#;
        let result = self::parse_from_utf8(http);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get_method(), "GET");
        assert_eq!(
            result.get_url(),
            "https://jsonplaceholder.typicode.com/todos/1"
        );

        assert!(result.has_body());
        assert_eq!(
            result.into_body(),
            Some(String::from(
                r#"{
    "message": "hello world",
    "count": 10
}"#
            ))
        );
    }

    #[test]
    pub fn http_with_headers_and_many_header_values() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
Content-Type: application/json
Accept-Language: en, mi
"#;
        let result = self::parse_from_utf8(http);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get_method(), "GET");
        assert_eq!(
            result.get_url(),
            "https://jsonplaceholder.typicode.com/todos/1"
        );
        assert!(result.has_headers());
        assert_eq!(result.get_header("Content-Type"), Some("application/json"));
        assert_eq!(result.get_header("Accept-Language"), Some("en, mi"));
    }

    /*     #[test]
        pub fn throws_error_for_invalid_header() {
            let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
    Content-Type: application/json"#;
            assert!(matches!(
                self::parse_from_utf8(http),
                Err(HttpLexerError::MalformedLine {
                    row: 1,
                    col: 1,
                    content: _,
                    reason: _
                })
            ));
        } */

    #[test]
    pub fn http_multiline_errors_when_last_line_is_not_only_newline() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
Content-Type: application/json
     "#;
        let result = self::parse_from_utf8(http);
        //println!("{:#?}", result);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(HttpLexerError::MalformedLine {
                row: 3,
                col: 1,
                content: _,
                reason: _
            })
        ));
    }

    /*     #[test]
        pub fn http_multiline_errors_when_a_line_is_empty() {

            let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
    Content-Type: application/json

    Accept-Language: en, mi
    asdsads
    asldaslkdm
    alkaslkdas
    asmasldmslkm
    "#;
            let result = self::parse_from_utf8(http);
            println!("{:#?}", result);
            assert!(result.is_err());
            assert!(matches!(
                result,
                Err(HttpLexerError::MalformedLine {
                    row: 3,
                    col: 1,
                    content: _,
                    reason: _
                })
            ));
        } */

    #[test]
    pub fn http_multiline_errors_when_header_name_is_missing_colon() {
        let http = r#"GET https://jsonplaceholder.typicode.com/todos/1
Content-Type application/json
Accept-Language: en, mi
"#;
        assert!(matches!(
            self::parse_from_utf8(http),
            Err(HttpLexerError::MalformedLine {
                row: 2,
                col: 1,
                content: _,
                reason: _
            })
        ));
    }

    #[test]
    pub fn http_errors_when_empty() {
        let http = r#""#;
        assert!(matches!(
            self::parse_from_utf8(http),
            Err(HttpLexerError::MalformedHttpFile(_))
        ));
    }

    #[test]
    pub fn throws_error_on_missing_url() {
        let http = r#"GET "#;
        let result = self::parse_from_utf8(http);
        println!("{:#?}", result);
        assert!(matches!(
            result,
            Err(HttpLexerError::MalformedLine {
                row: 1,
                col: _,
                content: _,
                reason: _,
            })
        ));
    }
}
