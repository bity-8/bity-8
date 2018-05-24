extern crate sdl2;

mod test;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/image.(png|jpg)")
    } else {
        test::run(Path::new(&args[1]));
    }
}
