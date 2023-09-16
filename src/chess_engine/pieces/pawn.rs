use crate::chess_engine::board::{MoveOffset, Rank};

use super::{Action, BoardPosition, Color, MovementOptions, PieceMovement, Piece};

#[derive(Debug, Clone)]
pub struct Pawn;
impl PieceMovement for Pawn {
    fn get_movement_options(
        piece:&Piece,
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
                false => Action::new(piece, board, v).ok(),
            }))
        } else if pos.rank == Rank::Seven && *color == Color::Black {
            let temp_pos = (pos.clone() + MoveOffset(-2, 0)).ok();
            potential_moves.push(temp_pos.and_then(|v| match board.has_piece(&v) {
                true => None,
                false => Action::new(piece, board, v).ok(),
            }))
        }

        let temp_pos = (pos.clone() + MoveOffset(1, 1)).ok();
        potential_moves.push(temp_pos.and_then(|v| match board.has_piece(&v) {
            true => Action::new(piece, board, v).ok(),
            false => None,
        }));

        let temp_pos = (pos.clone() + MoveOffset(1, 1)).ok();
        potential_moves.push(temp_pos.and_then(|v| match board.has_piece(&v) {
            true => Action::new(piece, board, v).ok(),
            false => None,
        }));
        let temp_pos = (pos.clone() + MoveOffset(1, 0)).ok();
        potential_moves.push(temp_pos.and_then(|v| match board.has_piece(&v) {
            true => Action::new(piece, board, v).ok(),
            false => None,
        }));
        MovementOptions {
            0: potential_moves.into_iter().flatten().collect(),
        }
    }
}
