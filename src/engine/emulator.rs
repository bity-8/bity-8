extern crate hlua;
extern crate sdl2;

use audio;
use display;
use lua;
use memory as mem;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::collections::HashSet;

use std::thread;
use std::time::Duration;

const UP_BTN: i8 = 1;
const DOWN_BTN: i8 = 2;
const LEFT_BTN: i8 = 4;
const RIGHT_BTN: i8 = 8;
const A_BTN: i8 = 16;
const B_BTN: i8 = 32;
const START_BTN: i8 = 64;
const SELECT_BTN: i8 = -128;


pub struct Emulator<'a> {
    pub sdl: Sdl,
    pub channels: [audio::Channel; 4],
    pub lua: hlua::Lua<'a>,
    pub screen: WindowCanvas,
}

// You can only create one of these.
impl<'a> Emulator<'a> {
    pub fn new() -> Emulator<'a> {
        let mut sdl = sdl2::init().unwrap();
        let channels = [
            audio::Channel::new(&mut sdl), audio::Channel::new(&mut sdl),
            audio::Channel::new(&mut sdl), audio::Channel::new(&mut sdl),
        ];

        let l = lua::create_lua();
        let s = display::init(&mut sdl);

        Emulator {
            sdl: sdl,
            channels: channels,
            lua: l,
            screen: s,
        }
    }

    pub fn run(&mut self) {
        // Measured in nano seconds.
        let fps = Duration::from_secs(1).checked_div(60).unwrap();

        let mut events = self.sdl.event_pump().unwrap();
        let mut prev_keys = HashSet::new();

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

            // Handle input            
            prev_keys = get_input(&mut events, prev_keys);

            self.lua.execute::<()>("_update()").unwrap();

            display::draw_screen(&mut self.screen);

            // ----- end measuring time...
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
        let hw_cfg = mem::get_area(mem::OFF_HARD_INP);
        
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
