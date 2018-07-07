extern crate bity_8;

use bity_8::lua;
use bity_8::emulator;
use bity_8::cartridge;
use bity_8::memory as mem;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run --bin bity-8 /path/to/file.lua")
    } else {
        // Hardware initialization.
        let mut em = emulator::Emulator::new();

        // Memory initialization.
        mem::reset_memory();

        // Code Initialization.
        cartridge::open(Path::new(&args[1]));
        let code = cartridge::get_code_string();

        // In theory, you should only pass it the section that has your code.
        // Well, maybe it should read from a byte array instead.
        lua::load_code(&code, &mut em.lua);

        // Game loop.
        em.run();
    }
}
