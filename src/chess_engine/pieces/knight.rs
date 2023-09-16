use crate::chess_engine::board::MoveOffset;

use super::{BoardPosition, MovementOptions, PieceMovement, Piece, BoardWalker};
static POTENTIAL_MOVES: [(i8, i8); 8] = [
    (-1, 3),
    (1, -3),
    (1, 3),
    (-1, -3),
    (3, 1),
    (3, -1),
    (-3, 1),
    (-3, -1),
];
#[derive(Debug, Clone)]
pub struct Knight;
impl PieceMovement for Knight {
    fn get_movement_options(
        piece:&Piece,
        pos: BoardPosition,
        board: &crate::chess_engine::board::Board,
        _: &super::Color,
    ) -> MovementOptions
    where
        Self: Sized,
    {
        MovementOptions {
            0: POTENTIAL_MOVES
                .into_iter()
                .map(|v| BoardWalker::new(&pos, &board, MoveOffset(v.0, v.1),piece).next()
            ).flatten().collect()
        }
    }
}
