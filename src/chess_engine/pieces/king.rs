use crate::chess_engine::board::{Board, MoveOffset};

use super::{
    BoardPosition, InnerAction, MovablePiece, MovementDirection, MovementOptions, Piece,
    PieceMovement, PieceStep,
};
#[derive(Debug, Clone)]
pub struct King;
impl MovablePiece for King {
    fn get_movement_options<'a>(
        pos: &BoardPosition,
        board: &'a Board,
        piece: &'a Piece,
    ) -> MovementOptions {
        let potential_moves = vec![
            PieceMovement::new(PieceStep::Fixed(MovementDirection::East, 1))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Fixed(MovementDirection::North, 1))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Fixed(MovementDirection::NorthEast, 1))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Fixed(MovementDirection::NorthWest, 1))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Fixed(MovementDirection::South, 1))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Fixed(MovementDirection::SouthEast, 1))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Fixed(MovementDirection::SouthWest, 1))
                .allowed_action(InnerAction::Take),
            PieceMovement::new(PieceStep::Fixed(MovementDirection::West, 1))
                .allowed_action(InnerAction::Take),
        ];

        MovementOptions::new(potential_moves, pos, board, piece)
    }
}

// Hej Ludvig. Alvin skrev detta på Cindys dator!!! hehe