use raylib::{
    color::Color,
    ffi::KeyboardKey,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibHandle},
};

use crate::SCREEN_SIZE;

const PADDLE_WIDTH: i32 = 50;
const PADDLE_HEIGHT: i32 = 6;
const PADDLE_POS_Y: f32 = 260.0;
const PADDLE_SPEED: f32 = 200.0;

pub struct Paddle {
    position: Vector2,
    color: Color,
    velocity: f32,
}

impl Paddle {
    pub fn new() -> Self {
        Self {
            position: Vector2 {
                x: (SCREEN_SIZE - PADDLE_WIDTH) as f32 / 2.0, // to start the paddle in the middle
                y: PADDLE_POS_Y,
            },
            color: Color::GREEN,
            velocity: 0.0,
        }
    }

    pub fn update(&mut self, raylib_handle: &RaylibHandle) {
        self.velocity = 0.0; // this avoids the paddle accelerating
        if raylib_handle.is_key_down(KeyboardKey::KEY_LEFT) {
            self.velocity -= PADDLE_SPEED;
        }
        if raylib_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.velocity += PADDLE_SPEED;
        }
        self.position.x += self.velocity * raylib_handle.get_frame_time();
        self.position.x = self
            .position
            .x
            .clamp(0.0, (SCREEN_SIZE - PADDLE_WIDTH) as f32);
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
