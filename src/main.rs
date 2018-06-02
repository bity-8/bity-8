extern crate sdl2;
extern crate bity_8;

use bity_8::lua;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/file.lua")
    } else {
        let mut l = lua::load_file(Path::new(&args[1]));

        // I'm opening the libs for testing. In the final thing, we prob won't open any libs!
        l.openlibs();

        match lua::call_init(l) {
            Ok(_v) => (),
            Err(_e) => eprintln!("{}", _e),
        }
    }
}
