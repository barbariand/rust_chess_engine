use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}
impl Rank {
    pub fn from_char(c: char) -> Result<Rank, String> {
        Ok(match c {
            '1' => Rank::One,
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            err => Err(format!("Could not parse {} to a rank", err))?,
        })
    }
}
impl Into<i8> for Rank {
    //only implementing into for it beacuse we need checks when from i8 -> File
    fn into(self) -> i8 {
        match self {
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
impl Into<i8> for &Rank {
    //only implementing into for it beacuse we need checks when from i8 -> File
    fn into(self) -> i8 {
        match self {
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
impl TryFrom<i8> for Rank {
    type Error = String; // porob should have custom error here
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            err => Err(format!("{} is not a valid file", err))?,
        })
    }
}
impl Add for Rank {
    // maybe do AddAssign to so you can use both + and +=
    type Output = Result<Rank, String>;
    fn add(self, rhs: Self) -> Self::Output {
        let lhsv: i8 = self.into();
        let rhsv: i8 = rhs.into();
        (lhsv + rhsv).try_into()
    }
}
impl Sub for Rank {
    type Output = Result<Rank, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        let lhsv: i8 = self.into();
        let rhsv: i8 = rhs.into();
        (lhsv - rhsv).try_into()
    }
}
impl Add<i8> for Rank {
    type Output = Result<Rank, String>;
    fn add(self, rhs: i8) -> Self::Output {
        let lhsv: i8 = self.into();
        (lhsv + rhs).try_into()
    }
}
impl Sub<i8> for Rank {
    type Output = Result<Rank, String>;
    fn sub(self, rhs: i8) -> Self::Output {
        let lhsv: i8 = self.into();
        (lhsv - rhs).try_into()
    }
}
