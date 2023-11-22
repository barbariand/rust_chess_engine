use super::board::InnerBoard;
use super::BoardPosition;
use crate::chess_engine::errors::BoardError;
use crate::chess_engine::pieces::Color;
use crate::chess_engine::pieces::PieceType;
use dyn_clone::DynClone;
trait Action: std::fmt::Debug + DynClone {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError>;
}
pub(super) trait PreformAction {
    fn perform(self, board: &mut InnerBoard) -> Result<(), BoardError>;
}
impl<T> PreformAction for T
where
    T: Action,
{
    fn perform(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        //println!("here i can do more stuff");
        self.execute(board)
    }
}
dyn_clone::clone_trait_object!(Action);
#[derive(Debug, Clone)]
#[allow(private_interfaces)] //actions should be viewable and exportable but not creatable by end user
pub enum Actions {
    Move(MoveAction),
    Take(TakeAction, MoveAction),
    Promote(MoveAction, PromoteAction),
    TakeAndPromote(TakeAction, MoveAction, PromoteAction),
    Castle(CastleAction),
    EnPassant(EnPassant),
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
            Actions::EnPassant(enpassant_action) => enpassant_action.execute(board),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct MoveAction {
    from: BoardPosition,
    to: BoardPosition,
    piece: PieceType,
    piece_color: Color,
}
impl MoveAction {
    pub fn new(
        from: BoardPosition,
        to: BoardPosition,
        piece: PieceType,
        piece_color: Color,
    ) -> Self {
        Self {
            from,
            to,
            piece,
            piece_color,
        }
    }
}
impl Action for MoveAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        board[self.piece].move_piece(&self.piece_color, &self.from, &self.to)
    }
}
#[derive(Debug, Clone)]
pub(super) struct TakeAction {
    take_piece: PieceType,
    take_piece_color: Color,
    pos: BoardPosition,
}
impl TakeAction {
    pub fn new(take_piece: PieceType, take_piece_color: Color, pos: BoardPosition) -> Self {
        Self {
            take_piece,
            take_piece_color,
            pos,
        }
    }
}
impl Action for TakeAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        board[self.take_piece].remove_piece_with_color(&self.pos, &self.take_piece_color)
    }
}
#[derive(Debug, Clone)]
pub(super) struct PromoteAction {
    was_piece: PieceType,
    pos: BoardPosition,
    become_piece: PieceType,
    piece_color: Color,
}
impl PromoteAction {
    pub fn new(
        was_piece: PieceType,
        pos: BoardPosition,
        become_piece: PieceType,
        piece_color: Color,
    ) -> Self {
        Self {
            was_piece,
            pos,
            become_piece,
            piece_color,
        }
    }
}
impl Action for PromoteAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), BoardError> {
        board[self.was_piece].remove_piece_with_color(&self.pos, &self.piece_color)?;
        board[self.become_piece].insert_piece_with_color(&self.pos, &self.piece_color)
    }
}
#[derive(Debug, Clone)]
pub(super) enum CastleAction {
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
pub(super) struct EnPassant {
    from: BoardPosition,
    to: BoardPosition,
    // the pos can be calculated from where it was comming and where it was going, if we had acces to history we would not need any info funily enough
}
impl EnPassant {
    pub fn new(from: BoardPosition, to: BoardPosition) -> Self {
        Self { from, to }
    }
}
impl Action for EnPassant {
    fn execute(self, _board: &mut InnerBoard) -> Result<(), BoardError> {
        todo!()
    }
}
