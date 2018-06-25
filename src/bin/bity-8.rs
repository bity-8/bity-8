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
        let mut audio_device = audio::init(&mut sdl_context);
        let mut audio_device2 = audio::init(&mut sdl_context);

        for i in 0..20*90 {
            audio::play_instrument(&mut audio_device, 56 + i / 20, mem::LOC_INS1);
            audio::play_instrument(&mut audio_device2, 0 + i / 20, mem::LOC_INS1);
        }
        audio_device.resume();
        audio_device2.resume();

        thread::sleep(Duration::from_millis(100_000u64));
        // audio::run(&mut audio_device);
        // display::run(&mut l, &mut sdl_context);
    }
}
