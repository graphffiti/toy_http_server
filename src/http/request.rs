use crate::http::request;

use super::method::Method;
use super::method::MethodError;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // A: 1st way:to do it
        // match str::from_utf8(value) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // B: 2nd way to do it, and most common
        // match str::from_utf8(value).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }

        // C: Shorcut of B, should have From<Utf8Error> implementations
        //let request = str::from_utf8(value).or(Err(ParseError::InvalidEncoding))?;

        // D: More idiomatic way, should have From<Utf8Error> to ParseError convertion implementations for it to convert the error type
        let request = str::from_utf8(value)?;

        //A.
        // match get_next_words(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }

        //B
        let (method, request) = get_next_words(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_words(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_words(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;

        //IF-LET
        //A.
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[(i + 1)..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }
        //
        //B.
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[(i + 1)..]);
        //     path = &path[..i];
        // }
        //
        //C.Idiomatic way
        if let Some(i) = path.find('?') {
            query_string = Some(&path[(i + 1)..]);
            path = &path[..i];
        }

        unimplemented!();
    }
}

fn get_next_words(request: &str) -> Option<(&str, &str)> {
    //A: 1st way to do it
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break,
    //     }
    // }

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[(i + 1)..]));
        }
    }
    None
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
