use std::{ops::Add, str::FromStr};

use crate::chess_engine::{errors::ParsingError, Error};

use super::{File, MoveOffset, Rank};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoardPosition {
    pub file: File,
    pub rank: Rank,
}

impl BoardPosition {
    pub fn new(file: File, rank: Rank) -> Self {
        BoardPosition { file, rank }
    }
    pub fn to_num(&self) -> u64 {
        self.rank as u64 * 8_u64 + self.file as u64
    }
}
impl From<&BoardPosition> for u64 {
    fn from(val: &BoardPosition) -> Self {
        val.rank as u64 * 8_u64 + val.file as u64
    }
}
impl Add<MoveOffset> for BoardPosition {
    type Output = Result<BoardPosition, Error>;
    fn add(self, rhs: MoveOffset) -> Self::Output {
        let rank = (self.rank + rhs.0)?;
        let file = (self.file + rhs.1)?;
        Ok(BoardPosition { file, rank })
    }
}
impl FromStr for BoardPosition {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("hello2????");
        let mut chars = s.chars();
        if s.len() > 2 {
            Err(ParsingError::CharOverflow)?;
        }
        if s.len() < 2 {
            Err(ParsingError::CharUnderflow)?; // ;)
        }
        println!("hello3????");
        Ok(BoardPosition {
            file: File::try_from(
                chars
                    .next()
                    .unwrap_or_else(|| unreachable!("we already checked for this above")),
            )?,
            rank: Rank::try_from(
                chars
                    .next()
                    .unwrap_or_else(|| unreachable!("we already checked for this above")),
            )?,
        })
    }
}
impl Add<MoveOffset> for &BoardPosition {
    type Output = Result<BoardPosition, Error>;
    fn add(self, rhs: MoveOffset) -> Self::Output {
        let rank = (self.rank + rhs.0)?;
        let file = (self.file + rhs.1)?;
        Ok(BoardPosition { file, rank })
    }
}
