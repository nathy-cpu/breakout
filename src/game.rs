use raylib::{
    RaylibHandle,
    audio::{RaylibAudio, Sound},
    color::Color,
    ffi::KeyboardKey,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    SCREEN_SIZE,
    ball::{BALL_RADIUS, BALL_SPEED, BALL_START_POS_Y, Ball},
    blocks::{BLOCK_HEIGHT, BLOCK_WIDTH, Blocks, NUM_BLOCKS_X, NUM_BLOCKS_Y},
    paddle::{PADDLE_HEIGHT, PADDLE_POS_Y, PADDLE_SPEED, PADDLE_WIDTH, Paddle},
};

const DELTA_TIME: f32 = 1.0 / 60.0; // 16 ms or 0.016 s

pub struct GameState<'aud> {
    started: bool,
    game_over: bool,
    score: i32,
    accumilated_time: f32,
    ball: Ball,
    paddle: Paddle<'aud>,
    blocks: Blocks<'aud>,
    game_over_sound: Sound<'aud>,
}

impl<'aud> GameState<'aud> {
    pub fn new(audio_handle: &'aud RaylibAudio) -> Self {
        Self {
            started: false,
            game_over: false,
            score: 0,
            accumilated_time: 0.0,
            ball: Ball::new(),
            paddle: Paddle::new(audio_handle),
            blocks: Blocks::new(audio_handle),
            game_over_sound: audio_handle
                .new_sound("assets/game_over.wav")
                .expect("Failed to load sound"),
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

    pub fn restart(&mut self) {
        self.started = false;
        self.game_over = false;
        self.score = 0;
        self.ball.reset();
        self.paddle.reset();
        self.blocks.reset();
    }

    pub fn update(&mut self, raylib_handle: &RaylibHandle) {
        // check state
        if !self.started {
            self.ball.position.y = BALL_START_POS_Y;
            self.ball.position.x = (SCREEN_SIZE as f32 / 2.0)
                + (raylib_handle.get_time().cos() * SCREEN_SIZE as f64 / 2.5) as f32;

            if raylib_handle.is_key_pressed(KeyboardKey::KEY_SPACE) {
                self.start();
            }
        } else if self.game_over {
            if raylib_handle.is_key_pressed(KeyboardKey::KEY_SPACE) {
                self.restart();
            }
        } else {
            self.accumilated_time += raylib_handle.get_time() as f32;
        }

        // now we can update everything with fixed timestamp
        if self.accumilated_time >= DELTA_TIME {
            // update ball
            let prev_ball_pos = self.ball.position;
            self.ball.position += self.ball.direction * BALL_SPEED * DELTA_TIME;

            if self.ball.position.x + BALL_RADIUS > SCREEN_SIZE as f32 {
                self.ball.position.x = SCREEN_SIZE as f32 - BALL_RADIUS;
                reflect(&mut self.ball.direction, Vector2 { x: -1.0, y: 0.0 });
            }
            if self.ball.position.x - BALL_RADIUS < 0.0 {
                self.ball.position.x = BALL_RADIUS;
                reflect(&mut self.ball.direction, Vector2 { x: 1.0, y: 0.0 });
            }
            if self.ball.position.y - BALL_RADIUS < 0.0 {
                self.ball.position.y = BALL_RADIUS;
                reflect(&mut self.ball.direction, Vector2 { x: 0.0, y: 1.0 });
            }
            if !self.game_over && self.ball.position.y > SCREEN_SIZE as f32 + BALL_RADIUS * 9.0 {
                self.game_over = true;
                self.game_over_sound.play();
            }
            // update paddle
            self.paddle.velocity = 0.0; // this avoids the paddle accelerating
            if raylib_handle.is_key_down(KeyboardKey::KEY_LEFT) {
                self.paddle.velocity -= PADDLE_SPEED;
            }
            if raylib_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
                self.paddle.velocity += PADDLE_SPEED;
            }
            self.paddle.position.x += self.paddle.velocity * DELTA_TIME;
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

                self.paddle.hit_sound.play();
            }

            // update blocks
            for y in 0..NUM_BLOCKS_Y {
                'outer: for x in 0..NUM_BLOCKS_X {
                    if self.blocks.grid[y][x] {
                        let block_rect = Rectangle {
                            x: 20.0 + x as f32 * BLOCK_WIDTH,
                            y: 40.0 + y as f32 * BLOCK_HEIGHT,
                            width: BLOCK_WIDTH,
                            height: BLOCK_HEIGHT,
                        };
                        if block_rect.check_collision_circle_rec(self.ball.position, BALL_RADIUS) {
                            let block_left = block_rect.x;
                            let block_right = block_rect.x + block_rect.width;
                            let block_top = block_rect.y;
                            let block_bottom = block_rect.y + block_rect.height;

                            let overlap_left = (self.ball.position.x + BALL_RADIUS) - block_left;
                            let overlap_right = block_right - (self.ball.position.x - BALL_RADIUS);
                            let overlap_top = (self.ball.position.y + BALL_RADIUS) - block_top;
                            let overlap_bottom =
                                block_bottom - (self.ball.position.y - BALL_RADIUS);

                            let min_overlap = overlap_left
                                .min(overlap_right)
                                .min(overlap_top)
                                .min(overlap_bottom);

                            let mut collision_normal = Vector2::zero();
                            if min_overlap == overlap_left {
                                collision_normal = Vector2 { x: -1.0, y: 0.0 };
                                self.ball.position.x = block_left - BALL_RADIUS;
                            } else if min_overlap == overlap_right {
                                collision_normal = Vector2 { x: 1.0, y: 0.0 };
                                self.ball.position.x = block_right + BALL_RADIUS;
                            } else if min_overlap == overlap_top {
                                collision_normal = Vector2 { x: 0.0, y: -1.0 };
                                self.ball.position.y = block_top - BALL_RADIUS;
                            } else if min_overlap == overlap_bottom {
                                collision_normal = Vector2 { x: 0.0, y: 1.0 };
                                self.ball.position.y = block_bottom + BALL_RADIUS;
                            }

                            if collision_normal != Vector2::zero() {
                                reflect(&mut self.ball.direction, collision_normal);
                            }
                            self.blocks.grid[y][x] = false;
                            self.score += self.blocks.row_scores[y];
                            self.blocks.hit_sound.play();
                            break 'outer;
                        }
                    }
                }
            }
            self.accumilated_time -= DELTA_TIME;
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        self.ball.draw(draw_handle);
        self.paddle.draw(draw_handle);
        self.blocks.draw(draw_handle);
        if self.game_over {
            let message = format!("Score: {}. press space to reset", self.score);
            let message_width = draw_handle.measure_text(&message, 15);
            draw_handle.draw_text(
                &message,
                SCREEN_SIZE / 2 - message_width / 2,
                BALL_START_POS_Y as i32 - 30,
                15,
                Color::WHITE,
            );
        } else if !self.started {
            let message = format!("Press space to start");
            let message_width = draw_handle.measure_text(&message, 15);
            draw_handle.draw_text(
                &message,
                SCREEN_SIZE / 2 - message_width / 2,
                BALL_START_POS_Y as i32 - 30,
                15,
                Color::WHITE,
            );
        } else {
            let score = format!("{}", self.score);
            draw_handle.draw_text(&score, 5, 5, 10, Color::WHITE);
        }
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
