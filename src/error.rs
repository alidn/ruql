use crate::lexer::Location;
use std::fmt;

pub struct Error {
    repr: ErrorKind,
    location: Location,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ERR!")
            .field(&self.repr)
            .field(&self.location)
            .finish()
    }
}

impl Error {
    pub fn new(kind: ErrorKind, loc: Location) -> Error {
        Error {
            repr: kind,
            location: loc,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidToken,
}
