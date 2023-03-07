use std::collections::LinkedList;

use crate::draw::{draw_block, to_coord, to_coord_u32};
use piston_window::types::Color;
use piston_window::*;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

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
    pub fn new(x: i32, y: i32) -> Self {
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
        for block in self.body.iter() {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }
}
