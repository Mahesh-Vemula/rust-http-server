use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone)]
pub enum StatusCode{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404
}

impl StatusCode {
    pub fn reason_pharse(&self) -> &str{
        match self{
            StatusCode::Ok => "Ok",
            StatusCode::BadRequest => "BadRequest",
            StatusCode::NotFound => "NotFound"
        }
    }
}

impl Display for StatusCode{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, {}, *self as u16)
    }
}