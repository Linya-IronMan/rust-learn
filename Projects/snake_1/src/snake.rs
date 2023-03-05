use std::collections::LinkedList;

use piston_window::*;

struct Block {
    x: i32,
    y: i32,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(&self, x: i32, y: i32) -> Self {
        let mut body = LinkedList::new();

        body.push_back(Block { x, y });
        body.push_back(Block { x: x + 1, y });

        body.push_back(Block { x: x + 2, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn init_draw(&self, con: &Context, g: &mut G2d) {
        // self.body
    }
}
