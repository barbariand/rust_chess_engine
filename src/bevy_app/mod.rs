use bevy::{ecs::component::TableStorage, prelude::*};
mod chess_engine_glue;
use crate::chess_engine::board::Board;

pub fn run() {
    App::new().init_resource::<Board>();
}
#[derive(Default, Debug, Hash, PartialEq, PartialOrd, Eq, Clone)]
enum GameStates {
    #[default]
    WhitesTurn,
    BlacksTurn,
    GameOver(Color),
}
impl States for crate::chess_engine::pieces::Color {
    type Iter;

    fn variants() -> Self::Iter {
        todo!()
    }
}
fn hello_world() {
    println!("hello world!");
}
