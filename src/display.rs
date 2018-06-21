extern crate sdl2;
extern crate hlua;

const SCR_X: u32 = 192;
const SCR_Y: u32 = 144;
const PIX_LEN: u32 = 2; // the size for each pixel.

use std::time::Duration;
use std::thread;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color;
use self::sdl2::pixels::PixelFormatEnum;
use self::sdl2::render::Texture;
use self::sdl2::sys::{SDL_BYTEORDER, SDL_LIL_ENDIAN};
use memory as mem;

// Does the obvious, draws the screen to the canvas.
// This could be threaded maybe.
pub fn draw_screen(texture: &mut Texture) {
    let pal = mem::get_sub_area(mem::LOC_HARD, mem::OFF_HARD_PAL);
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

pub fn run(l: &mut hlua::Lua) {
    // Measured in nano seconds.
    let fps = Duration::from_secs(1).checked_div(60).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Cursor", SCR_X*PIX_LEN, SCR_Y*PIX_LEN)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build() .unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB888, SCR_X, SCR_Y)
        .unwrap();

    let mut events = sdl_context.event_pump().unwrap();

    'mainloop: loop {
        // ----- start measuring time...
        use std::time::Instant;
        let now = Instant::now();

        for event in events.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape), ..
                } => break 'mainloop,
                _ => {}
            }
        }

        l.execute::<()>("_update()").unwrap();

        draw_screen(&mut texture);
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        // ----- end measuring time...
        // TODO: put a match here.
        let elapsed = now.elapsed();

        if fps > elapsed {
            let diff = fps.checked_sub(elapsed).unwrap();
            // println!("elapsed: {}, sleep: {}", elapsed.subsec_nanos(), diff.subsec_nanos());
            thread::sleep(diff);
        }
    }
}
