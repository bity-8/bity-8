extern crate hlua;
extern crate sdl2;

use audio;
use display;
use lua;
use memory as mem;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::Sdl;
use std::collections::HashSet;

use std::thread;
use std::time::Duration;

const LEFT_BTN: i8  = 1;
const RIGHT_BTN: i8 = 2;
const UP_BTN: i8    = 4;
const DOWN_BTN: i8  = 8;
const A_BTN: i8     = 16;
const B_BTN: i8     = 32;
const START_BTN: i8 = 64;
const SELECT_BTN: i8 = -128;

pub struct Emulator<'a> {
    pub sdl: Sdl,
    pub channels: [audio::Channel; 4],
    pub lua: hlua::Lua<'a>,
}

// You can only create one of these.
impl<'a> Emulator<'a> {
    // Reads the memory, plays the note, updates memory.
    pub fn update_audio_memory(&mut self) {
        let notes = mem::get_sub_area(mem::LOC_HARD, mem::OFF_HARD_NOT);

        // 2 notes per channel. 2 bytes per note. 4 channels. So... 16 bytes.
        for i in 0..4 {
            notes[i*4+0] = notes[i*4+2]; // move next to current note.
            notes[i*4+1] = notes[i*4+3]; // move next to current note.
        }
    }

    pub fn new() -> Emulator<'a> {
        let mut sdl = sdl2::init().unwrap();
        let channels = [
            audio::Channel::new(&mut sdl, 0), audio::Channel::new(&mut sdl, 1),
            audio::Channel::new(&mut sdl, 2), audio::Channel::new(&mut sdl, 3),
        ];

        let l = lua::create_lua();

        Emulator {
            sdl: sdl,
            channels: channels,
            lua: l,
        }
    }

    pub fn run(&mut self) {
        // Measured in nano seconds.
        let fps = Duration::from_secs(1).checked_div(60).unwrap();
        let video_subsystem = self.sdl.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo: Cursor",
                                            display::SCR_X*display::PIX_LEN,
                                            display::SCR_Y*display::PIX_LEN)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas()
            .target_texture()
            .present_vsync()
            .build() .unwrap();

        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB888, display::SCR_X, display::SCR_Y)
            .unwrap();

        let mut events = self.sdl.event_pump().unwrap();
        let mut prev_keys = HashSet::new();

        // call init
        // self.lua.execute::<()>("_init()").unwrap();

        // start sound
        for x in self.channels.iter() { x.device.resume(); }

        'mainloop: loop {
            // ----- start measuring time...
            use std::time::Instant;
            let now = Instant::now();

            // Handle sound first
            self.update_audio_memory();

            for event in events.poll_iter() {
                match event {
                    Event::Quit{..} | Event::KeyDown {
                        keycode: Option::Some(Keycode::Escape), ..
                    } => break 'mainloop,
                    _ => {}
                }
            }

            // Handle input            
            prev_keys = get_input(&mut events, prev_keys);


            // Handle code
            self.lua.execute::<()>("_update()").unwrap();

            display::draw_screen(&mut texture);
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
}

fn get_input(events: &mut sdl2::EventPump, prev_keys: HashSet<Keycode>) -> HashSet<Keycode> {
    let keys: HashSet<Keycode> = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

    // Get the difference between the new and old sets.
    let new_keys = &keys - &prev_keys;
    let old_keys = &prev_keys - &keys;

    if !new_keys.is_empty() || !old_keys.is_empty() {
        let hw_cfg = mem::get_sub_area(mem::LOC_HARD, mem::OFF_HARD_INP);
        
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
    
    //println!("Register: {:08b}", mem::get_area(mem::OFF_HARD_INP)[0]);
    }
    return keys;
}