//extern crate hlua;
extern crate sdl2;

mod lua;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/file.lua")
    } else {
        lua::load_file(Path::new(&args[1]));
    }
}