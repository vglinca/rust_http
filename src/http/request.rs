use super::methods::{HttpMethod, MethodError};
use super::errors::ParseError;
use std::convert::TryFrom;
use std::str::FromStr;
use std::str;

pub struct Request<'buf> {
    path: &'buf str,
    query: Option<&'buf str>,
    method: HttpMethod
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {

    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS
    fn try_from(buf: &'buf [u8]) -> std::result::Result<Self, Self::Error> {

        let result = str::from_utf8(buf).or(Err(ParseError::InvalidEncodingError))?;

        let (method, request) = get_next_word(result).ok_or(ParseError::InvalidRequestError)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequestError)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequestError)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidHttpVersionError);
        }

        let method: HttpMethod = method.parse()?;

        let mut query = None;
        

        if let Some(i) = path.find('?') {
            query = Some(&path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {path, query, method})
    }
}

fn get_next_word(req: &str) -> Option<(&str, &str)> {

    for (i, val) in req.chars().enumerate() {
        if val == ' ' || val == '\r' {
            return Some((&req[..i], &req[i+1..]))
        }
    }

    None
}