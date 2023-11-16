use dyn_clone::DynClone;

use super::board::InnerBoard;
use super::BoardPosition;
use crate::chess_engine::errors::BoardError;
use crate::chess_engine::pieces::Color;
use crate::chess_engine::pieces::PieceType;
#[derive(Debug, Clone)]
pub enum Actions {
    Move(MoveAction),
    Take(TakeAction),
    Promote(PromoteAction),
    Castle(CastleAction),
}
impl Action for Actions {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        match self {
            Actions::Move(a) => a.execute(board),
            Actions::Take(a) => a.execute(board),
            Actions::Promote(a) => a.execute(board),
            Actions::Castle(a) => a.execute(board),
        }
    }
}
pub(in super::super) trait Action: std::fmt::Debug + DynClone {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError>;
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
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        board[self.piece].move_piece(self.piece_color, &self.from, &self.to)
    }
}
#[derive(Debug, Clone)]
pub struct TakeAction {
    take_piece: PieceType,
    take_piece_color: Color,
    place:BoardPosition,
}
impl Action for TakeAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        let pb=board[self.take_piece];
        todo!()
    }
}
#[derive(Debug, Clone)]
pub struct PromoteAction {
    was_piece:PieceType,
    pos:BoardPosition,
    become_piece: PieceType,
    become_piece_color: Color,
}
impl Action for PromoteAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        todo!()
    }
}
#[derive(Debug, Clone)]
pub enum CastleAction {
    Long(Color),
    Short(Color),
}
impl Action for CastleAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        todo!()
    }
}
#[derive(Debug, Clone)]
pub struct EnPassant {
    from: BoardPosition,
    to: BoardPosition,
    // the pos can be calculated from where it was comming and where it was going, if we had acces to history we would not need any info
}
impl Action for EnPassant {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        todo!()
    }
}
