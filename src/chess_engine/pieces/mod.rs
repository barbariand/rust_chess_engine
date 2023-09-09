use super::board::{Board, BoardPosition, MoveOffset};
use std::{fmt::Debug, marker::PhantomData};
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
    fn get_movement_options(pos: BoardPosition, board: Board, color: &Color) -> MovementOptions
    where
        Self: Sized;
}
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Black,
    White,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Piece<U: PieceMovement + ?Sized> {
    color: Color,
    type_of_pice: PhantomData<U>,
}
impl<U: PieceMovement> Piece<U> {
    fn get_movement_options(&self, board: Board, pos: BoardPosition) -> MovementOptions
    where
        Self: Sized,
    {
        U::get_movement_options(pos, board, &self.color)
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
    type Item = BoardPosition;
    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }
        let new_pos = (self.pos.clone() + self.changer).ok()?;
        self.end = self.board.has_piece(&new_pos);
        self.pos = new_pos.clone();
        Some(new_pos)
    }
}
