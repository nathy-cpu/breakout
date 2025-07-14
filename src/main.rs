use raylib::prelude::*;

mod paddle;

const SCREEN_SIZE: i32 = 320;

fn main() {
    let (mut ray, thread) = raylib::init()
        .size(1000, 1000)
        .title("Breakout")
        .vsync()
        .build();

    let mut paddle = paddle::Paddle::new();

    while !ray.window_should_close() {
        let (_screen_width, screen_height) = (ray.get_screen_width(), ray.get_screen_height());
        paddle.update(&ray);

        let mut draw_handle = ray.begin_drawing(&thread);

        draw_handle.clear_background(Color::SKYBLUE);
        let _ = draw_handle.begin_mode2D(Camera2D {
            zoom: screen_height as f32 / SCREEN_SIZE as f32,
            offset: Vector2 { x: 0.0, y: 0.0 },
            target: Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
        });
        paddle.draw(&mut draw_handle);
        draw_handle.draw_fps(0, 0);
    }
}
