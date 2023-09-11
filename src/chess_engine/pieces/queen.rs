use crate::chess_engine::board::MoveOffset;

use super::{Action, BoardPosition, BoardWalker, MovementOptions, PieceMovement};
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
pub struct Queen;
impl PieceMovement for Queen {
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
                .map(|walker| walker.collect::<Vec<Action>>())
                .reduce(|v, mut acc| {
                    acc.extend(v);
                    acc
                })
                .expect("iterator cant be zero beacuse we start it with 4"),
        }
    }
}
