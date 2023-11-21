use std::ops::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitMap64(u64);

impl Default for BitMap64 {
    fn default() -> Self {
        Self(0)
    }
}

impl BitMap64 {
    pub const fn new(num: u64) -> BitMap64 {
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
    pub fn get_bit_value(&self, bit: u64) -> u64 {
        (self.0 & (1 << bit)) >> bit
    }
    pub fn contains(&self, bit: u64) -> bool {
        self.get_bit(bit)
    }
    pub fn count_ones(&self) -> u8 {
        self.0.count_ones() as u8
    }
    pub fn get_copied_inner(&self) -> u64 {
        self.0
    }
}
impl Shr<u64> for BitMap64 {
    type Output = Self;
    fn shr(self, rhs: u64) -> Self::Output {
        BitMap64(self.0 >> rhs)
    }
}
impl ShrAssign<u64> for BitMap64 {
    fn shr_assign(&mut self, rhs: u64) {
        self.0 = self.0 >> rhs
    }
}
impl ShlAssign<u64> for BitMap64 {
    fn shl_assign(&mut self, rhs: u64) {
        self.0 = self.0 << rhs
    }
}
impl Shl<u64> for BitMap64 {
    type Output = Self;
    fn shl(self, rhs: u64) -> Self::Output {
        BitMap64(self.0 << rhs)
    }
}
impl BitAnd for BitMap64 {
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
impl BitOr for BitMap64 {
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
impl BitXor for BitMap64 {
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

impl BitAnd<u64> for BitMap64 {
    type Output = Self;
    fn bitand(self, rhs: u64) -> Self::Output {
        BitMap64(self.0 & rhs)
    }
}
impl BitAndAssign<u64> for BitMap64 {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 = self.0 & rhs
    }
}
impl BitOr<u64> for BitMap64 {
    type Output = Self;
    fn bitor(self, rhs: u64) -> Self::Output {
        BitMap64(self.0 | rhs)
    }
}
impl BitOrAssign<u64> for BitMap64 {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 = self.0 | rhs
    }
}
impl BitXor<u64> for BitMap64 {
    type Output = Self;
    fn bitxor(self, rhs: u64) -> Self::Output {
        BitMap64(self.0 ^ rhs)
    }
}
impl BitXorAssign<u64> for BitMap64 {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.0 = self.0 ^ rhs
    }
}

impl IntoIterator for &BitMap64 {
    type Item = bool;

    type IntoIter = BitMap64Iterator;

    fn into_iter(self) -> Self::IntoIter {
        BitMap64Iterator {
            bitmap: *self, //copy semantics, it is safe to just move it here
            current: 0,
        }
    }
}
pub struct BitMap64Iterator {
    bitmap: BitMap64,
    current: u64,
}
impl Iterator for BitMap64Iterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 64 {
            return None;
        }
        let res = self.bitmap.get_bit(self.current);
        self.current += 1;
        Some(res)
    }
}
