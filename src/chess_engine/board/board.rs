use super::bitmap::BitMap64;
use super::BoardPosition;
use super::File;
use super::Rank;
use crate::chess_engine::history::History;
use crate::chess_engine::pieces::Action;
use crate::chess_engine::pieces::Bishop;
use crate::chess_engine::pieces::Color;
use crate::chess_engine::pieces::InnerPiece;
use crate::chess_engine::pieces::King;
use crate::chess_engine::pieces::Knight;
use crate::chess_engine::pieces::Pawn;
use crate::chess_engine::pieces::Piece;
use crate::chess_engine::pieces::Queen;
use crate::chess_engine::pieces::Rook;
use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug, Clone)]
struct InnerBoard {
    pieces: HashMap<InnerPiece, PieceBoard>,
}
#[derive(Debug, Clone)]
struct PieceBoard {
    white_piece: BitMap64,
    black_piece: BitMap64,
}
impl Default for InnerBoard {
    fn default() -> Self {
        InnerBoard {
            pieces: HashMap::new(),
        }
    }
}

impl InnerBoard {
    fn get(&self, index: BoardPosition) -> Option<Piece> {
        let square = index.to_num();
        let num = self.pawns.get_white_squares().get_bit_value(square) << 1
            | self.rook.get_white_squares().get_bit_value(square) << 2
            | self.knight.get_white_squares().get_bit_value(square) << 3
            | self.bishop.get_white_squares().get_bit_value(square) << 4
            | self.kings.get_white_squares().get_bit_value(square) << 5
            | self.queen.get_white_squares().get_bit_value(square) << 6
            | self.pawns.get_black_squares().get_bit_value(square) << 7
            | self.rook.get_black_squares().get_bit_value(square) << 8
            | self.knight.get_black_squares().get_bit_value(square) << 9
            | self.bishop.get_black_squares().get_bit_value(square) << 10
            | self.kings.get_black_squares().get_bit_value(square) << 11
            | self.queen.get_black_squares().get_bit_value(square) << 12;
        Piece::try_from_bitmap(index, num)
    }
}

impl PieceBoard {
    fn new_custom(white: i64, black: i64) -> PieceBoard<T> {
        PieceBoard {
            white_piece: BitMap64::new(white),
            black_piece: BitMap64::new(black),
            phantom: PhantomData::default(),
        }
    }
}
impl PieceBoard {
    fn remove_piece(&mut self, pos: &BoardPosition) {
        self.white_piece = self.white_piece ^ (0b1 << pos.to_num());
        self.black_piece = self.black_piece ^ (0b1 << pos.to_num());
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

    fn is_square_occupied(&self, square: u64) -> bool {
        self.get_occupied_squares().contains(square)
    }

    fn is_white_piece_on_square(&self, square: u64) -> bool {
        self.get_white_squares().contains(square)
    }

    fn is_black_piece_on_square(&self, square: u64) -> bool {
        self.get_black_squares().contains(square)
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
    pub fn remove_piece(&mut self) {
        //impl stuff
    }
    pub fn get_all_occupied_squares(&self) -> BitMap64 {
        self.pawns.get_occupied_squares()
            | self.kings.get_occupied_squares()
            | self.queen.get_occupied_squares()
            | self.rook.get_occupied_squares()
            | self.bishop.get_occupied_squares()
            | self.knight.get_occupied_squares()
    }

    pub fn get_all_white_squares(&self) -> BitMap64 {
        self.pawns.get_white_squares()
            | self.kings.get_white_squares()
            | self.queen.get_white_squares()
            | self.rook.get_white_squares()
            | self.bishop.get_white_squares()
            | self.knight.get_white_squares()
    }

    pub fn get_all_black_squares(&self) -> BitMap64 {
        self.pawns.get_black_squares()
            | self.kings.get_black_squares()
            | self.queen.get_black_squares()
            | self.rook.get_black_squares()
            | self.bishop.get_black_squares()
            | self.knight.get_black_squares()
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
