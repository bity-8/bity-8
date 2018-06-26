extern crate bity_8;

use bity_8::lua;
use bity_8::sdl2;
use bity_8::display;
use bity_8::audio;
use bity_8::memory as mem;
use std::env;
use std::thread;
use std::path::Path;
use std::time::Duration;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/file.lua")
    } else {
        let mut l = lua::create_lua();
        let mut sdl_context = sdl2::init().unwrap();

        mem::reset_memory();
        lua::load_file(Path::new(&args[1]), &mut l);
        let mut channels = [
            audio::Channel::new(&mut sdl_context), audio::Channel::new(&mut sdl_context),
            audio::Channel::new(&mut sdl_context), audio::Channel::new(&mut sdl_context),
        ];

        for i in 0..20*90 {
            channels[0].play_instrument(i / 20,      i);
            channels[1].play_instrument(i / 20 + 4,  i+0);
            channels[2].play_instrument(i / 20 + 7,  i+0);
            channels[3].play_instrument(i / 20 + 12, i+0);
            for x in channels.iter() { x.device.resume(); }
        }


        thread::sleep(Duration::from_millis(100_000u64));
        // display::run(&mut l, &mut sdl_context);
    }
}
