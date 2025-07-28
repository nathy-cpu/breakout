use crate::SCREEN_SIZE;
use raylib::{
    RaylibHandle, RaylibThread,
    audio::{RaylibAudio, Sound},
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
};

pub const PADDLE_WIDTH: i32 = 50;
pub const PADDLE_HEIGHT: i32 = 6;
pub const PADDLE_POS_Y: f32 = 260.0;
pub const PADDLE_SPEED: f32 = 300.0;

pub struct Paddle<'aud> {
    pub position: Vector2,
    pub velocity: f32,
    pub hit_sound: Sound<'aud>,
    pub texture: Texture2D,
}

impl<'aud> Paddle<'aud> {
    pub fn new(
        raylib_handle: &mut RaylibHandle,
        thread: &RaylibThread,
        audio_handle: &'aud RaylibAudio,
    ) -> Self {
        Self {
            position: Vector2 {
                x: (SCREEN_SIZE - PADDLE_WIDTH) as f32 / 2.0, // to start the paddle in the middle
                y: PADDLE_POS_Y,
            },
            velocity: 0.0,
            hit_sound: audio_handle
                .new_sound("assets/hit_paddle.wav")
                .expect("Failed to load sound"),
            texture: raylib_handle
                .load_texture(thread, "assets/paddle.png")
                .expect("Failed to load texture"),
        }
    }

    pub fn reset(&mut self) {
        self.position = Vector2 {
            x: (SCREEN_SIZE - PADDLE_WIDTH) as f32 / 2.0,
            y: PADDLE_POS_Y,
        };
        self.velocity = 0.0;
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_texture_v(&self.texture, self.position, Color::WHITE);
    }
}
