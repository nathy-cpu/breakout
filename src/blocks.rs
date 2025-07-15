use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
};

pub const NUM_BLOCKS_X: usize = 10;
pub const NUM_BLOCKS_Y: usize = 8;
pub const BLOCK_WIDTH: f32 = 28.0;
pub const BLOCK_HEIGHT: f32 = 10.0;

pub struct Blocks {
    pub blocks: [[bool; NUM_BLOCKS_X]; NUM_BLOCKS_Y],
    pub row_colors: [Color; NUM_BLOCKS_Y],
}

impl Blocks {
    pub fn new() -> Self {
        Self {
            blocks: [[true; NUM_BLOCKS_X]; NUM_BLOCKS_Y],
            row_colors: [
                Color::BLUE,
                Color::BLUE,
                Color::GREEN,
                Color::GREEN,
                Color::YELLOW,
                Color::YELLOW,
                Color::ORANGE,
                Color::ORANGE,
            ],
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        for y in 0..NUM_BLOCKS_Y {
            for x in 0..NUM_BLOCKS_X {
                // dont ask why it just works
                if self.blocks[y][x] {
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
