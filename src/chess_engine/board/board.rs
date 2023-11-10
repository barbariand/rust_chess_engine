use super::bitmap::BitMap64;
use super::BoardPosition;
use super::File;
use super::Rank;
use crate::chess_engine::errors::*;
use crate::chess_engine::history::History;
use crate::chess_engine::pieces::Action;
use crate::chess_engine::pieces::Bishop;
use crate::chess_engine::pieces::Color;
use crate::chess_engine::pieces::King;
use crate::chess_engine::pieces::Knight;
use crate::chess_engine::pieces::Pawn;
use crate::chess_engine::pieces::Piece;
use crate::chess_engine::pieces::PieceType;
use crate::chess_engine::pieces::Queen;
use crate::chess_engine::pieces::Rook;
use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug, Clone)]
struct InnerBoard {
    pieces: PieceBoards,
}
type PieceBoards = [(PieceType, PieceBoard); 6];
#[derive(Debug, Clone, Copy)]
struct PieceBoard {
    white_piece: BitMap64,
    black_piece: BitMap64,
}
impl Default for InnerBoard {
    fn default() -> Self {
        use PieceType as PT;
        let kings = PieceBoard::new_king();
        let queens = PieceBoard::new_queen();
        let bishops = PieceBoard::new_bishop();
        let rooks = PieceBoard::new_rook();
        let knights = PieceBoard::new_knight();
        let pawns = PieceBoard::new_pawn();
        InnerBoard {
            pieces: [
                (PT::Pawn, pawns),
                (PT::Rook, rooks),
                (PT::Knight, knights),
                (PT::Bishop, bishops),
                (PT::King, kings),
                (PT::Queen, queens),
            ],
        }
    }
}
impl Index<PieceType> for InnerBoard {
    type Output = PieceBoard;
    fn index(&self, index: PieceType) -> &Self::Output {
        &self.get(index as usize).expect("This bounds should never be violated, check the enum values").1
    }
}
impl IndexMut<PieceType> for InnerBoard {
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        &mut self.pieces.get_mut(index as usize).expect("This bounds should never be violated, check the enum values").1
    }
}
impl Deref for InnerBoard {
    type Target = PieceBoards;
    fn deref(&self) -> &Self::Target {
        &self.pieces
    }
}
impl InnerBoard {
    fn get_piece(&self, index: BoardPosition) -> Option<Piece> {
        let square = index.to_num();
        let mut num = 0;
        for (piece_type, board) in self.deref() {
            num |= board.get_white_squares().get_bit_value(square) << (*piece_type as i64)
                | board.get_black_squares().get_bit_value(square) << (*piece_type as i64 + 6)
        }
        Piece::try_from_bitmap(index, num)
    }
}

impl PieceBoard {
    fn new_custom(white: i64, black: i64) -> PieceBoard {
        PieceBoard {
            white_piece: BitMap64::new(white),
            black_piece: BitMap64::new(black),
        }
    }
    fn remove_piece_with_color(&mut self, pos: &BoardPosition, color: Color) {
        match color {
            Color::White => self.white_piece,
            Color::Black => self.black_piece,
        }
        .clear_bit(pos.to_num())
    }
    fn remove_piece(&mut self, pos: &BoardPosition){
        self.white_piece.clear_bit(pos.to_num());
        self.black_piece.clear_bit(pos.to_num());
    }
    fn insert_unchecked_with_color(&mut self, pos: &BoardPosition, color: Color) {
        match color {
            Color::White => self.white_piece,
            Color::Black => self.black_piece,
        }
        .set_bit(pos.to_num())
    }
    fn move_piece_unchecked(&mut self, color: Color, from: &BoardPosition, to: &BoardPosition) {
        let mut pieces = match color {
            Color::White => self.white_piece,
            Color::Black => self.black_piece,
        };
        pieces.clear_bit(from.to_num());
        pieces.set_bit(to.to_num());
    }

    pub fn move_piece(
        &mut self,
        color: Color,
        from: &BoardPosition,
        to: &BoardPosition,
    ) -> Result<(), Error> {
        match (!self.is_square_occupied(from), self.is_square_occupied(to)) {
            (false, false) => {
                self.move_piece_unchecked(color, from, to);
                Ok(())
            }
            (true, _) => Err(Error::Board(BoardError::PieceMissing)),
            (_, true) => Err(Error::Board(BoardError::PieceWhereMoving)),
        }
    }

    fn get_occupied_squares(&self) -> BitMap64 {
        self.white_piece | self.black_piece
    }

    fn get_white_squares(&self) -> BitMap64 {
        self.white_piece
    }

    fn get_black_squares(&self) -> BitMap64 {
        self.black_piece
    }

