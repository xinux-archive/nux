use std::fmt::{Display, Formatter};
use std::io::{Error as IoError};
use serde::Serialize;


#[derive(Debug, PartialEq, Serialize)]
pub enum NuxError {
    SpecificError(String)
}

//To provide a user-friendly error message
impl Display for NuxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NuxError::SpecificError(err) => write!(f, "Oops: {}", err),
            _ => write!(f, "Oops, something went wrong :("),
        }
    }
}
