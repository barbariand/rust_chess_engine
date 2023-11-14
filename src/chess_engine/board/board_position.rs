use std::{ops::Add, str::FromStr};

use crate::chess_engine::{errors::BoardPositionError, Error};

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
        self.rank as u64 * 8 as u64 + self.file as u64
    }
}
impl Into<u64> for &BoardPosition {
    fn into(self) -> u64 {
        self.rank as u64 * 8 as u64 + self.file as u64
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        if s.len() > 2 {
            Err(BoardPositionError::CharOverflow)?;
        }
        if s.len() < 2 {
            Err(BoardPositionError::CharUnderflow)?; // ;)
        }
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
