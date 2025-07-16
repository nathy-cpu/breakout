use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::SCREEN_SIZE;

pub const BALL_START_POS_Y: f32 = 160.0;
pub const BALL_SPEED: f32 = 350.0;
pub const BALL_RADIUS: f32 = 4.0;
pub struct Ball {
    pub position: Vector2,
    pub direction: Vector2,
    pub color: Color,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            position: Vector2 {
                x: SCREEN_SIZE as f32 / 2.0,
                y: BALL_START_POS_Y,
            },
            direction: Vector2 { x: 0.0, y: 1.0 },
            color: Color::RED,
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            BALL_RADIUS,
            self.color,
        );
    }
}
