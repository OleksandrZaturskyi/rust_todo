use super::StatusCode;
use std::io::{Result, Write};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    headers: Option<String>,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            body,
            headers: None,
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> Result<()> {
        let headers = match &self.headers {
            Some(h) => h,
            None => "",
        };

        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}\r\n",
            self.status_code,
            self.status_code.reason_phrase(),
            headers,
            body,
        )
    }
}
