use crate::chess_engine::board::MoveOffset;

use super::{ BoardPosition, BoardWalker, MovementOptions, PieceMovement, Piece};
static POTENTIAL_MOVES: [(i8, i8); 4] = [
    (-1, 0), // Top
    (0, -1), // Left
    (0, 1),  // Right
    (1, 0),  // Bottom
];
#[derive(Debug, Clone)]
pub struct Rook;
impl PieceMovement for Rook {
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
