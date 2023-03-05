use crate::draw::*;
use crate::snake::{Snake};

struct Game {
    game_over: bool,

    snake: Snake,
}

impl Game {
    pub fn new(&self) -> Self {
        Game { game_over: false, snake: Snake{

        } }
    }
}
