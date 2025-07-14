use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
};

const PADDLE_WIDTH: f32 = 50.0;
const PADDLE_HEIGHT: f32 = 6.0;
const PADDLE_POS_Y: f32 = 260.0;

pub struct Paddle {
    position: Vector2,
    rectangle: Rectangle,
    color: Color,
}

impl Paddle {
    pub fn new() -> Self {
        Self {
            position: Vector2 {
                x: 0.0,
                y: PADDLE_POS_Y,
            },
            rectangle: Rectangle {
                x: 0.0,
                y: PADDLE_POS_Y,
                width: PADDLE_WIDTH,
                height: PADDLE_HEIGHT,
            },
            color: Color::GREEN,
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_rectangle_rec(self.rectangle, self.color);
    }
}
