use crate::chess_engine::board::MoveOffset;

use super::{Action, BoardPosition, BoardWalker, MovementOptions, PieceMovement};
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
                .map(|v| BoardWalker::new(&pos, &board, MoveOffset(v.0, v.1)))
                .map(|walker| walker.collect::<Vec<BoardPosition>>())
                .reduce(|v, mut acc| {
                    acc.extend(v);
                    acc
                })
                .expect("iterator cant be zero beacuse we start it with 4")
                .into_iter()
                .map(|pos| match board.has_piece(&pos) {
                    true => Action::Take(pos),
                    false => Action::MoveTo(pos),
                })
                .collect(),
        }
    }
}
