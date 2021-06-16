use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum Method{
    GET,
    POST,
    PUT,
    DELETE
}

impl FromStr for Method{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            _ => Err(String::from("error")),
        }
    }
}

impl Display for Method{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self)
    }
}

pub struct MethodError{

}