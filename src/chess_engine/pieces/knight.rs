use crate::chess_engine::board::{Board, MoveOffset};

use super::{
    BoardPosition, BoardWalker, InnerAction, MovablePiece, MovementDirection, MovementOptions,
    Piece, PieceMovement, PieceStep,
};
#[derive(Debug, Clone)]
pub struct Knight;
impl MovablePiece for Knight {
    fn get_movement_options<'a>(
        pos: &BoardPosition,
        board: &'a Board,
        piece: &'a Piece,
    ) -> MovementOptions {
        let potential_moves = vec![
            // . .
            //   .
            //   *
            PieceMovement::new(PieceStep::Fixed(MovementDirection::North, 2))
                .addon(|| PieceMovement::new(PieceStep::Fixed(MovementDirection::West, 1)))
                .allowed_action(InnerAction::Take),
            // . .
            // .
            // *
            PieceMovement::new(PieceStep::Fixed(MovementDirection::North, 2))
                .addon(|| PieceMovement::new(PieceStep::Fixed(MovementDirection::East, 1)))
                .allowed_action(InnerAction::Take),
            //     .
            // * . .
            PieceMovement::new(PieceStep::Fixed(MovementDirection::East, 2))
                .addon(|| PieceMovement::new(PieceStep::Fixed(MovementDirection::North, 1)))
                .allowed_action(InnerAction::Take),
            // * . .
            //     .
            PieceMovement::new(PieceStep::Fixed(MovementDirection::East, 2))
                .addon(|| PieceMovement::new(PieceStep::Fixed(MovementDirection::South, 1)))
                .allowed_action(InnerAction::Take),
            //   *
            //   .
            // . .
            PieceMovement::new(PieceStep::Fixed(MovementDirection::South, 2))
                .addon(|| PieceMovement::new(PieceStep::Fixed(MovementDirection::West, 1)))
                .allowed_action(InnerAction::Take),
            // *
            // .
            // . .
            PieceMovement::new(PieceStep::Fixed(MovementDirection::South, 2))
                .addon(|| PieceMovement::new(PieceStep::Fixed(MovementDirection::East, 1)))
                .allowed_action(InnerAction::Take),
            // .
            // . . *
            PieceMovement::new(PieceStep::Fixed(MovementDirection::West, 2))
                .addon(|| PieceMovement::new(PieceStep::Fixed(MovementDirection::North, 1)))
                .allowed_action(InnerAction::Take),
            // . . *
            // .
            PieceMovement::new(PieceStep::Fixed(MovementDirection::West, 2))
                .addon(|| PieceMovement::new(PieceStep::Fixed(MovementDirection::South, 1)))
                .allowed_action(InnerAction::Take),
        ];

        MovementOptions::new(potential_moves, pos, board, piece)
    }
}
