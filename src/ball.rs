use raylib::{
    RaylibHandle, RaylibThread,
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
};

use crate::SCREEN_SIZE;

pub const BALL_START_POS_Y: f32 = 160.0;
pub const BALL_SPEED: f32 = 350.0;
pub const BALL_RADIUS: f32 = 4.0;
pub struct Ball {
    pub position: Vector2,
    pub direction: Vector2,
    pub texture: Texture2D,
}

impl Ball {
    pub fn new(raylib_handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            position: Vector2 {
                x: SCREEN_SIZE as f32 / 2.0,
                y: BALL_START_POS_Y,
            },
            direction: Vector2 { x: 0.0, y: 1.0 },
            texture: raylib_handle
                .load_texture(thread, "assets/ball.png")
                .expect("Failed to load texture"),
        }
    }

    pub fn reset(&mut self) {
        self.direction = Vector2 { x: 0.0, y: 1.0 };
        self.position = Vector2 {
            x: SCREEN_SIZE as f32 / 2.0,
            y: BALL_START_POS_Y,
        };
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_texture_v(
            &self.texture,
            self.position
                - Vector2 {
                    x: BALL_RADIUS,
                    y: BALL_RADIUS,
                },
            Color::WHITE,
        );
    }
}
