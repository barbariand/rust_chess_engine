#![feature(allocator_api)]
use chess_engine::board::Board;

mod chess_engine;

fn main() {
    let board=Board::new();
    println!("{}",board)
}
