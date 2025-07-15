use raylib::{
    RaylibHandle,
    ffi::KeyboardKey,
    math::{Rectangle, Vector2},
    prelude::RaylibDrawHandle,
};

use crate::{
    SCREEN_SIZE,
    ball::{BALL_RADIUS, BALL_SPEED, BALL_START_POS_Y, Ball},
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

    pub fn started(&self) -> bool {
        self.started
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
        // update ball
        let prev_ball_pos = self.ball.position;
        if !self.started {
            self.ball.position.y = BALL_START_POS_Y;
            self.ball.position.x = (SCREEN_SIZE as f32 / 2.0)
                + (raylib_handle.get_time().cos() * SCREEN_SIZE as f64 / 2.5) as f32;
        } else {
            self.ball.position += self.ball.direction * BALL_SPEED * raylib_handle.get_frame_time();
            if self.ball.position.x + BALL_RADIUS > SCREEN_SIZE as f32 {
                self.ball.position.x = SCREEN_SIZE as f32 - BALL_RADIUS;
                reflect(&mut self.ball.direction, Vector2 { x: -1.0, y: 0.0 });
            }
            if self.ball.position.x - BALL_RADIUS < 0.0 {
                self.ball.position.x = BALL_RADIUS;
                reflect(&mut self.ball.direction, Vector2 { x: 1.0, y: 0.0 });
            }
            if self.ball.position.y + BALL_RADIUS > SCREEN_SIZE as f32 {
                self.ball.position.y = SCREEN_SIZE as f32 - BALL_RADIUS;
                reflect(&mut self.ball.direction, Vector2 { x: 0.0, y: -1.0 });
            }
            if self.ball.position.y - BALL_RADIUS < 0.0 {
                self.ball.position.y = BALL_RADIUS;
                reflect(&mut self.ball.direction, Vector2 { x: 0.0, y: 1.0 });
            }
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
            .clamp(0.0, (SCREEN_SIZE - PADDLE_WIDTH) as f32);

        // reflect ball off of the paddle
        let paddle_rect = Rectangle {
            x: self.paddle.position.x,
            y: self.paddle.position.y,
            width: PADDLE_WIDTH as f32,
            height: PADDLE_HEIGHT as f32,
        };
        if paddle_rect.check_collision_circle_rec(self.ball.position, BALL_RADIUS) {
            let mut collision_normal: Vector2 = Default::default();

            if prev_ball_pos.y < paddle_rect.y + paddle_rect.height {
                collision_normal += Vector2 { x: 0.0, y: -1.0 };
                self.ball.position.y = paddle_rect.y - BALL_RADIUS;
            }
            if prev_ball_pos.y > paddle_rect.y + paddle_rect.height {
                collision_normal += Vector2 { x: 0.0, y: 1.0 };
                self.ball.position.y = paddle_rect.y + paddle_rect.height + BALL_RADIUS;
            }
            if prev_ball_pos.x < paddle_rect.x {
                collision_normal += Vector2 { x: -1.0, y: 0.0 };
            }
            if prev_ball_pos.x > paddle_rect.x + paddle_rect.width {
                collision_normal += Vector2 { x: 1.0, y: 0.0 };
            }

            if collision_normal != Vector2::zero() {
                reflect(&mut self.ball.direction, collision_normal.normalized());
            }
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        self.ball.draw(draw_handle);
        self.paddle.draw(draw_handle);
    }
}

// ig i gotta do it myself
fn reflect(vector: &mut Vector2, normal: Vector2) {
    let dot_product = vector.dot(normal);
    *vector = Vector2 {
        x: vector.x - (2.0 * normal.x) * dot_product,
        y: vector.y - (2.0 * normal.y) * dot_product,
    };
    vector.normalize();
}
