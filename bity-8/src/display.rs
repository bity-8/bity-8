extern crate hlua;

use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::sys::{SDL_BYTEORDER, SDL_LIL_ENDIAN};

use memory as mem;

pub const SCR_X: u32 = 192;
pub const SCR_Y: u32 = 144;
pub const PIX_LEN: u32 = 2; // the size for each pixel.

// Does the obvious, draws the screen to the canvas.
// This could be threaded maybe.
pub fn draw_screen(texture: &mut Texture) {
    let pal = mem::get_sub_area(mem::LOC_HARD, mem::OFF_PALETTE);
    let mut colors = [Color::RGB(0, 0, 0); 16];

    for i in 0..16 {
        let (col1, col2, col3) = (pal[i*3] as u8, pal[i*3+1] as u8, pal[i*3+2] as u8);

        if SDL_BYTEORDER == SDL_LIL_ENDIAN {
            colors[i] = Color::RGB(col3, col2, col1);
        } else {
            colors[i] = Color::RGB(col1, col2, col3);
        }
    }

    let screen = mem::get_area(mem::LOC_SCRE);

    texture.with_lock(None, |arr, _row_w| {
        // remember there are 2 pixels in each byte.
        for i in 0..screen.len() {
            let cols  = screen[i] as usize;
            let left  = (cols & 0xF0) >> 4;
            let right = cols & 0x0F;
            let i = (i * 8) as usize;

            // left
            arr[i + 0usize] = colors[left].r;
            arr[i + 1usize] = colors[left].g;
            arr[i + 2usize] = colors[left].b;

            // right
            arr[i + 4usize] = colors[right].r;
            arr[i + 5usize] = colors[right].g;
            arr[i + 6usize] = colors[right].b;
        }
    }).unwrap();
}
