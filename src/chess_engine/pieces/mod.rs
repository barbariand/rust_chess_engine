use super::{
    board::{self, Board, BoardPosition, MoveOffset},
    errors::{ActionError, Error},
};
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::Deref,
    vec::IntoIter,
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
pub mod tables {
    pub mod innertable {
        include!(concat!(env!("OUT_DIR"), "/movetables.rs"));
    }
    pub use innertable::king_moves_bitmask as king_movetable;
    pub use innertable::rook_moves_bitmask as rook_movetable;
}
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
                todo!("not implemented")
            }
            InnerAction::Take => {
                todo!("not implemented")
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum InnerAction {
    Take,
    MoveTo,
}
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
}
impl Deref for MovementOptions {
    type Target = Vec<Action>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
trait MovablePiece {
    fn get_movement_options<'a>(
        pos: &BoardPosition,
        board: &'a Board,
        piece: &'a Piece,
    ) -> MovementOptions;
}
#[derive(Default)]
struct NoAction;
#[derive(Default, Clone, Copy)]
struct NoCondition;
trait PieceMovementCondition<'a> {}
struct Condition<'a>(Box<dyn Fn() -> bool + 'a>);
pub struct PieceMovement<A, C> {
    step: PieceStep,
    allowed_action: A,
    addon: Option<Box<PieceMovement<NoAction, NoCondition>>>,
    condition: C,
}
impl PieceMovement<NoAction, NoCondition> {
    pub fn new(step: PieceStep) -> Self {
        Self {
            step,
            allowed_action: NoAction::default(),
            addon: None,
            condition: NoCondition::default(),
        }
    }
}
impl<'a, A, C> PieceMovement<A, C> {
    fn allowed_action(self, action: InnerAction) -> PieceMovement<InnerAction, C> {
        PieceMovement {
            step: self.step,
            allowed_action: action,
            addon: self.addon,
            condition: self.condition,
        }
    }

    fn addon(mut self, addon: PieceMovement<NoAction, NoCondition>) -> Self {
        self.addon = Some(Box::new(addon));
        self
    }

    fn condition(self, condition: impl Fn() -> bool + 'a) -> PieceMovement<A, Condition<'a>> {
        PieceMovement {
            step: self.step,
            allowed_action: self.allowed_action,
            addon: self.addon,
            condition: Condition(Box::new(condition)),
        }
    }
}
impl<'a, C> PieceMovement<InnerAction, C> {
    fn get_allowed_action(&self) -> &InnerAction {
        &self.allowed_action
    }
}
impl Into<Vec<PieceStep>> for PieceMovement<NoAction, NoCondition> {
    fn into(self) -> Vec<PieceStep> {
        let mut action_vec: Vec<PieceStep> = Vec::new();
        let mut traversed = false;
        let mut last = self;

        while !traversed {
            let current = match last.addon {
                Some(func) => *func,
                None => {
                    traversed = true;
                    continue;
                }
            };
            action_vec.push(last.step);
            last = current;
        }

        action_vec
    }
}
impl<C> From<PieceMovement<InnerAction, C>> for PieceMovement<NoAction, NoCondition> {
    fn from(value: PieceMovement<InnerAction, C>) -> Self {
        Self {
            step: value.step,
            allowed_action: NoAction::default(),
            addon: value.addon,
            condition: NoCondition::default(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum PieceStep {
    Fixed(MovementDirection, i8),
    Full(MovementDirection),
}
#[derive(Debug, Clone, Copy)]
pub enum MovementDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
impl Into<MoveOffset> for &MovementDirection {
    fn into(self) -> MoveOffset {
        match self {
            MovementDirection::North => MoveOffset(-1, 0),
            MovementDirection::NorthEast => MoveOffset(-1, 1),
            MovementDirection::East => MoveOffset(0, -1),
            MovementDirection::SouthEast => MoveOffset(1, -1),
            MovementDirection::South => MoveOffset(1, 0),
            MovementDirection::SouthWest => MoveOffset(1, 1),
            MovementDirection::West => MoveOffset(0, 1),
            MovementDirection::NorthWest => MoveOffset(-1, -1),
        }
    }
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
    type_of_piece: InnerPiece,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum InnerPiece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

impl Piece {
    pub fn new(color: Color, type_of_piece: InnerPiece) -> Piece {
        Piece {
            color,
            pos: None,
            type_of_piece,
        }
    }
}

impl Piece {
    pub fn get_movement_options(&self, board: &Board) -> Option<MovementOptions>
    where
        Self: Sized,
    {
        self.pos.map(|pos| match self.type_of_piece {
            InnerPiece::Bishop => Bishop::get_movement_options(&pos, board, self),
            InnerPiece::King => King::get_movement_options(&pos, board, self),
            InnerPiece::Knight => Knight::get_movement_options(&pos, board, self),
            InnerPiece::Pawn => Pawn::get_movement_options(&pos, board, self),
            InnerPiece::Queen => Queen::get_movement_options(&pos, board, self),
            InnerPiece::Rook => Rook::get_movement_options(&pos, board, self),
        })
    }
    pub fn try_from_bitmap(pos: BoardPosition, mut piece_type: i64) -> Option<Piece> {
        let is_black = piece_type > 0b100000;
        if is_black {
            piece_type = piece_type >> 6
        }
        Some(Self {
            color: match is_black {
                false => Color::White,
                true => Color::Black,
            },
            pos: Some(pos),
            type_of_piece: match piece_type {
                0b1 => InnerPiece::Pawn,
                0b1 => InnerPiece::Rook,
                0b1 => InnerPiece::Knight,
                0b1 => InnerPiece::Bishop,
                0b1 => InnerPiece::King,
                0b1 => InnerPiece::Queen,
                e => return None,
            },
        })
    }
}
impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.type_of_piece {
            InnerPiece::Bishop => write!(f, " B "),
            InnerPiece::King => write!(f, " K "),
            InnerPiece::Knight => write!(f, " Kn"),
            InnerPiece::Pawn => write!(f, " p "),
            InnerPiece::Queen => write!(f, " Q "),
            InnerPiece::Rook => write!(f, " R "),
        }
    }
}

pub(super) struct BoardWalker<'a> {
    piece: &'a Piece,
    pos: BoardPosition,
    board: &'a Board,
    steps: Vec<PieceStep>,
}
impl<'a> BoardWalker<'a> {
    fn new(
        pos: &BoardPosition,
        board: &'a Board,
        step: Vec<PieceStep>,
        piece: &'a Piece,
    ) -> BoardWalker<'a> {
        BoardWalker {
            piece,
            pos: pos.clone(),
            board,
            steps: step,
        }
    }
}
impl<'a> Iterator for BoardWalker<'a> {
    type Item = Action;
    fn next(&mut self) -> Option<Self::Item> {
        for step in (&self.steps).into_iter() {
            let changer: MoveOffset = match step {
                PieceStep::Fixed(direction, len) => {
                    <&MovementDirection as Into<MoveOffset>>::into(direction) * len
                }
                PieceStep::Full(direction) => direction.into(),
            };

            let new_pos = (self.pos.clone() + changer).ok()?;
            self.pos = new_pos;
        }
        Action::new(self.piece, self.board, self.pos).ok()
    }
}
