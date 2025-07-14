use raylib::prelude::*;

mod paddle;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 1000;

fn main() {
    let (mut ray, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Breakout")
        .vsync()
        .build();

    let paddle = paddle::Paddle::new();

    while !ray.window_should_close() {
        let mut draw_handle = ray.begin_drawing(&thread);

        draw_handle.clear_background(Color::SKYBLUE);
        paddle.draw(&mut draw_handle);
    }
}
