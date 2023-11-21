use std::ops::{Add, Sub};

use crate::chess_engine::{
    errors::{ExpectedOneOf, ParsingError},
    Error,
};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum File {
    H,
    G,
    F,
    E,
    D,
    C,
    B,
    A,
}
impl TryFrom<char> for File {
    type Error = ParsingError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            'h' => Self::H,
            err => Err(ParsingError::NotAFile(ExpectedOneOf::new(
                vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
                Some(value as char),
                None,
            )))?,
        })
    }
}
// can stil implement from_str and just ensure it is one long buut not needed
impl TryFrom<u8> for File {
    type Error = ParsingError; // porob should have custom error here
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            5 => Self::E,
            6 => Self::F,
            7 => Self::G,
            8 => Self::H,
            err => Err(ParsingError::NotAFile(ExpectedOneOf::new(
                vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
                Some(value as char),
                None,
            )))?,
        })
    }
}
impl Into<u8> for &File {
    //only implementing into for it beacuse we need checks when from u8 -> File
    fn into(self) -> u8 {
        match self {
            File::A => 1,
            File::B => 2,
            File::C => 3,
            File::D => 4,
            File::E => 5,
            File::F => 6,
            File::G => 7,
            File::H => 8,
        }
    }
}
impl Into<u8> for File {
    //only implementing into for it beacuse we need checks when from u8 -> File
    fn into(self) -> u8 {
        (&self).into()
    }
}
impl Add for File {
    // maybe do AddAssign to so you can use both + and +=
    type Output = Result<File, ParsingError>;
    fn add(self, rhs: Self) -> Self::Output {
        let lhsv: u8 = self.into();
        let rhsv: u8 = rhs.into();
        (lhsv + rhsv).try_into()
    }
}
impl Sub for File {
    type Output = Result<File, ParsingError>;
    fn sub(self, rhs: Self) -> Self::Output {
        let lhsv: u8 = self.into();
        let rhsv: u8 = rhs.into();
        (lhsv - rhsv).try_into()
    }
}
impl Add<u8> for File {
    type Output = Result<File, ParsingError>;
    fn add(self, rhs: u8) -> Self::Output {
        let lhsv: u8 = self.into();
        (lhsv + rhs).try_into()
    }
}
impl Sub<u8> for File {
    type Output = Result<File, ParsingError>;
    fn sub(self, rhs: u8) -> Self::Output {
        let lhsv: u8 = self.into();
        (lhsv - rhs).try_into()
    }
}
