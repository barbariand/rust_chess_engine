use super::actions::PreformAction;
use super::actions::*;
use super::bitmap::BitMap64;
use super::BoardPosition;
use crate::chess_engine::board::actions::CastleAction;
use crate::chess_engine::errors::*;
use crate::chess_engine::history::History;
use crate::chess_engine::pieces::tables;
use crate::chess_engine::pieces::Color;
use crate::chess_engine::pieces::PieceType;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub(super) struct InnerBoard {
    pieces: PieceBoards,
}
impl InnerBoard {
    fn perform_actions(&mut self, actions: Actions) -> Result<(), BoardError> {
        actions.preform(self)
    }

    fn what_piece_on_square(&self, pos: &BoardPosition) -> Option<PieceType> {
        for (pb, i) in self.pieces.iter().zip(PieceType::Pawn..PieceType::Queen) {
            if pb.is_square_occupied(pos) {
                return Some(i);
            }
        }
        None
    }
    fn what_color_is_on_square(&self, pos: &BoardPosition) -> Option<Color> {
        for pb in self.pieces {
            if pb.is_white_piece_on_square(pos) {
                return Some(Color::White);
            }
            if pb.is_black_piece_on_square(pos) {
                return Some(Color::Black);
            }
        }
        None
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
            .map(|v| func(&v))
            .reduce(|acc, piece_board| (acc | piece_board))
            .unwrap_or_default()
    }

    fn has(&self, square: u64) -> bool {
        self.get_all_occupied_squares().contains(square)
    }

    fn has_white_piece(&self, square: u64) -> bool {
        self.get_all_white_squares().contains(square)
    }

    fn has_black_piece(&self, square: u64) -> bool {
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
    pub fn get_movement_options(&self, color: Color) -> Vec<Actions> {
        self.iter()
            .flat_map(|pb| pb.get_movement_options(color))
            .collect()
    }
    pub fn can_take(
        &self,
        taker: PieceType,
        target: PieceType, //technicaly we dont need this but it is nice so we dont sort thrugh everyone lmao
        from: BoardPosition,
        to: BoardPosition,
    ) -> bool {
        todo!()
    }
}
impl Default for InnerBoard {
    fn default() -> Self {
        let kings = PieceBoard::new_king();
        let queens = PieceBoard::new_queen();
        let bishops = PieceBoard::new_bishop();
        let rooks = PieceBoard::new_rook();
        let knights = PieceBoard::new_knight();
        let pawns = PieceBoard::new_pawn();
        InnerBoard {
            pieces: [pawns, rooks, knights, bishops, kings, queens],
        }
    }
}
impl Index<PieceType> for InnerBoard {
    type Output = PieceBoard;
    fn index(&self, index: PieceType) -> &Self::Output {
        &self
            .get(index as usize)
            .expect("The PieceType number bounds should never be violated, check the enum values")
    }
}
impl IndexMut<PieceType> for InnerBoard {
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        self.pieces
            .get_mut(index as usize)
            .expect("The PieceType number bounds should never be violated, check the enum values")
    }
}
impl Deref for InnerBoard {
    type Target = PieceBoards;
    fn deref(&self) -> &Self::Target {
        &self.pieces
    }
}
type PieceBoards = [PieceBoard; 6];

