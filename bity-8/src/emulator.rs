extern crate hlua;
extern crate sdl2;

use audio;
use display;
use lua;
use memory as mem;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::gfx::framerate::FPSManager;
use sdl2::Sdl;
use std::collections::HashSet;
use std::time::SystemTime;

use std::thread;
use std::time::Duration;

const LEFT_BTN:  u8 = 0b0000_0001u8;
const RIGHT_BTN: u8 = 0b0000_0010u8;
const UP_BTN:    u8 = 0b0000_0100u8;
const DOWN_BTN:  u8 = 0b0000_1000u8;
const O_BTN:     u8 = 0b0001_0000u8;
const X_BTN:     u8 = 0b0010_0000u8;
const PLUS_BTN:  u8 = 0b0100_0000u8;
const MINUS_BTN: u8 = 0b1000_0000u8;

pub struct Emulator<'a> {
    pub sdl: Sdl,
    pub channels: [audio::Channel; 4],
    pub lua: hlua::Lua<'a>,
}

// You can only create one of these.
impl<'a> Emulator<'a> {
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
        let window = video_subsystem.window("BITY-8",
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

        let mut fps_mgr = FPSManager::new();
        fps_mgr.set_framerate(60)
            .expect("Error when setting framerate!");
        let mut timer = SystemTime::now();
        let mut frames = 0;

        'mainloop: loop {
            // ----- start measuring time...
            use std::time::Instant;
            let now = Instant::now();

            // Music, then measure, then sound.
            audio::update_mem_measure();
            audio::update_mem_note();

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
            frames += 1;

            if SystemTime::now().duration_since(timer).expect("88 MPH").as_secs() == 1 {
                canvas.window_mut().set_title(&format!("BITY-8 ({})", frames))
                    .expect("Error when changing window title!");
                frames = 0;
                timer = SystemTime::now();
            }

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
    // keys = currently held keys
    let keys: HashSet<Keycode> = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

    // Possible way to rewrite this:
    // - Get currently pressed keys
    // - Loop through set of keys
    // - Match against key
    // - Set bit if pressed
    // This does avoid looping through two sets of keys and doing set math.

    // Get the difference between the new and old sets.

    // pressed_keys = current set of keys - last set of keys
    // Results in a set of keys that were not pressed last time
    // If nothing changed, pressed_keys is empty
    let pressed_keys = &keys - &prev_keys;

    // released_keys = last set of keys - current set of keys
    // Results in a set of keys that are not pressed now 
    // that were pressed last time.
    // If nothing changed, released_keys is empty
    let released_keys = &prev_keys - &keys;

    if !pressed_keys.is_empty() || !released_keys.is_empty() {
        let hw_cfg = mem::get_sub_area(mem::LOC_HARD, mem::OFF_INPUT);
        
        // Because we are only using sets of newly pressed or released keys,
        // we can't wipe the input register and OR everything back in. We
        // instead XOR to toggle bits on and off.
        for key in released_keys {
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
                    hw_cfg[0] ^= O_BTN;
                },
                Keycode::X => {
                    hw_cfg[0] ^= X_BTN;
                },
                Keycode::Backspace => {
                    hw_cfg[0] ^= MINUS_BTN;
                },
                Keycode::Return => {
                    hw_cfg[0] ^= PLUS_BTN;
                },
                _ => {}
            }
        }
     
        for key in pressed_keys {
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
                    hw_cfg[0] ^= O_BTN;
                },
                Keycode::X => {
                    hw_cfg[0] ^= X_BTN;
                },
                Keycode::Backspace => {
                    hw_cfg[0] ^= MINUS_BTN;
                },
                Keycode::Return => {
                    hw_cfg[0] ^= PLUS_BTN;
                },
                _ => {}
         
            }
        }
    
    //println!("Register: {:08b}", mem::get_area(mem::OFF_INPUT)[0]);
    }
    return keys;
}
