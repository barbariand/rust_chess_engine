use crate::chess_engine::board::MoveOffset;

use super::{Action, BoardPosition, BoardWalker, MovementOptions, PieceMovement};
static potential_moves: [(i8, i8); 4] = [
    (-1, -1), // Top
    (1, -1),  // Left
    (1, 1),   // Right
    (-1, 1),  // Bottom
];
#[derive(Debug, Clone)]
pub struct Bishop;
impl PieceMovement for Bishop {
    fn get_movement_options(
        pos: BoardPosition,
        board: crate::chess_engine::board::Board,
        color: &super::Color,
    ) -> MovementOptions
    where
        Self: Sized,
    {
        MovementOptions {
            0: potential_moves
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
