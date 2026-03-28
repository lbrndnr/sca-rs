use std::str::FromStr;

const CRLF: &[u8] = "\r\n".as_bytes();

#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    Custom(String),
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" | "get" => Method::GET,
            "HEAD" | "head" => Method::HEAD,
            "POST" | "post" => Method::POST,
            "PUT" | "put" => Method::PUT,
            "DELETE" | "delete" => Method::DELETE,
            "CONNECT" | "connect" => Method::CONNECT,
            "OPTIONS" | "options" => Method::OPTIONS,
            "TRACE" | "trace" => Method::TRACE,
            "PATCH" | "patch" => Method::PATCH,
            _ => Method::Custom(s.to_string()),
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Custom(s) => s.clone(),
            _ => format!("{:?}", self),
        }
    }
}

pub struct HeaderField<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

pub struct Http1<'a> {
    pub method: Method,
    pub path: &'a str,
    pub fields: Vec<HeaderField<'a>>,
}

impl<'a> Http1<'a> {
    pub fn new(method: Method, path: &'a str, fields: Vec<HeaderField<'a>>) -> Self {
        Self {
            method,
            path,
            fields,
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Http1<'a> {
    type Error = String;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        let status_line_end = buf
            .windows(2)
            .position(|w| w == CRLF)
            .ok_or_else(|| "CRLF not found".to_string())?;

        let status_line_comps: Vec<&str> = buf[..status_line_end]
            .split(|b| *b == b' ')
            .map(|b| std::str::from_utf8(b))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        // .unwrap_or_else(|| "Invalid UTF8 in status line".to_string())?;

        if status_line_comps.len() != 3 {
            return Err("Invalid status line".to_string());
        }

        if !status_line_comps[2]
            .trim_ascii()
            .eq_ignore_ascii_case("http/1.1")
        {
            return Err("Invalid HTTP version".to_string());
        }

        let method = Method::from(status_line_comps[0].trim_ascii());
        let path = status_line_comps[1].trim_ascii();

        let prev_line_end = status_line_end;
        let mut fields = Vec::new();
        while let Some(next_line_end) = buf[prev_line_end..].windows(2).position(|w| w == CRLF) {
            let field_end = prev_line_end + next_line_end;
            let mut field = buf[prev_line_end..field_end].split(|b| *b == b':');
            let Some(Ok(key)) = field.next().map(|b| std::str::from_utf8(b)) else {
                return Err("Invalid header field".to_string());
            };
            let Some(Ok(value)) = field.next().map(|b| std::str::from_utf8(b)) else {
                return Err("Invalid header field".to_string());
            };
            if field.next().is_some() {
                return Err("Invalid header field".to_string());
            };

            fields.push(HeaderField { key, value });
        }

        Ok(Http1 {
            method,
            path,
            fields,
        })
    }
}

impl<'a> From<Http1<'a>> for Vec<u8> {
    fn from(value: Http1<'a>) -> Vec<u8> {
        let status_line = format!("{} {} HTTP/1.1\r\n", value.method.to_string(), value.path);
        let mut buf = Vec::new();
        buf.extend_from_slice(status_line.as_bytes());
        for field in &value.fields {
            buf.extend_from_slice(format!("{}: {}\r\n", field.key, field.value).as_bytes());
        }
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_line_into_bytes() {
        let http1 = Http1 {
            method: Method::GET,
            path: "/",
            fields: Vec::new(),
        };
        let bytes = Vec::from(http1);
        assert_eq!(bytes, b"GET / HTTP/1.1\r\n\r\n");
    }
}
