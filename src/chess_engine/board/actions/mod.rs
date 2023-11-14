use dyn_clone::DynClone;

use super::board::InnerBoard;
use super::BoardPosition;
use crate::chess_engine::errors::ActionError;
use crate::chess_engine::pieces::Color;
use crate::chess_engine::pieces::PieceType;

pub(in super::super) trait Action: std::fmt::Debug + DynClone {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError>;
}
dyn_clone::clone_trait_object!(Action);

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct TakeAction {
    take_piece: PieceType,
    take_piece_color: Color,
}
impl Action for TakeAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError> {
        todo!()
    }
}
#[derive(Debug, Clone)]
pub struct PromoteAction {
    become_piece: PieceType,
    become_piece_color: Color,
}
impl Action for PromoteAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError> {
        todo!()
    }
}
#[derive(Debug, Clone)]
pub enum CastleAction {
    Long,
    Short,
}
impl Action for CastleAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError> {
        todo!()
    }
}
