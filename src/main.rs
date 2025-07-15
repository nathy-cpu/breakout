use raylib::prelude::*;

use crate::game::GameState;

mod ball;
mod game;
mod paddle;

const SCREEN_SIZE: i32 = 320;
const WINDOW_SIZE: i32 = 1000;

fn main() {
    let (mut ray, thread) = raylib::init()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .title("Breakout")
        .vsync()
        .build();

    let mut game = GameState::new();

    let camera = Camera2D {
        zoom: WINDOW_SIZE as f32 / SCREEN_SIZE as f32, // Zoom in to fit canvas
        offset: Vector2 { x: 0.0, y: 0.0 },
        target: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
    };

    while !ray.window_should_close() {
        if ray.is_key_down(KeyboardKey::KEY_SPACE) {
            game.start();
        }
        game.update(&ray);
        let mut draw_handle = ray.begin_drawing(&thread);
        draw_handle.clear_background(Color::SKYBLUE);

        let mut mode2d = draw_handle.begin_mode2D(camera);
        game.draw(&mut mode2d);
    }
}
