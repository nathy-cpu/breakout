use raylib::{
    color::Color,
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
                if self.blocks[y][x] {
                    // dont ask why it just works
                    draw_handle.draw_rectangle(
                        20 + x as i32 * BLOCK_WIDTH as i32,
                        40 + y as i32 * BLOCK_HEIGHT as i32,
                        BLOCK_WIDTH as i32,
                        BLOCK_HEIGHT as i32,
                        self.row_colors[y],
                    );
                }
            }
        }
    }
}
