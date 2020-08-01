use crate::cursor::Location;
use std::fmt;

pub struct LexError {
    repr: ErrorKind,
    location: Location,
}

impl fmt::Debug for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ERR!")
            .field(&self.repr)
            .field(&self.location)
            .finish()
    }
}

impl LexError {
    pub fn new(kind: ErrorKind, loc: Location) -> LexError {
        LexError {
            repr: kind,
            location: loc,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidToken,
}