#[derive(Debug, Clone, Copy)]
pub struct PieceBoard {
    white_piece: BitMap64,
    black_piece: BitMap64,
}
impl PieceBoard {
    fn new_custom(white: u64, black: u64) -> PieceBoard {
        PieceBoard {
            white_piece: BitMap64::new(white),
            black_piece: BitMap64::new(black),
        }
    }
    fn remove_piece_unchecked_with_color(&mut self, pos: &BoardPosition, color: &Color) {
        match color {
            Color::White => &mut self.white_piece,
            Color::Black => &mut self.black_piece,
        }
        .clear_bit(pos.to_num())
    }
    pub fn remove_piece_with_color(
        &mut self,
        pos: &BoardPosition,
        color: &Color,
    ) -> Result<(), BoardError> {
        let bitmap = match color {
            Color::White => &mut self.white_piece,
            Color::Black => &mut self.black_piece,
        };

        bitmap
            .get_bit(pos.to_num())
            .then(|| bitmap.clear_bit(pos.to_num()))
            .ok_or(BoardError::PieceMissing)
    }
    fn insert_unchecked_with_color(&mut self, pos: &BoardPosition, color: &Color) {
        match color {
            Color::White => &mut self.white_piece,
            Color::Black => &mut self.black_piece,
        }
        .set_bit(pos.to_num())
    }
    pub fn insert_piece_with_color(
        &mut self,
        pos: &BoardPosition,
        color: &Color,
    ) -> Result<(), BoardError> {
        let bitmap = match color {
            Color::White => &mut self.white_piece,
            Color::Black => &mut self.black_piece,
        };

        bitmap
            .get_bit(pos.to_num())
            .then(|| bitmap.set_bit(pos.to_num()))
            .ok_or(BoardError::PieceMissing)
    }
    fn move_piece_unchecked(&mut self, color: &Color, from: &BoardPosition, to: &BoardPosition) {
        let pieces = match color {
            Color::White => &mut self.white_piece,
            Color::Black => &mut self.black_piece,
        };
        pieces.clear_bit(from.to_num());
        pieces.set_bit(to.to_num());
    }

