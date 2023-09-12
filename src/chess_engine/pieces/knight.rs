use crate::chess_engine::board::MoveOffset;

use super::{Action, BoardPosition, MovementOptions, PieceMovement};
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
                .map(|v| (&pos + MoveOffset::from(v)).ok())
                .flat_map(|v| {
                    v.map(|pos| match board.has_piece(&pos) {
                        true => Action::Take(pos),
                        false => Action::MoveTo(pos),
                    })
                }).collect()
        }
    }
}
