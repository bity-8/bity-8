extern crate hlua;

use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::sys::{SDL_BYTEORDER, SDL_LIL_ENDIAN};

use memory as mem;

pub const SCR_X: u32 = 192;
pub const SCR_Y: u32 = 144;
pub const PIX_LEN: u32 = 2; // the size for each pixel.

// Does the obvious, draws the screen to the canvas.
// TODO: this is too slow.
// This could be threaded maybe.
pub fn draw_screen(texture: &mut Texture) {
    let pal = mem::get_sub_area(mem::LOC_HARD, mem::OFF_HARD_PAL);
    let mut colors = [Color::RGB(0, 0, 0); 16];

    for i in 0..16 {
        colors[i] = Color::RGB(pal[i*3] as u8, pal[i*3+1] as u8, pal[i*3+2] as u8)
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
            arr[i + 0usize] = colors[left].rgba().2;
            arr[i + 1usize] = colors[left].rgba().1;
            arr[i + 2usize] = colors[left].rgba().0;

            // right
            arr[i + 4usize] = colors[right].rgba().2;
            arr[i + 5usize] = colors[right].rgba().1;
            arr[i + 6usize] = colors[right].rgba().0;
        }
    }).unwrap();
}
/*
<<<<<<< HEAD
=======
pub fn run(l: &mut hlua::Lua, sdl_context: &mut Sdl) {
    // Measured in nano seconds.
    let fps = Duration::from_secs(1).checked_div(60).unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Cursor", SCR_X*PIX_LEN, SCR_Y*PIX_LEN)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build() .unwrap();

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(texture_creator.default_pixel_format(), SCR_X, SCR_Y)
        .unwrap();

    let mut events = sdl_context.event_pump().unwrap();

    let mut prev_keys = HashSet::new();

    //mem::get_area(mem::LOC_HARD)[0] = 0;

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
        // TODO: put a match here instead.
        let elapsed = now.elapsed();

        if fps > elapsed {
            let diff = fps.checked_sub(elapsed).unwrap();
            // println!("elapsed: {}, sleep: {}", elapsed.subsec_nanos(), diff.subsec_nanos());
            thread::sleep(diff);
        }
    }
}
>>>>>>> c302ad8d73ea68bc115543484c211c7ed6e44b42
*/
