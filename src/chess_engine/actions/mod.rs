use crate::error::ActionError;

trait Action {
    pub fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError>;
}

struct MoveAction {
    from: BoardPosition,
    to: BoardPosition,
    piece: PieceType,
    piece_color: Color,
}
impl Action for MoveAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError> {}
}

struct TakeAction {
    take_piece: PieceType,
    take_piece_color: Color,
}
impl Action for TakeAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError> {
        todo!()
    }
}

struct PromoteAction {
    become_piece: PieceType,
    become_piece_color: Color,
}
impl Action for PromoteAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError> {
        todo!()
    }
}

enum CastleAction {
    Right,
    Left,
}
impl Action for CastleAction {
    fn execute(self, board: &mut InnerBoard) -> Result<(), ActionError> {
        todo!()
    }
}
