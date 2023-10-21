use bevy::prelude::Resource;

use prelude::*;

impl Resource for Board {}
impl Resource for BoardPosition{}


pub mod prelude{
    pub use crate::chess_engine::board::Board;
    pub use crate::chess_engine::board::BoardPosition;
}
