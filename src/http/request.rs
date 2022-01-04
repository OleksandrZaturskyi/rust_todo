use super::Method;
use std::convert::TryFrom;

pub struct Request {
    method: String,
    path: Option<String>,
    query_string: String,
    protocol: Method,
    body: Option<String>,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
