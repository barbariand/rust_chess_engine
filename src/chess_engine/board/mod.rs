mod board;
mod board_position;
mod file;
mod rank;
use std::ops::Mul;
pub mod actions;
pub mod bitmap;
pub use board::Board;
pub use board_position::BoardPosition;
pub use file::File;
pub use rank::Rank;
#[derive(Clone, Copy)]
pub struct MoveOffset(pub u8, pub u8); // should prob remove
impl<U, T> From<(U, T)> for MoveOffset
where
    U: Into<u8>,
    T: Into<u8>,
{
    fn from((v1, v2): (U, T)) -> Self {
        MoveOffset(v1.into(), v2.into())
    }
}
impl Mul<&u8> for MoveOffset {
    type Output = MoveOffset;
    fn mul(self, rhs: &u8) -> Self::Output {
        MoveOffset(self.0 * rhs, self.1 * rhs)
    }
}
