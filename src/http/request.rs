use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str;
use std::str::Utf8Error;

pub struct Request{
    path: String,
    query_string: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request{
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidReqeust)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidReqeust)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidReqeust)?;

        if protocol != "HTTP/1.1"{
           return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;


    }
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for Request{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Request method: {}, \n Request path: {}", self.method, self.path);
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)>{
    for(i,c) in request.chars().enumerate(){
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]))
        }
    }
    None
}

pub enum ParseError{
    InvalidReqeust,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError{
    fn message(&self) -> &str{
        match self {
            Self::InvalidReqeust => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message());
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message());
    }
}

impl Error for ParseError{

}