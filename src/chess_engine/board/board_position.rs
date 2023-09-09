use std::{ops::Add, str::FromStr};

use super::{File, MoveOffset, Rank};
#[derive(Clone)]
pub struct BoardPosition {
    pub file: File,
    pub rank: Rank,
}

impl BoardPosition {
    fn new(file: File, rank: Rank) -> Self {
        BoardPosition { file, rank }
    }
    fn move_to_file(&mut self, file: File) {
        self.file = file;
    }
    fn move_to_rank(&mut self, rank: Rank) {
        self.rank = rank;
    }
}
impl Add<MoveOffset> for BoardPosition {
    type Output = Result<BoardPosition, String>;
    fn add(self, rhs: MoveOffset) -> Self::Output {
        let rank = (self.rank + rhs.0)?;
        let file = (self.file + rhs.1)?;
        Ok(BoardPosition { file, rank })
    }
}
impl FromStr for BoardPosition {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        if s.len() > 2 {
            Err(format!("to many chars"))?;
        }
        if s.len() < 2 {
            Err(format!("to few chars"))?;
        }
        Ok(BoardPosition {
            file: File::from_char(chars.next().expect("we already checked for this above"))?,
            rank: Rank::from_char(chars.next().expect("we already checked for this above"))?,
        })
    }
}
impl Add<MoveOffset> for &BoardPosition {
    type Output = Result<BoardPosition, String>;
    fn add(self, rhs: MoveOffset) -> Self::Output {
        let rank = (self.rank + rhs.0)?;
        let file = (self.file + rhs.1)?;
        Ok(BoardPosition { file, rank })
    }
}
