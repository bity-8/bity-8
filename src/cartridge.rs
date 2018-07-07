// NOTE: This comment block will be deleted soon. It is Alan's rant.
// there are two ideas for reading the cartridge.
// One idea is to put it all into a data structure, and that structure can manipulate things.
// The other idea is to keep it all in memory and just reference the memory when wanting to do
// something.
//
// I think we want the memory thing, because the developer's code should see what the engine sees.
// Nothing should change in the read only portion except on startup. So the first thing that the
// cartridge should do is copy the file into the read only section.
//
// Let's do that then.

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::str;

use memory as mem;

// Wow, is it that simple?
pub fn open(file: &Path) {
    let mut f = File::open(file).expect("cart not found");
    let mut buffer = mem::get_area(mem::LOC_CART);

    // read up to the cartridge
    f.read_exact(buffer);
}

pub fn get_code_string() -> String {
    // TODO: make this more professional for the cartridge, and use the offsets specified at the
    // beginning.

    // for now, we will go until a null character.
    let mut ind = 0;
    let buffer = mem::get_area(mem::LOC_CART);
    for i in 0..buffer.len() {
        if buffer[i] == 0 {
            ind = i;
            break;
        }
    }

    let s = str::from_utf8(&buffer[0..ind]).unwrap();
    let mut s = String::from(s);
    println!("{}", s);
    s
}
