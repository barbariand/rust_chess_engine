use super::{BoardPosition, MovementOptions, PieceMovement};
#[derive(Debug, Clone)]
pub struct Queen;
impl PieceMovement for Queen {
    fn get_movement_options(
        pos: BoardPosition,
        board: crate::chess_engine::board::Board,
        color: &super::Color,
    ) -> MovementOptions
    where
        Self: Sized,
    {
        MovementOptions {
            0: potential_moves
                .iter()
                .map(|v| BoardWalker::new(&pos, &board, MoveOffset(v.0, v.1)))
                .map(|walker| walker.collect::<Vec<BoardPosition>>())
                .reduce(|v, mut acc| {
                    acc.extend(v);
                    acc
                })
                .expect("iterator cant be zero beacuse we start it with 4")
                .into_iter()
                .map(|pos| match board.has_piece(&pos) {
                    true => Action::Take(pos),
                    false => Action::MoveTo(pos),
                })
                .collect(),
        }
    }
}
