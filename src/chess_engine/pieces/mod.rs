pub mod tables {
    mod innertable {
        use crate::chess_engine::board::bitmap::BitMap64;
        include!(concat!(env!("OUT_DIR"), "/movetables.rs"));
    }
    pub use innertable::bishop_moves_bitmask as bishop_movetable;
    pub use innertable::king_moves_bitmask as king_movetable;
    pub use innertable::knight_moves_bitmask as knight_movetable;
    pub use innertable::queen_moves_bitmask as queen_movetable;
    pub use innertable::rook_moves_bitmask as rook_movetable;
    pub use innertable::white_pawn_moves as white_pawn_movetable;
}
/*
pub struct MovementOptions(Vec<Action>);
impl MovementOptions {
    pub fn new<'a, C>(
        potential_moves: impl IntoIterator<
            Item = PieceMovement<InnerAction, C>,
            IntoIter = IntoIter<PieceMovement<InnerAction, C>>,
        >,
        pos: &BoardPosition,
        board: &'a Board,
        piece: &'a Piece,
    ) -> Self {
        Self(
            potential_moves
                .into_iter()
                .map(|movement| {
                    let allowed_action = &movement.allowed_action;
                    let condition = &movement.condition;
                    BoardWalker::new(
                        pos,
                        board,
                        PieceMovement::<NoAction, NoCondition>::from(movement).into(),
                        piece,
                    )
                })
                .flatten()
                .collect(),
        )
    }
} */
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
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PieceType {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    King = 4,
    Queen = 5,
}
