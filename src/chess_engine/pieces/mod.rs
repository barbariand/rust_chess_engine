pub mod tables {
    mod innertable {
        include!(concat!(env!("OUT_DIR"), "/movetables.rs"));
    }
    pub use innertable::king_moves_bitmask as king_movetable;
    pub use innertable::rook_moves_bitmask as rook_movetable;
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
#[derive(Debug,Clone,Copy)]
pub enum Color {
    Black,
    White,
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
