extern crate sdl2;
extern crate hlua;

const SCR_X: u32 = 192;
const SCR_Y: u32 = 144;
const PIX_LEN: u32 = 2; // the size for each pixel.

use std::time::Duration;
use std::thread;
use self::sdl2::event::Event;
use self::sdl2::render::WindowCanvas;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color;
use self::sdl2::rect::Point;
use self::sdl2::render::Texture;
use memory as mem;

// Does the obvious, draws the screen to the canvas.
// TODO: this is too slow.
// This could be threaded maybe.
pub fn draw_screen(canvas: &mut WindowCanvas, mut texture: &mut Texture) {
    canvas.with_texture_canvas(&mut texture, |tc| {
        let pal = mem::get_sub_area(mem::LOC_HARD, mem::OFF_HARD_PAL);
        let mut colors = [Color::RGB(0, 0, 0); 16];

        for i in 0..16 {
            colors[i] = Color::RGB(pal[i*3] as u8, pal[i*3+1] as u8, pal[i*3+2] as u8)
        }

        let screen = mem::get_area(mem::LOC_SCRE);
        let mut draw_func = |col, x, y| {
            tc.set_draw_color(col);
            tc.draw_point(Point::new(x as i32, y as i32)).unwrap();
        };

        // remember there are 2 pixels in each byte.
        for i in 0..screen.len() {
            let cols  = screen[i] as usize;
            let left  = (cols & 0xF0) >> 4;
            let right = cols & 0x0F;

            let i = (i as i32) * 2;
            let x = i % SCR_X as i32;
            let y = i / SCR_X as i32;

            draw_func(colors[left], x, y);
            draw_func(colors[right], x+1, y);
        }
    }).unwrap();
}

pub fn run(l: &mut hlua::Lua) {
    let fps = 30i64;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Cursor", SCR_X*PIX_LEN, SCR_Y*PIX_LEN)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .target_texture() .software() .build() .unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(texture_creator.default_pixel_format(), SCR_X, SCR_Y)
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

        draw_screen(&mut canvas, &mut texture);
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        // ----- end measuring time...
        let elapsed = now.elapsed();
        let elapsed = (elapsed.as_secs()*1_000) as i64 + (elapsed.subsec_nanos()/1_000_000) as i64;
        let diff = 1_000i64 / fps - elapsed;

        println!("{}: elapsed, {}: fps", elapsed, diff);

        if diff > 0 {
            thread::sleep(Duration::from_millis((fps-elapsed) as u64));
            println!("slept");
        }
    }
}
