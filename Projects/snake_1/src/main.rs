
extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::draw::*;
use crate::game::*;
use crate::snake::*;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {

    let (width, height) = (30, 30);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();



    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _d| {
            clear(BACK_COLOR, g);
            // draw_block(BACK_COLOR, 0, 0, &c, g)
            // game.draw(&c, g);
        });
    }
}
