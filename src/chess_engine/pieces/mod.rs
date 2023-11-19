use std::iter::Step;
pub mod tables {
    use crate::chess_engine::board::bitmap::BitMap64;
    use crate::chess_engine::pieces::Color;
    use crate::chess_engine::pieces::PieceType;
    use std::ops::Deref;
    use std::ops::Index;
    use std::ops::IndexMut;
    mod innertable {
        use crate::chess_engine::board::bitmap::BitMap64;
        include!(concat!(env!("OUT_DIR"), "/movetables.rs"));
    }
    use innertable::bishop_moves_bitmask as bishop_movetable;
    use innertable::black_pawn_moves as black_pawn_movetable;
    use innertable::king_moves_bitmask as king_movetable;
    use innertable::knight_moves_bitmask as knight_movetable;
    use innertable::queen_moves_bitmask as queen_movetable;
    use innertable::rook_moves_bitmask as rook_movetable;
    use innertable::white_pawn_moves as white_pawn_movetable;
    pub struct MoveTable([(BitMap64, BitMap64); 64]);
    impl Deref for MoveTable {
        type Target = [(BitMap64, BitMap64); 64];
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    pub struct MoveTables {
        tables: [MoveTable; 7],
    }
    pub static MOVETABLES: MoveTables = MoveTables {
        tables: [
            MoveTable(white_pawn_movetable),
            MoveTable(black_pawn_movetable),
            MoveTable(rook_movetable),
            MoveTable(knight_movetable),
            MoveTable(bishop_movetable),
            MoveTable(king_movetable),
            MoveTable(queen_movetable),
        ],
    };
    impl Index<(PieceType, Color)> for MoveTables {
        type Output = MoveTable;
        fn index(&self, index: (PieceType, Color)) -> &Self::Output {
            match index {
                (PieceType::Pawn, Color::White) => &self.tables[0],
                (PieceType::Pawn, Color::Black) => &self.tables[1],
                (PieceType::Rook, _) => &self.tables[2],
                (PieceType::Knight, _) => &self.tables[3],
                (PieceType::Bishop, _) => &self.tables[4],
                (PieceType::King, _) => &self.tables[5],
                (PieceType::Queen, _) => &self.tables[6],
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}
impl std::ops::Not for Color {
    type Output = Color;
    fn not(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy, PartialOrd)]
pub enum PieceType {
    Queen = 5,
    King = 4,
    Bishop = 3,
    Knight = 2,
    Rook = 1,
    Pawn = 0,
}
impl Step for PieceType {
    fn steps_between(rhs: &Self, lhs: &Self) -> Option<usize> {
        if (*rhs as usize) > (*lhs as usize) {
            None
        } else {
            Some((*rhs as usize) - (*lhs as usize))
        }
    }

    fn forward_checked(piece: Self, step: usize) -> Option<Self> {
        let next_piece = (piece as usize) + step;
        match next_piece {
            0 => Some(PieceType::Pawn),
            1 => Some(PieceType::Rook),
            2 => Some(PieceType::Knight),
            3 => Some(PieceType::Bishop),
            4 => Some(PieceType::King),
            5 => Some(PieceType::Queen),
            _ => None, // No piece is higher than Queen
        }
    }

    fn backward_checked(piece: Self, step: usize) -> Option<Self> {
        if step > piece as usize {
            None
        } else {
            let prev_piece = (piece as usize) - step;
            match prev_piece {
                0 => Some(PieceType::Pawn),
                1 => Some(PieceType::Rook),
                2 => Some(PieceType::Knight),
                3 => Some(PieceType::Bishop),
                4 => Some(PieceType::King),
                5 => Some(PieceType::Queen),
                _ => unreachable!(), // We already checked for underflow
            }
        }
    }
}
