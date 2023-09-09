use super::file;
use super::rank;
use super::BoardPosition;
use crate::chess_engine::pieces::{Piece, PieceMovement};

#[derive(Debug)]
pub struct Board {
    inner_board: [[Option<Piece<dyn PieceMovement>>; 8]; 8],
}

impl Board {
    pub fn has_piece(&self, pos: &BoardPosition) -> bool {
        self.inner_board[<&file::File as Into<i8>>::into(&pos.file) as usize]
            [<&rank::Rank as Into<i8>>::into(&pos.rank) as usize]
            .is_some()
    }
}
