extern crate sdl2;
extern crate hlua;

use audio;
use lua;
use display;

use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use std::time::Duration;
use std::thread;

pub struct Emulator<'a> {
    pub sdl: Sdl,
    pub channels: [audio::Channel; 4],
    pub lua: hlua::Lua<'a>,
}

// You can only create one of these.
impl<'a> Emulator<'a> {
    pub fn new() -> Emulator<'a> {
        let mut sdl = sdl2::init().unwrap();
        let mut channels = [
            audio::Channel::new(&mut sdl), audio::Channel::new(&mut sdl),
            audio::Channel::new(&mut sdl), audio::Channel::new(&mut sdl),
        ];

        let mut l = lua::create_lua();

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

