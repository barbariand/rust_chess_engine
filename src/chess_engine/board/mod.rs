mod board;
mod board_position;
mod file;
mod rank;
pub use board::Board;
pub use board_position::BoardPosition;
pub use file::File;
pub use rank::Rank;
#[derive(Clone, Copy)]
pub struct MoveOffset(pub i8, pub i8);
impl<U, T> From<(U, T)> for MoveOffset
where
    U: Into<i8>,
    T: Into<i8>,
{
    fn from((v1, v2): (U, T)) -> Self {
        MoveOffset(v1.into(), v2.into())
    }
}
