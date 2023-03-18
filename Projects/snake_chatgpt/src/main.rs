extern crate piston;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop};
use piston::input::{Button, Key};
use piston::window::Window;
use piston::input::RenderArgs;
use piston::input::UpdateArgs;
use rand::Rng;

const BLOCK_SIZE: f64 = 20.0;
const WIDTH: i32 = 30;
const HEIGHT: i32 = 30;

#[derive(Copy, Clone)]
struct Block {
    x: i32,
    y: i32,
}

impl Block {
    fn new(x: i32, y: i32) -> Block {
        Block { x: x, y: y }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: Vec<Block>,
    direction: Direction,
}

impl Snake {
    fn new() -> Snake {
        let mut body = Vec::new();
        body.push(Block::new(10, 10));
        body.push(Block::new(10, 11));
        body.push(Block::new(10, 12));

        Snake {
            body: body,
            direction: Direction::Down,
        }
    }

    fn update(&mut self) {
        let head = self.body[0];
        let mut new_head = match self.direction {
            Direction::Up => Block::new(head.x, head.y - 1),
            Direction::Down => Block::new(head.x, head.y + 1),
            Direction::Left => Block::new(head.x - 1, head.y),
            Direction::Right => Block::new(head.x + 1, head.y),
        };
        self.body.insert(0, new_head);
        self.body.pop();
    }

    fn grow(&mut self) {
        let tail = self.body[self.body.len() - 1];
        let mut new_tail = match self.direction {
            Direction::Up => Block::new(tail.x, tail.y + 1),
            Direction::Down => Block::new(tail.x, tail.y - 1),
            Direction::Left => Block::new(tail.x + 1, tail.y),
            Direction::Right => Block::new(tail.x - 1, tail.y),
        };
        self.body.push(new_tail);
    }

    fn check_collision(&self, x: i32, y: i32) -> bool {
        for block in self.body.iter() {
            if block.x == x && block.y == y {
                return true;
            }
        }
        false
    }

    fn check_eat(&mut self, x: i32, y: i32) -> bool {
        let head = self.body[0];
        if head.x == x && head.y == y {
            self.grow();
            return true;
        }
        false
    }

    fn set_direction(&mut self, direction: Direction) {
        match self.direction {
            Direction::Up => {
                if direction != Direction::Down {
                    self.direction = direction;
                }
            }
            Direction::Down => {
                if direction != Direction::Up {
                    self.direction = direction;
                }
            }
            Direction::Left => {
                if direction != Direction::Right {
                    self.direction = direction;
                }
            }
            Direction::Right => {
                if direction != Direction::Left {
                    self.direction = direction;
                }
            }
        }
    }
    }
       