    fn is_square_occupied(&self, pos: &BoardPosition) -> bool {
        self.get_occupied_squares().contains(pos.into())
    }
    fn is_square_occupied_color(&self, pos: &BoardPosition, color: Color) -> bool {
        match color {
            Color::White => self.white_piece,
            Color::Black => self.black_piece,
        }
        .contains(pos.to_num())
    }

    fn is_white_piece_on_square(&self, pos: &BoardPosition) -> bool {
        self.get_white_squares().contains(pos.to_num())
    }

    fn is_black_piece_on_square(&self, pos: &BoardPosition) -> bool {
        self.get_black_squares().contains(pos.to_num())
    }

    fn count_pieces(&self) -> u8 {
        self.get_occupied_squares().count_ones()
    }

    fn count_white_pieces(&self) -> u8 {
        self.get_white_squares().count_ones()
    }

    fn count_black_pieces(&self) -> u8 {
        self.get_black_squares().count_ones()
    }
    fn new_pawn() -> PieceBoard {
        PieceBoard::new_custom(0b11111111 << 56, 0b11111111 << 8)
    }
    fn new_king() -> PieceBoard {
        PieceBoard::new_custom(0b1 << 60, 0b1 << 5)
    }
    fn new_bishop() -> PieceBoard {
        PieceBoard::new_custom(0b00100100 << 56, 0b00100100 << 8)
    }
    fn new_knight() -> PieceBoard {
        PieceBoard::new_custom(0b01000010 << 56, 0b01000010 << 8)
    }
    fn new_rook() -> PieceBoard {
        PieceBoard::new_custom(0b10000001 << 56, 0b10000001 << 8)
    }
    fn new_queen() -> PieceBoard {
        PieceBoard::new_custom(0b1 << 59, 0b1 << 4)
    }
}

impl InnerBoard {
    pub fn remove_piece(&mut self, piece: PieceType, pos: &BoardPosition) {
        self[piece].remove_piece(pos)
    }
    pub fn move_piece(&mut self,
        color: Color,
        from: &BoardPosition,
        piece: PieceType,
        to: &BoardPosition,)->Result<(),Error>{
            self[piece].move_piece(color,from,to)
        }
        pub fn take_piece(&mut self,
            color: Color,
            from: &BoardPosition,
            piece: PieceType,
            to: &BoardPosition,)->Result<(),Error>{
                let mut board=self[piece];
                board.remove_piece_with_color(to,!color);
                let res=board.move_piece(color,from,to);
                self[piece]=board;
                res
            }
    pub fn get_all_occupied_squares(&self) -> BitMap64 {
        self.get_all_squares(|v| v.get_occupied_squares())
    }

    pub fn get_all_white_squares(&self) -> BitMap64 {
        self.get_all_squares(|v| v.get_white_squares())
    }

    pub fn get_all_black_squares(&self) -> BitMap64 {
        self.get_all_squares(|v| v.get_black_squares())
    }

    fn get_all_squares(&self, func: impl Fn(&PieceBoard) -> BitMap64) -> BitMap64 {
        self.iter()
            .map(|v| func(&v.1))
            .reduce(|acc, piece_board| (acc | piece_board))
            .unwrap_or_default()
    }

    pub fn has(&self, square: u64) -> bool {
        self.get_all_occupied_squares().contains(square)
    }

    pub fn has_white(&self, square: u64) -> bool {
        self.get_all_white_squares().contains(square)
    }

    pub fn has_black(&self, square: u64) -> bool {
        self.get_all_black_squares().contains(square)
    }

    fn count_all_pieces(&self) -> u8 {
        self.get_all_occupied_squares().count_ones()
    }

    fn count_all_white_pieces(&self) -> u8 {
        self.get_all_white_squares().count_ones()
    }

    fn count_all_black_pieces(&self) -> u8 {
        self.get_all_black_squares().count_ones()
    }
}
#[derive(Debug, Clone)]
pub struct Board {
    inner_board: InnerBoard,
    turn: Color,
    history: History,
}

impl Board {
    pub fn has_piece(&self, pos: &BoardPosition) -> bool {
        self.inner_board
            .get_all_occupied_squares()
            .contains(pos.to_num())
    }
    pub fn is_piece_color(&self, pos: &BoardPosition, color: Color) -> bool {
        match color {
            Color::White => self.inner_board.get_all_white_squares(),
            Color::Black => self.inner_board.get_all_white_squares(),
        }
        .contains(pos.to_num())
    }
    pub fn new() -> Board {
        Board {
            inner_board: Default::default(),
            turn: Color::White,
            history: History(Vec::new()),
        }
    }
    pub fn move_piece(&mut self, action: Action) {
        action.execute(self);
        self.history.add(action);
    }
    pub fn get_movement_options() {}
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
