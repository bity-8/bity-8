extern crate sdl2;
extern crate hlua;

const SCR_X: u32 = 192;
const SCR_Y: u32 = 144;
const PIX_LEN: u32 = 2; // the size for each pixel.

const UP_BTN: i8 = 1;
const DOWN_BTN: i8 = 2;
const LEFT_BTN: i8 = 4;
const RIGHT_BTN: i8 = 8;
const A_BTN: i8 = 16;
const B_BTN: i8 = 32;
const START_BTN: i8 = 64;
const SELECT_BTN: i8 = -128;

use std::time::Duration;
use std::thread;
use std::collections::HashSet;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color;
use self::sdl2::render::Texture;
use memory as mem;

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

        // Create a set of pressed Keys.
        let keys = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        // Get the difference between the new and old sets.
        let new_keys = &keys - &prev_keys;
        let old_keys = &prev_keys - &keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            let hw_cfg = mem::get_area(mem::OFF_HARD_INP);
            //let mut input_register: u8 = mem::get_area(mem::OFF_HARD_INP)[0] as u8;

            for key in old_keys {
                match key {
                    Keycode::Up => {
                        hw_cfg[0] ^= UP_BTN;
                    },
                    Keycode::Down => {
                        hw_cfg[0] ^= DOWN_BTN;
                    },
                    Keycode::Left => {
                        hw_cfg[0] ^= LEFT_BTN;
                    },
                    Keycode::Right => {
                        hw_cfg[0] ^= RIGHT_BTN;
                    },
                    Keycode::Z => {
                        hw_cfg[0] ^= A_BTN;
                    },
                    Keycode::X => {
                        hw_cfg[0] ^= B_BTN;
                    },
                    Keycode::Backspace => {
                        hw_cfg[0] ^= SELECT_BTN;
                    },
                    Keycode::Return => {
                        hw_cfg[0] ^= START_BTN;
                    },
                    _ => {}
                }
            }

            for key in new_keys {
                match key {
                    Keycode::Up => {
                        hw_cfg[0] ^= UP_BTN;
                    },
                    Keycode::Down => {
                        hw_cfg[0] ^= DOWN_BTN;
                    },
                    Keycode::Left => {
                        hw_cfg[0] ^= LEFT_BTN;
                    },
                    Keycode::Right => {
                        hw_cfg[0] ^= RIGHT_BTN;
                    },
                    Keycode::Z => {
                        hw_cfg[0] ^= A_BTN;
                    },
                    Keycode::X => {
                        hw_cfg[0] ^= B_BTN;
                    },
                    Keycode::Backspace => {
                        hw_cfg[0] ^= SELECT_BTN;
                    },
                    Keycode::Return => {
                        hw_cfg[0] ^= START_BTN;
                    },
                    _ => {}
                }
                
            }
        }

        //println!("Register: {:08b}", mem::get_area(mem::OFF_HARD_INP)[0]);
        prev_keys = keys;

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
