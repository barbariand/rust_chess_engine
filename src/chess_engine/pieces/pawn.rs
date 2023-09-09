use super::{BoardPosition, MovementOptions, PieceMovement};
#[derive(Debug, Clone)]
pub struct Pawn;
impl PieceMovement for Pawn {
    fn get_movement_options(
        pos: BoardPosition,
        board: crate::chess_engine::board::Board,
        color: &super::Color,
    ) -> MovementOptions
    where
        Self: Sized,
    {
        todo!("Hello")
    }
}
