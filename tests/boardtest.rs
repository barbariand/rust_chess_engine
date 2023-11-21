use rust_chess_engine::{self, chess_engine::board::Board};
#[test]
fn validate_standard_board() {
    let board = Board::new();
    assert!(board.validate_current_board())
}
