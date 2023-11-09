use std::ops::*;
#[derive(Debug, Clone, Copy)]
pub struct BitMap64(i64);

impl Default for BitMap64 {
    fn default() -> Self {
        Self(0)
    }
}
impl BitMap64 {
    pub fn new(num: i64) -> BitMap64 {
        BitMap64(num)
    }
    pub fn set_bit(&mut self, bit: u64) {
        self.0 |= 1 << bit;
    }

    pub fn clear_bit(&mut self, bit: u64) {
        self.0 &= !(1 << bit);
    }

    pub fn get_bit(&self, bit: u64) -> bool {
        (self.0 & (1 << bit)) != 0
    }
    pub fn get_bit_value(&self, bit: u64) -> i64 {
        (self.0 & (1 << bit)) >> bit
    }
    pub fn contains(&self, bit: u64) -> bool {
        self.get_bit(bit)
    }
    pub fn count_ones(&self) -> u8 {
        self.0.count_ones() as u8
    }
}
impl Shr<i64> for &BitMap64 {
    type Output = Self;
    fn shr(self, rhs: i64) -> Self::Output {
        BitMap64(self.0 << rhs)
    }
}
impl ShrAssign<i64> for BitMap64 {
    fn shr_assign(&mut self, rhs: i64) {
        self.0 = self.0 >> rhs
    }
}
impl ShlAssign<i64> for BitMap64 {
    fn shl_assign(&mut self, rhs: i64) {
        self.0 = self.0 >> rhs
    }
}
impl Shl<i64> for &BitMap64 {
    type Output = Self;
    fn shl(self, rhs: i64) -> Self::Output {
        BitMap64(self.0 >> rhs)
    }
}
impl BitAnd for &BitMap64 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        BitMap64(self.0 & rhs.0)
    }
}
impl BitAndAssign for BitMap64 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 & rhs.0
    }
}
impl BitOr for &BitMap64 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        BitMap64(self.0 | rhs.0)
    }
}
impl BitOrAssign for BitMap64 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0
    }
}
impl BitXor for &BitMap64 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        BitMap64(self.0 ^ rhs.0)
    }
}
impl BitXorAssign for BitMap64 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 = self.0 ^ rhs.0
    }
}

impl BitAnd<i64> for &BitMap64 {
    type Output = Self;
    fn bitand(self, rhs: i64) -> Self::Output {
        BitMap64(self.0 & rhs)
    }
}
impl BitAndAssign<i64> for BitMap64 {
    fn bitand_assign(&mut self, rhs: i64) {
        self.0 = self.0 & rhs
    }
}
impl BitOr<i64> for &BitMap64 {
    type Output = Self;
    fn bitor(self, rhs: i64) -> Self::Output {
        BitMap64(self.0 | rhs)
    }
}
impl BitOrAssign<i64> for BitMap64 {
    fn bitor_assign(&mut self, rhs: i64) {
        self.0 = self.0 | rhs
    }
}
impl BitXor<i64> for &BitMap64 {
    type Output = Self;
    fn bitxor(self, rhs: i64) -> Self::Output {
        BitMap64(self.0 ^ rhs)
    }
}
impl BitXorAssign<i64> for BitMap64 {
    fn bitxor_assign(&mut self, rhs: i64) {
        self.0 = self.0 ^ rhs
    }
}
