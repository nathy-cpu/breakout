use raylib::{
    RaylibHandle,
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::SCREEN_SIZE;

const BALL_START_POS_Y: f32 = 160.0;
const BALL_SPEED: f32 = 250.0;
pub struct Ball {
    position: Vector2,
    direction: Vector2,
    radius: f32,
    color: Color,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            position: Vector2 {
                x: SCREEN_SIZE as f32 / 2.0,
                y: BALL_START_POS_Y,
            },
            direction: Vector2 { x: 0.0, y: 1.0 },
            radius: 4.0,
            color: Color::BLACK,
        }
    }

    pub fn update(&mut self, raylib_handle: &RaylibHandle) {
        self.position += self.direction * BALL_SPEED * raylib_handle.get_frame_time();
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            self.radius,
            self.color,
        );
    }
}
