use raylib::prelude::*;

mod ball;
mod blocks;
mod game;
mod paddle;

use crate::game::GameState;

const SCREEN_SIZE: i32 = 320;
const WINDOW_SIZE: i32 = 1000;

fn main() {
    let (mut ray, thread) = raylib::init()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .title("Breakout")
        .vsync()
        .build();
    ray.set_target_fps(500);

    let mut game = GameState::new();

    let camera = Camera2D {
        zoom: WINDOW_SIZE as f32 / SCREEN_SIZE as f32, // Zoom in to fit canvas
        offset: Vector2 { x: 0.0, y: 0.0 },
        target: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
    };

    while !ray.window_should_close() {
        if !game.started() && ray.is_key_pressed(KeyboardKey::KEY_SPACE) {
            game.start();
        }
        game.update(&ray);
        let mut draw_handle = ray.begin_drawing(&thread);
        draw_handle.clear_background(Color::SKYBLUE);

        let mut mode2d = draw_handle.begin_mode2D(camera);
        game.draw(&mut mode2d);
    }
}
