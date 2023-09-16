use std::{error::Error as StdError, fmt::Display};

#[derive(Debug)]
pub enum Error{
    Action(ActionError),
    BoardPosition(BoardPositionError)
}
#[derive(Debug)]
pub enum ActionError{
    PieceNotInPlay,
    SameColor
}
#[derive(Debug)]
pub enum BoardPositionError{
    CharOverflow,
    CharUnderflow,
    NotAFile(String),
    NotARank(String),
}
impl From<ActionError> for Error{
    fn from(value: ActionError) -> Self {
        Error::Action(value)
    }
}
impl From<BoardPositionError> for Error{
    fn from(value: BoardPositionError) -> Self {
        Error::BoardPosition(value)
    }
}
impl Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"fuck you")
    }
}
impl StdError for Error{}