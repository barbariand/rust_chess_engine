use crate::chess_engine::board::{Board, MoveOffset};

use super::{
    BoardPosition, BoardWalker, InnerAction, MovablePiece, MovementDirection, MovementOptions,
    Piece, PieceMovement, PieceStep,
};
#[derive(Debug, Clone)]
pub struct Bishop;
impl MovablePiece for Bishop {
    fn get_movement_options<'a>(
        pos: &BoardPosition,
        board: &'a Board,
        piece: &'a Piece,
    ) -> MovementOptions {
        let potential_moves = vec![
            PieceMovement::new(PieceStep::Full(MovementDirection::NorthEast))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Full(MovementDirection::NorthWest))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Full(MovementDirection::SouthEast))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Full(MovementDirection::SouthWest))
                .allowed_action(InnerAction::Take),
        ];

        MovementOptions::new(potential_moves, pos, board, piece)
    }
}
