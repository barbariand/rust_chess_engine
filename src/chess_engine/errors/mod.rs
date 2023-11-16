use std::{error::Error as StdError, fmt::Display};

#[derive(Debug)]
pub enum Error {
    Parsning(ParsingError),
    BoardPosition(BoardPositionError),
    Board(BoardError),
}
#[derive(Debug)]
pub enum ParsingError {
    UnbalancedBraces,
    Uninteligable(String),
}
impl From<ParsingError> for Error {
    fn from(value: ParsingError) -> Self {
        Error::Parsning(value)
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
#[derive(Debug)]
pub enum BoardPositionError {
    CharOverflow,
    CharUnderflow,
    NotAFile(String),
    NotARank(String),
}

impl From<BoardPositionError> for Error {
    fn from(value: BoardPositionError) -> Self {
        Error::BoardPosition(value)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("do a better error implementaton")
    }
}
impl StdError for Error {}
