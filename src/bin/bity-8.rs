extern crate bity_8;

use bity_8::lua;
use bity_8::sdl2;
use bity_8::display;
use bity_8::audio;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/file.lua")
    } else {
        let mut l = lua::create_lua();
        let mut sdl_context = sdl2::init().unwrap();

        lua::load_file(Path::new(&args[1]), &mut l);
        audio::run(&mut sdl_context);
        // display::run(&mut l, &mut sdl_context);
    }
}
