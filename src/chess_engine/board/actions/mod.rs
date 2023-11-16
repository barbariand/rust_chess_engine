use dyn_clone::DynClone;

use super::board::InnerBoard;
use super::BoardPosition;
use crate::chess_engine::errors::BoardError;
use crate::chess_engine::pieces::Color;
use crate::chess_engine::pieces::PieceType;
trait Action: std::fmt::Debug + DynClone {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError>;
}
pub(super) trait PreformAction {
    fn preform(self, board: &mut InnerBoard) -> Result<(), BoardError>;
}
impl<T> PreformAction for T
where
    T: Action,
{
    fn preform(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        //println!("here i can do more stuff");
        self.execute(board)
    }
}
dyn_clone::clone_trait_object!(Action);
#[derive(Debug, Clone)]
pub enum Actions {
    Move(MoveAction),
    Take(TakeAction, MoveAction),
    Promote(MoveAction, PromoteAction),
    TakeAndPromote(TakeAction, MoveAction, PromoteAction),
    Castle(CastleAction),
}
impl Action for Actions {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        match self {
            Actions::Move(move_action) => move_action.execute(board),
            Actions::Take(take_action, move_action) => {
                take_action.execute(board)?;
                move_action.execute(board)
            }
            Actions::Promote(move_action, promote_action) => {
                move_action.execute(board)?;
                promote_action.execute(board)
            }
            Actions::TakeAndPromote(take_action, move_action, promote_action) => {
                take_action.execute(board)?;
                move_action.execute(board)?;
                promote_action.execute(board)
            }
            Actions::Castle(castle_action) => castle_action.execute(board),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveAction {
    from: BoardPosition,
    to: BoardPosition,
    piece: PieceType,
    piece_color: Color,
}
impl Action for MoveAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        board[self.piece].move_piece(&self.piece_color, &self.from, &self.to)
    }
}
#[derive(Debug, Clone)]
pub struct TakeAction {
    take_piece: PieceType,
    take_piece_color: Color,
    pos: BoardPosition,
}
impl Action for TakeAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        board[self.take_piece].remove_piece_with_color(&self.pos, &self.take_piece_color)
    }
}
#[derive(Debug, Clone)]
pub struct PromoteAction {
    was_piece: PieceType,
    pos: BoardPosition,
    become_piece: PieceType,
    piece_color: Color,
}
impl Action for PromoteAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        board[self.was_piece].remove_piece_with_color(&self.pos, &self.piece_color)?;
        board[self.become_piece].insert_piece_with_color(&self.pos, &self.piece_color)
    }
}
#[derive(Debug, Clone)]
pub enum CastleAction {
    Long(Color),
    Short(Color),
}
impl Action for CastleAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        match self {
            Self::Long(c) => match c {
                Color::Black => {
                    board.get_all_occupied_squares();
                    todo!()
                }
                Color::White => {
                    board.get_all_occupied_squares();
                    todo!()
                }
            },
            Self::Short(c) => match c {
                Color::Black => {
                    todo!()
                }
                Color::White => {
                    todo!()
                }
            },
        }
    }
}
#[derive(Debug, Clone)]
pub struct EnPassant {
    from: BoardPosition,
    to: BoardPosition,
    // the pos can be calculated from where it was comming and where it was going, if we had acces to history we would not need any info funily enough
}
impl Action for EnPassant {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        todo!()
    }
}
