use std::collections::LinkedList;

use crate::draw::{draw_block, to_coord, to_coord_u32};
use piston_window::types::Color;
use piston_window::*;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];
const COLOR_BLUE: Color = [52.0, 39.0, 245.0, 0.8];

#[derive(Clone, Copy)]
struct Block {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
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
        body.push_back(Block { x: x - 1, y });

        body.push_back(Block { x: x - 2, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn init_draw(&self, con: &Context, g: &mut G2d) {
        for (num, block) in self.body.iter().enumerate() {
            if num == 0 {
                draw_block(COLOR_BLUE, block.x, block.y, con, g);
            } else {
                draw_block(SNAKE_COLOR, block.x, block.y, con, g);
            }
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    /// 根据方向移动
    pub fn move_forward(&mut self) {
        let (head_x, head_y) = self.head_position();

        let mut new_block = match self.direction {
            Direction::Up => Block {
                x: head_x,
                y: head_y - 1,
            },
            Direction::Down => Block {
                x: head_x,
                y: head_y + 1,
            },
            Direction::Left => Block {
                x: head_x - 1,
                y: head_y,
            },
            Direction::Right => Block {
                x: head_x + 1,
                y: head_y,
            },
        };

        self.body.push_front(new_block);
        self.tail = self.body.pop_back();
    }

    pub fn reset_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn head_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn next_head_position(&self) -> (i32, i32) {
        let (x, y) = self.head_position();
        match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
        }
    }

    pub fn is_overlap_tail(&self) -> bool {
        let (head_x_next, head_y_next) = self.next_head_position();

        for (_i, item) in self.body.iter().enumerate() {
            if item.x == head_x_next && item.y == head_y_next {
                return true;
            }
        }
        false
    }

    pub fn restore_tail(&mut self) {
        let tail = self.tail.clone().unwrap();
        self.body.push_back(tail);
    }
}
