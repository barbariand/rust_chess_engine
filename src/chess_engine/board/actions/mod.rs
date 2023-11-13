use crate::chess_engine::errors::ActionError;
use super::board::InnerBoard;
use super::BoardPosition;
use crate::chess_engine::pieces::PieceType;  
use crate::chess_engine::pieces::Color;
pub(in super::super) trait Action:std::fmt::Debug{
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError>;
}
#[derive(Debug)]
pub struct MoveAction {
    from: BoardPosition,
    to: BoardPosition,
    piece: PieceType,
    piece_color: Color,
}
impl Action for MoveAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError> {
        todo!()
    }
}
#[derive(Debug)]
pub struct TakeAction {
    take_piece: PieceType,
    take_piece_color: Color,
}
impl Action for TakeAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError> {
        todo!()
    }
}
#[derive(Debug)]
pub struct PromoteAction {
    become_piece: PieceType,
    become_piece_color: Color,
}
impl Action for PromoteAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError> {
        todo!()
    }
}
#[derive(Debug)]
pub enum CastleAction {
    Right,
    Left,
}
impl Action for CastleAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError> {
        todo!()
    }
}
