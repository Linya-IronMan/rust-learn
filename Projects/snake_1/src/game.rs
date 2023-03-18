use crate::draw::*;
use crate::snake::{Direction, Snake};
use piston_window::types::Color;
use piston_window::*;
use rand::{self, Rng};

const FOOD_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    game_over: bool,

    snake: Snake,
    /// 用来控制刷新率的
    waiting_time: f64,

    limit_x_min: i32,
    limit_x_max: i32,
    limit_y_min: i32,
    limit_y_max: i32,

    map_size: (i32, i32),

    is_food_exist: bool,
    food_position: Option<(i32, i32)>,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Game {
            game_over: false,
            snake: Snake::new(10, 2),
            waiting_time: 0.0,
            limit_x_min: 1,
            limit_x_max: width - 1,
            limit_y_min: 1,
            limit_y_max: height - 1,
            map_size: (width, height),
            is_food_exist: false,
            food_position: None,
        }
    }

    pub fn draw(&mut self, con: &Context, g: &mut G2d) {
        self.snake.init_draw(con, g);
        self.check_eating();
        if (self.is_food_exist && !self.food_position.is_none()) {
            self.draw_food_by(self.food_position.unwrap(), con, g);
        } else {
            self.set_new_food(con, g);
        }
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.map_size.0, self.map_size.1, con, g)
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.check_alive();
        if self.game_over {
            return;
        }
        self.waiting_time += delta_time;

        if self.waiting_time > MOVING_PERIOD {
            self.snake.move_forward();
            self.waiting_time = 0.0;
            println!("Over Moving Period");
        }
    }

    pub fn check_alive(&mut self) {
        // 撞墙检测
        if let (x, y) = self.snake.head_position() {
            if x <= self.limit_x_min
                || x >= self.limit_x_max
                || y <= self.limit_y_min
                || y >= self.limit_y_max
            {
                // 游戏结束
                self.game_over = true;
                println!("Game is Over");
            }
        }

        if self.snake.is_overlap_tail() {
            self.game_over = true;
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        let direction = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        }
        .unwrap();

        if direction != self.snake.head_direction().opposite() {
            self.snake.reset_direction(direction);
        }
    }

    pub fn draw_food_by(&mut self, food_position: (i32, i32), con: &Context, g: &mut G2d) {
        let (x, y) = food_position;
        draw_block(FOOD_COLOR, x, y, con, g);
        self.is_food_exist = true;
        self.food_position = Some((x, y));
    }

    pub fn set_new_food(&mut self, con: &Context, g: &mut G2d) {
        println!("set food fn");
        let mut rng = rand::thread_rng();
        let (width, height) = self.map_size;
        let x = rng.gen_range(2..width - 2);
        let y = rng.gen_range(2..height - 2);
        self.draw_food_by((x, y), con, g);
    }

    pub fn check_eating(&mut self) {
        match self.food_position {
            Some(position) => {
                if position == self.snake.head_position() {
                    println!("Food is Eating");
                    self.is_food_exist = false;
                    // 尾部追加元素
                    self.snake.restore_tail();
                }
            }
            None => (),
        }
    }
}
