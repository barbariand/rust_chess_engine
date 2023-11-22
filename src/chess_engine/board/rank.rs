use std::ops::{Add, Sub};

use crate::chess_engine::{
    errors::{ExpectedOneOf, ParsingError},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Rank {
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}
impl TryFrom<char> for Rank {
    type Error = ParsingError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let rank = value.to_string().parse::<u8>().map_err(|_| {
            ParsingError::NotAFile(ExpectedOneOf::new(
                vec!['1', '2', '3', '4', '5', '6', '7', '8'],
                Some(value),
                None,
            ))
        })?;
        Self::try_from(rank)
    }
}
impl From<&Rank> for u8 {
    //only implementing into for it beacuse we need checks when from u8 -> File
    fn from(val: &Rank) -> Self {
        match val {
            Rank::One => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
        }
    }
}
impl From<Rank> for u8 {
    //only implementing into for it beacuse we need checks when from u8 -> File
    fn from(val: Rank) -> Self {
        (&val).into()
    }
}
impl TryFrom<u8> for Rank {
    type Error = ParsingError; // porob should have custom error here
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            _err => Err(ParsingError::NotARank(ExpectedOneOf::new(
                vec!['1', '2', '3', '4', '5', '6', '7', '8'],
                Some(value as char),
                None,
            )))?,
        })
    }
}
impl Add for Rank {
    // maybe do AddAssign to so you can use both + and +=
    type Output = Result<Rank, ParsingError>;
    fn add(self, rhs: Self) -> Self::Output {
        let lhsv: u8 = self.into();
        let rhsv: u8 = rhs.into();
        (lhsv + rhsv).try_into()
    }
}
impl Sub for Rank {
    type Output = Result<Rank, ParsingError>;
    fn sub(self, rhs: Self) -> Self::Output {
        let lhsv: u8 = self.into();
        let rhsv: u8 = rhs.into();
        (lhsv - rhsv).try_into()
    }
}
impl Add<u8> for Rank {
    type Output = Result<Rank, ParsingError>;
    fn add(self, rhs: u8) -> Self::Output {
        let lhsv: u8 = self.into();
        (lhsv + rhs).try_into()
    }
}
impl Sub<u8> for Rank {
    type Output = Result<Rank, ParsingError>;
    fn sub(self, rhs: u8) -> Self::Output {
        let lhsv: u8 = self.into();
        (lhsv - rhs).try_into()
    }
}
