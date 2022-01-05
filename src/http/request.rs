use super::{Method, MethodError, QueryString};
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{from_utf8, Utf8Error};

#[derive(Debug)]
pub struct Request<'buf> {
    pub method: Method,
    pub path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    protocol: &'buf str,
    headers: Option<&'buf str>,
    body: Option<&'buf str>,
}

impl<'buf> Request<'buf> {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &'buf str {
        self.path
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

    pub fn protocol(&self) -> &'buf str {
        self.protocol
    }

    pub fn headers(&self) -> Option<&'buf str> {
        self.headers
    }

    pub fn body(&self) -> Option<&'buf str> {
        self.body
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let req = from_utf8(buf)?;

        let (method, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (path, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;

        let method: Method = method.parse()?;

        let (query_string, path) = parse_query_string(path);

        Ok(Self {
            path,
            query_string,
            method,
            protocol,
            // TODO: parse headers and body
            headers: None,
            body: None,
        })
    }
}

fn get_next_word(req: &str) -> Option<(&str, &str)> {
    for (i, c) in req.chars().enumerate() {
        if c == ' ' || c == '\r' || c == '\n' {
            return Some((&req[..i], &req[i + 1..]));
        }
    }

    None
}

fn parse_query_string(mut path: &str) -> (Option<QueryString>, &str) {
    let mut query_string = None;

    if let Some(i) = path.find('?') {
        query_string = Some(QueryString::from(&path[i + 1..]));
        path = &path[..i];
    }

    (query_string, path)
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
