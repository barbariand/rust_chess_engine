use std::{error::Error as StdError, fmt::Display};

#[derive(Debug)]
pub enum Error {
    Parsning(ParsingError),
    Board(BoardError),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl StdError for Error {}
#[derive(Debug)]
pub enum ParsingError {
    UnbalancedBraces(ExpectedOneOf),
    ExpectedOneOf(ExpectedOneOf),
    CharOverflow,
    CharUnderflow,
    NotAFile(ExpectedOneOf),
    NotARank(ExpectedOneOf),
}
impl From<ParsingError> for Error {
    fn from(value: ParsingError) -> Self {
        Error::Parsning(value)
    }
}
impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::UnbalancedBraces(e) => write!(f, "Brace missmatch {}", e),
            ParsingError::ExpectedOneOf(e) => write!(f, "Expectation failed {}", e),
            ParsingError::CharOverflow => write!(f, "Missing chars"),
            ParsingError::CharUnderflow => write!(f, "To many chars"),
            ParsingError::NotAFile(e) => write!(f, "Unable to parse file {}", e),
            ParsingError::NotARank(e) => write!(f, "Unable to parse rank {}", e),
        }
    }
}
#[derive(Debug)]
pub struct ExpectedOneOf {
    excpected: Vec<char>,
    got: Option<char>,
    complete: Option<String>,
}
impl ExpectedOneOf {
    pub fn new(excpected: Vec<char>, got: Option<char>, complete: Option<String>) -> Self {
        Self {
            excpected,
            got,
            complete,
        }
    }
}
impl Display for ExpectedOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.got, self.complete.clone()) {
            (Some(got), Some(complete)) => write!(
                f,
                "expected one of {:?} got \"{}\", complete string \"{}\"",
                self.excpected, got, complete
            ),
            (None, Some(complete)) => write!(
                f,
                "expected one of {:?} got nothing, complete string \"{}\"",
                self.excpected, complete
            ),
            (Some(got), None) => write!(f, "expected one of {:?} got \"{}\"", self.excpected, got),
            (None, None) => write!(f, "expected one of {:?} got nogthing", self.excpected),
        }
    }
}
#[derive(Debug)]
pub enum BoardError {
    PieceMissing,
    PieceWhereMoving,
    PieceNotInPlay,
    SameColor,
}
impl From<BoardError> for Error {
    fn from(value: BoardError) -> Self {
        Error::Board(value)
    }
}
impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
