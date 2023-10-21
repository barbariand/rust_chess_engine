#![feature(allocator_api)]
use chess_engine::board::Board;
mod bevy_app;
pub mod chess_engine;
fn main() {
    bevy_app::run();
}
