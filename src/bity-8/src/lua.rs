extern crate hlua;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;

// To see the generated assembly, run:
// cargo rustc --release --example basic -- --emit=asm

pub fn run(file: &Path) {
    // Create lua virtual machine.
    let mut lua = hlua::Lua::new();

    let f = File::open(file).expect("file not found");
    let buf_reader = BufReader::new(f);

    lua.execute_from_reader::<(), _>(buf_reader)
        .expect("something went wrong with running this file.");

    let val: i32 = lua.get("b").unwrap();
    println!("executed: {}", val);
}
