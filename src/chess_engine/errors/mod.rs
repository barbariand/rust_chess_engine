use std::{error::Error as StdError, fmt::Display};

#[derive(Debug)]
pub enum Error {
    Parsning(ParsingError),
    BoardPosition(BoardPositionError),
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
    UnbalancedBraces,
    Uninteligable(String),
}
impl From<ParsingError> for Error {
    fn from(value: ParsingError) -> Self {
        Error::Parsning(value)
    }
}
impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("")
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
impl Display for BoardPositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
