use super::board::{Board, BoardPosition, MoveOffset};
use std::fmt::{Debug, Display};
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
pub enum Action {
    Take(BoardPosition),
    MoveTo(BoardPosition),
}
pub struct MovementOptions(Vec<Action>);
pub trait PieceMovement: Debug {
    fn get_movement_options(pos: BoardPosition, board: &Board, color: &Color) -> MovementOptions;
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
            InnerPiece::Bishop => Bishop::get_movement_options(pos, board, &self.color),
            InnerPiece::King => King::get_movement_options(pos, board, &self.color),
            InnerPiece::Knight => Knight::get_movement_options(pos, board, &self.color),
            InnerPiece::Pawn => Pawn::get_movement_options(pos, board, &self.color),
            InnerPiece::Queen => Queen::get_movement_options(pos, board, &self.color),
            InnerPiece::Rook => Rook::get_movement_options(pos, board, &self.color),
        })
    }
}

pub(super) struct BoardWalker<'a> {
    pos: BoardPosition,
    board: &'a Board,
    changer: MoveOffset,
    end: bool,
}
impl<'a> BoardWalker<'a> {
    fn new(pos: &BoardPosition, board: &'a Board, changer: MoveOffset) -> BoardWalker<'a> {
        BoardWalker {
            pos: pos.clone(),
            board,
            changer,
            end: false,
        }
    }
}
impl<'a> Iterator for BoardWalker<'a> {
    type Item = Action;
    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }
        let new_pos = (self.pos.clone() + self.changer).ok()?;
        self.end = self.board.has_piece(&new_pos);
        self.pos = new_pos.clone();
        Some(match self.end {
            true => Action::Take(new_pos),
            false => Action::MoveTo(new_pos),
        })
    }
}
impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.type_of_pice {
            InnerPiece::Bishop => write!(f, " B "),
            InnerPiece::King => write!(f, " K "),
            InnerPiece::Knight => write!(f, " Kn"),
            InnerPiece::Pawn => write!(f, " p "),
            InnerPiece::Queen => write!(f, " Q "),
            InnerPiece::Rook => write!(f, " R "),
        }
    }
}
