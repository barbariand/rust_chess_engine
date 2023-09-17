use super::{
    board::{self, Board, BoardPosition, MoveOffset},
    errors::{ActionError, Error},
};
use std::{
    fmt::{Debug, Display},
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
pub enum InnerAction {
    Take,
    MoveTo,
}
pub struct MovementOptions(Vec<Action>);
impl MovementOptions {
    pub fn new<'a>(
        potential_moves: impl IntoIterator<
            Item = PieceMovement<'a, InnerAction>,
            IntoIter = IntoIter<PieceMovement<'a, InnerAction>>,
        >,
        pos: &BoardPosition,
        board: &'a Board,
        piece: &'a Piece,
    ) -> Self {
        Self(
            potential_moves
                .into_iter()
                .map(|movement| BoardWalker::new(pos, board, movement, piece))
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
pub struct PieceMovement<'a, A> {
    step: PieceStep,
    allowed_action: A,
    addon: Option<Box<dyn Fn() -> PieceMovement<'a, NoAction> + 'a>>,
    condition: Option<Box<dyn Fn() -> bool + 'a>>,
}
impl<'a> PieceMovement<'a, NoAction> {
    pub fn new(step: PieceStep) -> Self {
        Self {
            step,
            allowed_action: NoAction::default(),
            addon: None,
            condition: None,
        }
    }
}
impl<'a, A> PieceMovement<'a, A> {
    fn allowed_action(self, action: InnerAction) -> PieceMovement<'a, InnerAction> {
        PieceMovement {
            step: self.step,
            allowed_action: action,
            addon: self.addon,
            condition: self.condition,
        }
    }

    fn addon(mut self, addon: impl Fn() -> PieceMovement<'a, NoAction> + 'a) -> Self {
        self.addon = Some(Box::new(addon));
        self
    }

    fn condition(mut self, condition: impl Fn() -> bool + 'a) -> Self {
        self.condition = Some(Box::new(condition));
        self
    }
}
#[derive(Debug, Clone)]
pub enum PieceStep {
    Fixed(MovementDirection, i8),
    Full(MovementDirection),
}
#[derive(Debug, Clone)]
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
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
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
    movement: PieceMovement<'a, InnerAction>,
}
impl<'a> BoardWalker<'a> {
    fn new(
        pos: &BoardPosition,
        board: &'a Board,
        movement: PieceMovement<'a, InnerAction>,
        piece: &'a Piece,
    ) -> BoardWalker<'a> {
        BoardWalker {
            piece,
            pos: pos.clone(),
            board,
            movement,
        }
    }
}
impl<'a> Iterator for BoardWalker<'a> {
    type Item = Action;
    fn next(&mut self) -> Option<Self::Item> {
        let changer = match &self.movement.step {
            PieceStep::Fixed(direction, len) => todo!("implement walk with fixed range"),
            PieceStep::Full(direction) => match direction {
                MovementDirection::North => MoveOffset(-1, 0),
                MovementDirection::NorthEast => MoveOffset(-1, 1),
                MovementDirection::East => MoveOffset(0, -1),
                MovementDirection::SouthEast => MoveOffset(1, -1),
                MovementDirection::South => MoveOffset(1, 0),
                MovementDirection::SouthWest => MoveOffset(1, 1),
                MovementDirection::West => MoveOffset(0, 1),
                MovementDirection::NorthWest => MoveOffset(-1, -1),
            },
        };

        let new_pos = (self.pos.clone() + changer).ok()?;
        self.pos = new_pos;
        Action::new(self.piece, self.board, self.pos).ok()
    }
}
