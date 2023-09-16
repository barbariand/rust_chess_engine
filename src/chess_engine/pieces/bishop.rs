use crate::chess_engine::board::MoveOffset;

use super::{BoardPosition, BoardWalker, MovementOptions, PieceMovement, Piece};
static POTENTIAL_MOVES: [(i8, i8); 4] = [
    (-1, -1), // Top
    (1, -1),  // Left
    (1, 1),   // Right
    (-1, 1),  // Bottom
];
#[derive(Debug, Clone)]
pub struct Bishop;
impl PieceMovement for Bishop {
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
                .iter()
                .map(|v| BoardWalker::new(&pos, &board, MoveOffset(v.0, v.1),piece))
                .flatten().collect()
        }
    }
}
