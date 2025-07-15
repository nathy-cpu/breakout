use raylib::{RaylibHandle, ffi::KeyboardKey, math::Vector2, prelude::RaylibDrawHandle};

use crate::{
    SCREEN_SIZE,
    ball::{BALL_SPEED, BALL_START_POS_Y, Ball},
    paddle::{PADDLE_HEIGHT, PADDLE_POS_Y, PADDLE_SPEED, PADDLE_WIDTH, Paddle},
};

pub struct GameState {
    started: bool,
    ball: Ball,
    paddle: Paddle,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            started: false,
            ball: Ball::new(),
            paddle: Paddle::new(),
        }
    }

    pub fn start(&mut self) {
        self.started = true;
        let paddle_middle = Vector2 {
            x: self.paddle.position.x + (PADDLE_WIDTH as f32 / 2.0),
            y: PADDLE_POS_Y,
        };
        let ball_to_paddle = paddle_middle - self.ball.position;
        self.ball.direction = ball_to_paddle.normalized();
    }

    pub fn update(&mut self, raylib_handle: &RaylibHandle) {
        if !self.started {
            self.ball.position.y = BALL_START_POS_Y;
            self.ball.position.x = (SCREEN_SIZE as f32 / 2.0)
                + (raylib_handle.get_time().cos() * SCREEN_SIZE as f64 / 2.5) as f32;
        } else {
            self.ball.position += self.ball.direction * BALL_SPEED * raylib_handle.get_frame_time();
        }

        // update paddle
        self.paddle.velocity = 0.0; // this avoids the paddle accelerating
        if raylib_handle.is_key_down(KeyboardKey::KEY_LEFT) {
            self.paddle.velocity -= PADDLE_SPEED;
        }
        if raylib_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.paddle.velocity += PADDLE_SPEED;
        }
        self.paddle.position.x += self.paddle.velocity * raylib_handle.get_frame_time();
        self.paddle.position.x = self
            .paddle
            .position
            .x
            .clamp(0.0, (SCREEN_SIZE - PADDLE_HEIGHT) as f32);
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        self.ball.draw(draw_handle);
        self.paddle.draw(draw_handle);
    }
}
