use super::method::{Method, MethodError};
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{from_utf8, Utf8Error};

pub struct Request<'buf> {
    method: Method,
    path: &'buf str,
    query_string: Option<&'buf str>,
    protocol: &'buf str,
    headers: Option<&'buf str>,
    body: Option<&'buf str>,
}

// *** HTTP request example ***
// POST /path?query=q&string=s HTTP/1.1
// Host: localhost:8080
// User-Agent: curl/7.74.0
// Accept: */*
// Content-Length: 4
// Content-Type: application/x-www-form-urlencoded

// body

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let req = from_utf8(buf)?;

        let (method, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (path, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

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
        if c == ' ' || c == '\r' {
            return Some((&req[..i], &req[i + 1..]));
        }
    }

    None
}

fn parse_query_string(mut path: &str) -> (Option<&str>, &str) {
    let mut query_string = None;

    if let Some(i) = path.find('?') {
        query_string = Some(&path[i + 1..]);
        path = &path[..i];
    }

    (query_string, path)
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
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
