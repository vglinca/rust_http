use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str::Utf8Error;
use super::methods::MethodError;

pub enum ParseError {
    InvalidRequestError = 1,
    InvalidEncodingError,
    InvalidHttpVersionError,
    InvalidMethodError
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncodingError
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethodError
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self{
            Self::InvalidRequestError => "Invalid Request Error",
            Self::InvalidEncodingError => "Invalid Encoding Error",
            Self::InvalidHttpVersionError => "Invalid Http Version Error",
            Self::InvalidMethodError => "Invalid Method Error"
        }
    }
}

impl Error for ParseError {}