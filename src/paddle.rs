use crate::SCREEN_SIZE;
use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

pub const PADDLE_WIDTH: i32 = 50;
pub const PADDLE_HEIGHT: i32 = 6;
pub const PADDLE_POS_Y: f32 = 260.0;
pub const PADDLE_SPEED: f32 = 300.0;

pub struct Paddle {
    pub position: Vector2,
    pub color: Color,
    pub velocity: f32,
}

impl Paddle {
    pub fn new() -> Self {
        Self {
            position: Vector2 {
                x: (SCREEN_SIZE - PADDLE_WIDTH) as f32 / 2.0, // to start the paddle in the middle
                y: PADDLE_POS_Y,
            },
            color: Color::BLACK,
            velocity: 0.0,
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_rectangle(
            self.position.x as i32,
            self.position.y as i32,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            self.color,
        );
    }
}
