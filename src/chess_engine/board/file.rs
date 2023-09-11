use std::ops::{Add, Sub};

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
impl File {
    pub fn from_char(c: char) -> Result<File, String> {
        Ok(match c {
            'a' => File::A,
            'A' => File::A,
            'b' => File::B,
            'B' => File::B,
            'c' => File::C,
            'C' => File::C,
            'd' => File::D,
            'D' => File::D,
            'e' => File::E,
            'E' => File::E,
            'f' => File::F,
            'F' => File::F,
            'g' => File::G,
            'G' => File::G,
            'h' => File::H,
            'H' => File::H,
            err => Err(format!("Could not parse {} to a file", err))?,
        })
    }
}
// can stil implement from_str and just ensure it is one long buut not needed
impl TryFrom<i8> for File {
    type Error = String; // porob should have custom error here
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            5 => Self::E,
            6 => Self::F,
            7 => Self::G,
            8 => Self::H,
            err => Err(format!("{} is not a valid file", err))?,
        })
    }
}
impl Into<i8> for &File {
    //only implementing into for it beacuse we need checks when from i8 -> File
    fn into(self) -> i8 {
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
impl Into<i8> for File {
    //only implementing into for it beacuse we need checks when from i8 -> File
    fn into(self) -> i8 {
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
impl Add for File {
    // maybe do AddAssign to so you can use both + and +=
    type Output = Result<File, String>;
    fn add(self, rhs: Self) -> Self::Output {
        let lhsv: i8 = self.into();
        let rhsv: i8 = rhs.into();
        (lhsv + rhsv).try_into()
    }
}
impl Sub for File {
    type Output = Result<File, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        let lhsv: i8 = self.into();
        let rhsv: i8 = rhs.into();
        (lhsv - rhsv).try_into()
    }
}
impl Add<i8> for File {
    type Output = Result<File, String>;
    fn add(self, rhs: i8) -> Self::Output {
        let lhsv: i8 = self.into();
        (lhsv + rhs).try_into()
    }
}
impl Sub<i8> for File {
    type Output = Result<File, String>;
    fn sub(self, rhs: i8) -> Self::Output {
        let lhsv: i8 = self.into();
        (lhsv - rhs).try_into()
    }
}
