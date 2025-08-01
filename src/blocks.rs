use raylib::{
    audio::{RaylibAudio, Sound},
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
};

pub const NUM_BLOCKS_X: usize = 10;
pub const NUM_BLOCKS_Y: usize = 8;
pub const BLOCK_WIDTH: f32 = 28.0;
pub const BLOCK_HEIGHT: f32 = 10.0;

pub struct Blocks<'aud> {
    pub grid: [[bool; NUM_BLOCKS_X]; NUM_BLOCKS_Y],
    pub row_colors: [Color; NUM_BLOCKS_Y],
    pub row_scores: [i32; NUM_BLOCKS_Y],
    pub hit_sound: Sound<'aud>,
}

impl<'aud> Blocks<'aud> {
    pub fn new(audio_handle: &'aud RaylibAudio) -> Self {
        Self {
            grid: [[true; NUM_BLOCKS_X]; NUM_BLOCKS_Y],
            row_colors: [
                Color::PURPLE,
                Color::VIOLET,
                Color::BLUEVIOLET,
                Color::BLUE,
                Color::GREEN,
                Color::YELLOWGREEN,
                Color::YELLOW,
                Color::ORANGE,
            ],
            row_scores: [8, 7, 6, 5, 4, 3, 2, 1],
            hit_sound: audio_handle
                .new_sound("assets/hit_block.wav")
                .expect("Failed to load sound"),
        }
    }

    pub fn reset(&mut self) {
        self.grid = [[true; NUM_BLOCKS_X]; NUM_BLOCKS_Y];
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        for y in 0..NUM_BLOCKS_Y {
            for x in 0..NUM_BLOCKS_X {
                // dont ask why it just works
                if self.grid[y][x] {
                    let block_rect = Rectangle {
                        x: 20.0 + x as f32 * BLOCK_WIDTH,
                        y: 40.0 + y as f32 * BLOCK_HEIGHT,
                        width: BLOCK_WIDTH,
                        height: BLOCK_HEIGHT,
                    };
                    let top_left = Vector2 {
                        x: block_rect.x,
                        y: block_rect.y,
                    };
                    let top_right = Vector2 {
                        x: block_rect.x + BLOCK_WIDTH,
                        y: block_rect.y,
                    };
                    let bottum_left = Vector2 {
                        x: block_rect.x,
                        y: block_rect.y + BLOCK_HEIGHT,
                    };
                    let bottum_right = Vector2 {
                        x: block_rect.x + BLOCK_WIDTH,
                        y: block_rect.y + BLOCK_HEIGHT,
                    };
                    let thickness = 1.0;
                    draw_handle.draw_rectangle_rec(block_rect, self.row_colors[y]);
                    draw_handle.draw_line_ex(top_left, top_right, thickness, Color::LIGHTGRAY);
                    draw_handle.draw_line_ex(top_left, bottum_left, thickness, Color::LIGHTGRAY);
                    draw_handle.draw_line_ex(top_right, bottum_right, thickness, Color::DARKGRAY);
                    draw_handle.draw_line_ex(bottum_left, bottum_right, thickness, Color::DARKGRAY);
                }
            }
        }
    }
}
