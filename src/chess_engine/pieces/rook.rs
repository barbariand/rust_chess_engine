use crate::chess_engine::board::{Board, MoveOffset};

use super::{
    BoardPosition, BoardWalker, InnerAction, MovablePiece, MovementOptions, Piece, PieceMovement,
    PieceStep,
};

#[derive(Debug, Clone)]
pub struct Rook;
impl MovablePiece for Rook {
    fn get_movement_options<'a>(
        pos: &BoardPosition,
        board: &'a Board,
        piece: &'a Piece,
    ) -> MovementOptions {
        let potential_moves = vec![
            PieceMovement::new(PieceStep::Full(super::MovementDirection::East))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Full(super::MovementDirection::North))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Full(super::MovementDirection::South))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Full(super::MovementDirection::West))
                .allowed_action(InnerAction::Take),
        ];

        MovementOptions::new(potential_moves, pos, board, piece)
    }
}
