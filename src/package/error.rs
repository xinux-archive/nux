use std::fmt::{Display, Formatter};
use std::io::{Error as IoError, ErrorKind};


#[derive(Debug)]
pub enum NuxError {
    CustomError(ErrorKind),
    SpecificError(String)
}

//To provide a user-friendly error message
impl Display for NuxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NuxError::CustomError(ErrorKind::InvalidInput) => {
                write!(f, "Oops, this is an invalid input:(")
            }
            NuxError::SpecificError(err) => write!(f, "Oops: {}", err),
            _ => write!(f, "Oops, something went wrong :("),
        }
    }
}
//To allow for easy conversion from std::io::Error to our custom error type
impl From<IoError> for NuxError {
    fn from(err: IoError) -> NuxError {
        NuxError::CustomError(err.kind())
    }
}