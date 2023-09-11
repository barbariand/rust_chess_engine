use crate::chess_engine::board::MoveOffset;

use super::{Action, BoardPosition, MovementOptions, PieceMovement};
static POTENTIAL_MOVES: [(i8, i8); 8] = [
    (-1, -1), // Top-left
    (-1, 0),  // Top
    (-1, 1),  // Top-right
    (0, -1),  // Left
    (0, 1),   // Right
    (1, -1),  // Bottom-left
    (1, 0),   // Bottom
    (1, 1),   // Bottom-right
];
#[derive(Debug, Clone)]
pub struct King;
impl PieceMovement for King {
    fn get_movement_options(
        pos: BoardPosition,
        board: &crate::chess_engine::board::Board,
        _: &super::Color,
    ) -> MovementOptions
    where
        Self: Sized,
    {
        MovementOptions(
            POTENTIAL_MOVES
                .into_iter()
                .map(|v| (&pos + MoveOffset::from(v)).ok())
                .map(|v| {
                    v.map(|pos| match board.has_piece(&pos) {
                        true => Action::Take(pos),
                        false => Action::MoveTo(pos),
                    })
                })
                .into_iter()
                .flatten()
                .collect(),
        )
    }
}
