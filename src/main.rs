extern crate bity_8;

use bity_8::lua;
use bity_8::display;
use bity_8::memory;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/file.lua")
    } else {
        let mut l = lua::create_lua();
        lua::load_file(Path::new(&args[1]), &mut l);
        display::run();
    }
}
