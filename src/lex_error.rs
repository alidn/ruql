use crate::cursor::Location;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct LexError {
    repr: ErrorKind,
    location: Location,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Lex Error")
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

impl std::error::Error for LexError {}

#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    InvalidToken,
}
