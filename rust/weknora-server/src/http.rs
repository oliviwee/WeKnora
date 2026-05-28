use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Other(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpResponse {
    pub status: u16,
    pub reason: &'static str,
    pub body: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseRequestError(String);

impl fmt::Display for ParseRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ParseRequestError {}

impl HttpRequest {
    pub fn parse(bytes: &[u8]) -> Result<Self, ParseRequestError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|err| ParseRequestError(format!("request is not valid UTF-8: {err}")))?;
        let line = text
            .lines()
            .next()
            .ok_or_else(|| ParseRequestError("missing request line".into()))?;
        let mut parts = line.split_whitespace();
        let method = parts
            .next()
            .ok_or_else(|| ParseRequestError("missing request method".into()))?;
        let target = parts
            .next()
            .ok_or_else(|| ParseRequestError("missing request target".into()))?;

        Ok(Self {
            method: Method::from(method),
            path: target.split('?').next().unwrap_or(target).to_string(),
        })
    }
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "PATCH" => Self::Patch,
            other => Self::Other(other.to_string()),
        }
    }
}

impl HttpResponse {
    pub fn json(status: u16, reason: &'static str, body: String) -> Self {
        Self {
            status,
            reason,
            body,
        }
    }

    pub fn bad_request(message: &str) -> Self {
        Self::json(
            400,
            "Bad Request",
            format!(
                r#"{{"error":{{"code":"input.invalid_request","message":"{}"}}}}"#,
                escape_json(message)
            ),
        )
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        format!(
            "HTTP/1.1 {} {}\r\ncontent-type: application/json; charset=utf-8\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
            self.status,
            self.reason,
            self.body.len(),
            self.body
        )
        .into_bytes()
    }
}

pub fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_request_ignores_query_string() {
        let request = HttpRequest::parse(b"GET /healthz?verbose=true HTTP/1.1\r\n\r\n").unwrap();
        assert_eq!(request.method, Method::Get);
        assert_eq!(request.path, "/healthz");
    }
}
