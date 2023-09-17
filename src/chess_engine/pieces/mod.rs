use super::{
    board::{Board, BoardPosition, MoveOffset},
    errors::{ActionError, Error},
};
use std::{
    fmt::{Debug, Display},
    ops::Deref,
};
mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
pub use bishop::Bishop;
pub use king::King;
pub use knight::Knight;
pub use pawn::Pawn;
pub use queen::Queen;
pub use rook::Rook;

#[derive(Debug, Clone)]
pub struct Action {
    pub piece_pos: BoardPosition,
    inner_action: InnerAction,
    to_pos: BoardPosition,
}
impl Action {
    pub fn new(piece: &Piece, board: &Board, pos: BoardPosition) -> Result<Self, Error> {
        let piece_pos = piece.pos.ok_or(ActionError::PieceNotInPlay)?.clone();
        match (
            board.has_piece(&pos),
            board.is_piece_color(&pos, piece.color),
        ) {
            (true, true) => Err(ActionError::SameColor.into()),
            (true, false) => Ok(Action {
                inner_action: InnerAction::Take,
                piece_pos,
                to_pos: pos,
            }),
            (false, _) => Ok(Action {
                inner_action: InnerAction::MoveTo,
                piece_pos,
                to_pos: pos,
            }),
        }
    }
    pub fn execute(&self, board: &mut Board) {
        match self.inner_action {
            InnerAction::MoveTo => {
                let mut moved = board[&self.piece_pos]
                    .take()
                    .expect("Unreatcheble beacuse allready checked why hacking");
                moved.pos = Some(self.piece_pos);
                board[&self.to_pos] = Some(moved)
            }
            InnerAction::Take => {
                let mut piece = board[&self.to_pos]
                    .take()
                    .expect("Unreatcheble beacuse allready checked why hacking");
                piece.pos = None;
                let mut moved = board[&self.piece_pos]
                    .take()
                    .expect("Unreatcheble beacuse allready checked why hacking");
                moved.pos = Some(self.piece_pos);
                board[&self.to_pos] = Some(moved)
            }
        }
    }
}
#[derive(Debug, Clone)]
enum InnerAction {
    Take,
    MoveTo,
}
pub struct MovementOptions(Vec<Action>);
impl Deref for MovementOptions {
    type Target = Vec<Action>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
trait PieceMovement {
    fn get_movement_options(
        piece: &Piece,
        pos: BoardPosition,
        board: &crate::chess_engine::board::Board,
        color: &Color,
    ) -> MovementOptions;
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Color {
    Black,
    White,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Piece {
    pub color: Color,
    pub pos: Option<BoardPosition>,
    type_of_pice: InnerPiece,
}
#[derive(Debug, Clone, PartialEq, Copy)]
enum InnerPiece {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

impl Piece {
    pub fn new_pawn(color: Color) -> Piece {
        Piece {
            color,
            pos: None,
            type_of_pice: InnerPiece::Pawn,
        }
    }
    pub fn new_knight(color: Color) -> Piece {
        Piece {
            color,
            pos: None,
            type_of_pice: InnerPiece::Knight,
        }
    }
    pub fn new_king(color: Color) -> Piece {
        Piece {
            color,
            pos: None,
            type_of_pice: InnerPiece::King,
        }
    }
    pub fn new_queen(color: Color) -> Piece {
        Piece {
            color,
            pos: None,
            type_of_pice: InnerPiece::Queen,
        }
    }
    pub fn new_bishop(color: Color) -> Piece {
        Piece {
            color,
            pos: None,
            type_of_pice: InnerPiece::Bishop,
        }
    }
    pub fn new_rook(color: Color) -> Piece {
        Piece {
            color,
            pos: None,
            type_of_pice: InnerPiece::Rook,
        }
    }
}
impl Piece {
    pub fn get_movement_options(&self, board: &Board) -> Option<MovementOptions>
    where
        Self: Sized,
    {
        self.pos.map(|pos| match self.type_of_pice {
            InnerPiece::Bishop => Bishop::get_movement_options(self, pos, board, &self.color),
            InnerPiece::King => King::get_movement_options(self, pos, board, &self.color),
            InnerPiece::Knight => Knight::get_movement_options(self, pos, board, &self.color),
            InnerPiece::Pawn => Pawn::get_movement_options(self, pos, board, &self.color),
            InnerPiece::Queen => Queen::get_movement_options(self, pos, board, &self.color),
            InnerPiece::Rook => Rook::get_movement_options(self, pos, board, &self.color),
        })
    }
}

pub(super) struct BoardWalker<'a> {
    piece: &'a Piece,
    pos: BoardPosition,
    board: &'a Board,
    changer: MoveOffset,
}
impl<'a> BoardWalker<'a> {
    fn new(
        pos: &BoardPosition,
        board: &'a Board,
        changer: MoveOffset,
        piece: &'a Piece,
    ) -> BoardWalker<'a> {
        BoardWalker {
            piece,
            pos: pos.clone(),
            board,
            changer,
        }
    }
}
impl<'a> Iterator for BoardWalker<'a> {
    type Item = Action;
    fn next(&mut self) -> Option<Self::Item> {
        let new_pos = (self.pos.clone() + self.changer).ok()?;
        self.pos = new_pos;
        Action::new(self.piece, self.board, self.pos).ok()
    }
}
impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.color, self.type_of_pice) {
            (Color::White, InnerPiece::Bishop) => write!(f, " ♝ "),
            (Color::White, InnerPiece::King) => write!(f, " ♚ "),
            (Color::White, InnerPiece::Knight) => write!(f, " ♞ "),
            (Color::White, InnerPiece::Pawn) => write!(f, " ♟︎ "),
            (Color::White, InnerPiece::Queen) => write!(f, " ♛ "),
            (Color::White, InnerPiece::Rook) => write!(f, " ♜ "),
            (Color::Black, InnerPiece::Bishop) => write!(f, " ♗ "),
            (Color::Black, InnerPiece::King) => write!(f, " ♔ "),
            (Color::Black, InnerPiece::Knight) => write!(f, " ♘ "),
            (Color::Black, InnerPiece::Pawn) => write!(f, " ♙ "),
            (Color::Black, InnerPiece::Queen) => write!(f, " ♕ "),
            (Color::Black, InnerPiece::Rook) => write!(f, " ♖ "),
        }
    }
}
