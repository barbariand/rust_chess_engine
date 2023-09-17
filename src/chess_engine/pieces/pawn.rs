use crate::chess_engine::board::{Board, MoveOffset, Rank};

use super::{
    Action, BoardPosition, Color, InnerAction, MovablePiece, MovementDirection, MovementOptions,
    Piece, PieceMovement, PieceStep,
};

#[derive(Debug, Clone)]
pub struct Pawn;
impl MovablePiece for Pawn {
    fn get_movement_options<'a>(
        pos: &BoardPosition,
        board: &'a Board,
        piece: &'a Piece,
    ) -> MovementOptions {
        let mut potential_moves = vec![
            PieceMovement::new(PieceStep::Fixed(MovementDirection::North, 1))
                .allowed_action(InnerAction::MoveTo),
            PieceMovement::new(PieceStep::Fixed(MovementDirection::NorthEast, 1))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Fixed(MovementDirection::NorthWest, 1))
                .allowed_action(InnerAction::Take),
        ];

        MovementOptions::new(potential_moves, pos, board, piece)
    }
}