    pub fn move_piece(
        &mut self,
        color: &Color,
        from: &BoardPosition,
        to: &BoardPosition,
    ) -> Result<(), BoardError> {
        match (!self.is_square_occupied(from), self.is_square_occupied(to)) {
            (false, false) => {
                self.move_piece_unchecked(color, from, to);
                Ok(())
            }
            (true, _) => Err(BoardError::PieceMissing),
            (_, true) => Err(BoardError::PieceWhereMoving),
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
    pub fn get_movement_options(&self, color: Color) -> Vec<Actions> {
        todo!()
    }
}

#[derive(Debug)]
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

    pub fn perform_actions(&mut self, actions: Actions) -> Result<(), BoardError> {
        self.inner_board.perform_actions(actions)
    }
    pub fn try_from_algebraic_chess_notation(input: &str) -> Result<Self, Error> {
        let input = remove_comments(input)?;
        let mut state = Color::White;
        let mut turns = input.split_whitespace().map(|segment| {
            let innerstate = state.clone();
            state = !state;
            (
                segment
                    .chars()
                    .skip_while(|c| c.is_digit(10) || *c == '.')
                    .collect::<String>(),
                innerstate,
            )
        });
        let mut board = Board::new();
        for (string, turn) in turns {
            board.perform_actions(action_parser(string, turn, &board.inner_board)?)?;
        }

        Ok(board)
    }

    pub fn get_movement_options(&self) -> Vec<Actions> {
        todo!()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
fn remove_comments(input: &str) -> Result<String, ParsingError> {
    let mut cleaned = String::from(input);
    while let Some(start) = cleaned.find('{') {
        match cleaned[start..].find('}') {
            Some(end) => cleaned.replace_range(start..=start + end, ""),
            None => {
                return Err(ParsingError::UnbalancedBraces(ExpectedOneOf::new(
                    vec!['}'],
                    None,
                    Some(input.to_string()),
                )))
            }
        }
    }
    if cleaned.find('}').is_some() {
        return Err(ParsingError::UnbalancedBraces(ExpectedOneOf::new(
            vec!['}'],
            None,
            Some(input.to_string()),
        )));
    }
    Ok(cleaned)
}
/// parses a single string with no space acording to chess anotation
fn action_parser(v: String, color: Color, board: &InnerBoard) -> Result<Actions, Error> {
    let is_taking = v.contains('x');
    if v == "e.p." {
        todo!("enpassant")
    }
    if v == "O-O-O" {
        return Ok(Actions::Castle(CastleAction::Long(color)));
    }
    if v == "O-O" {
        return Ok(Actions::Castle(CastleAction::Short(color)));
    }

    let mut chars = v.chars().peekable();
    let piecetype = match chars.peek().ok_or(ParsingError::CharUnderflow)? {
        'N' => {
            chars.next(); // need to iterate past it
            PieceType::Knight
        }
        'Q' => {
            chars.next(); // need to iterate past it
            PieceType::Queen
        }
        'R' => {
            chars.next(); // need to iterate past it
            PieceType::Rook
        }
        'K' => {
            chars.next(); // need to iterate past it
            PieceType::King
        }
        'B' => {
            chars.next(); // need to iterate past it
            PieceType::Bishop
        }
        _ => PieceType::Pawn, // dont go past it as it will consume the invisible char that reprisents the pawn :(
    };
    let maybe_unambigous_taking =
        chars
            .next()
            .ok_or(ParsingError::ExpectedOneOf(ExpectedOneOf::new(
                vec![
                    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', '1', '2', '3', '4', '5', '6', '7', '8',
                    'x',
                ],
                None,
                Some(v.to_string()),
            )))?;
    let is_unambigous_taking = maybe_unambigous_taking == 'x';
    Ok(match (is_taking, is_unambigous_taking) {
        (true, true) => {
            let next_pos = BoardPosition::from_str(&format!(
                "{}{}",
                chars
                    .next()
                    .ok_or(ParsingError::ExpectedOneOf(ExpectedOneOf::new(
                        vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
                        None,
                        Some(v.to_string()),
                    )))?,
                chars
                    .next()
                    .ok_or(ParsingError::ExpectedOneOf(ExpectedOneOf::new(
                        vec!['1', '2', '3', '4', '5', '6', '7', '8'],
                        None,
                        Some(v.to_string()),
                    )))?
            ))?;
            // we are gona use the bitmasks to know where it was,
            todo!()
        }
        (true, false) => {
            let maybe_easy_ambigous_taking =
                chars
                    .next()
                    .ok_or(ParsingError::ExpectedOneOf(ExpectedOneOf::new(
                        vec![
                            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', '1', '2', '3', '4', '5', '6',
                            '7', '8',
                        ],
                        None,
                        Some(v.to_string()),
                    )))?;
            if maybe_easy_ambigous_taking == 'x' {
                todo!("hard ambigous")
            }
            let from_pos = BoardPosition::from_str(&format!(
                "{}{}",
                maybe_unambigous_taking, maybe_easy_ambigous_taking
            ))?;
            chars
                .next()
                .ok_or(ParsingError::ExpectedOneOf(ExpectedOneOf::new(
                    vec!['x'],
                    None,
                    Some(v.to_string()),
                )))?;
            let to_pos = BoardPosition::from_str(&format!(
                "{}{}",
                chars
                    .next()
                    .ok_or(ParsingError::ExpectedOneOf(ExpectedOneOf::new(
                        vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
                        None,
                        Some(v.to_string()),
                    )))?,
                chars
                    .next()
                    .ok_or(ParsingError::ExpectedOneOf(ExpectedOneOf::new(
                        vec!['1', '2', '3', '4', '5', '6', '7', '8'],
                        None,
                        Some(v.to_string()),
                    )))?
            ))?;
            let taking_piecetype = board
                .what_piece_on_square(&to_pos)
                .ok_or(BoardError::PieceMissing)?;
            let taking_piece_color = board.what_color_is_on_square(&to_pos).expect(
                "The piece existed on a square the line before, somethign has corrupted the board",
            );
            Actions::Take(
                TakeAction::new(taking_piecetype, taking_piece_color, to_pos),
                MoveAction::new(from_pos, to_pos, piecetype, color),
            )
        }
        (false, e) => {
            assert!(!e, "it is not takable but it is an ambigous taing?????");
            todo!()
            //it is not taking atleast, now we need to look into wheather it is ambigous or not
        }
    })
}
