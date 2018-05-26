//extern crate hlua;
extern crate sdl2;

mod lua;
mod display;

use std::env;
use std::path::Path;

fn main() {
    lua::run_lua_test();
    // let args: Vec<_> = env::args().collect();

    // if args.len() < 2 {
        // println!("Usage: cargo run /path/to/image.(png|jpg)")
    // } else {
        // display::run(Path::new(&args[1]));
    // }
}
