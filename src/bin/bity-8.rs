extern crate bity_8;

use bity_8::lua;
use bity_8::emulator;
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
        lua::load_file(Path::new(&args[1]), &mut em.lua);

        //for i in 0..5*90 {
            //em.channels[0].play_note(i, 0, 15);
            //em.channels[1].play_note(i, 1, 15);
            //em.channels[2].play_note(i, 2, 15);
            //em.channels[3].play_note(i, 3, 15);
            //for x in em.channels.iter() { x.device.resume(); }
        //}
        
        // Game loop.
        em.run();
    }
}
