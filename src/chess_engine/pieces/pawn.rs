use crate::chess_engine::board::{MoveOffset, Rank};

use super::{Action, BoardPosition, Color, MovementOptions, PieceMovement};

#[derive(Debug, Clone)]
pub struct Pawn;
impl PieceMovement for Pawn {
    fn get_movement_options(
        pos: BoardPosition,
        board: &crate::chess_engine::board::Board,
        color: &super::Color,
    ) -> MovementOptions
    where
        Self: Sized,
    {
        let mut potential_moves = Vec::new();

        if pos.rank == Rank::Two && *color == Color::White {
            let temp_pos = (pos.clone() + MoveOffset(2, 0)).ok();
            potential_moves.push(temp_pos.and_then(|v| match board.has_piece(&v) {
                true => None,
                false => Some(Action::MoveTo(v)),
            }))
        } else if pos.rank == Rank::Seven && *color == Color::Black {
            let temp_pos = (pos.clone() + MoveOffset(-2, 0)).ok();
            potential_moves.push(temp_pos.and_then(|v| match board.has_piece(&v) {
                true => None,
                false => Some(Action::MoveTo(v)),
            }))
        }

        let temp_pos = (pos.clone() + MoveOffset(1, 1)).ok();
        potential_moves.push(temp_pos.and_then(|v| match board.has_piece(&v) {
            true => Some(Action::Take(v)),
            false => None,
        }));

        let temp_pos = (pos.clone() + MoveOffset(1, 1)).ok();
        potential_moves.push(temp_pos.and_then(|v| match board.has_piece(&v) {
            true => Some(Action::Take(v)),
            false => None,
        }));
        let temp_pos = (pos.clone() + MoveOffset(1, 0)).ok();
        potential_moves.push(temp_pos.and_then(|v| match board.has_piece(&v) {
            true => Some(Action::Take(v)),
            false => None,
        }));
        MovementOptions {
            0: potential_moves.into_iter().flatten().collect(),
        }
    }
}
