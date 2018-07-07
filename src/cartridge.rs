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

// So, do I still want to store all the offsets in a header data structure?
// Or do I want to just have functions that return the offsets? In theory, the readonly cartridge
// section won't ever change, so it would be fine to have a header thing. But, what about when
// multi-cart functionality is included? Then we would have a problem. Unless we figured out the
// header information.
//
// And if someone were able to change the read only section, then they could change the data in it,
// which should be fine. We should be assuming that all the information is contained in memory and
// that the program doesn't store "hidden variables". This is emulating fantasy hardware. So there
// should just be functions that gives you each thing.

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::str;

use memory as mem;

pub struct Version {
    major: u8,
    minor: u8
}

// Wow, is it that simple?
pub fn open(file: &Path) {
    let mut f = File::open(file).expect("cart not found");
    let mut buffer = mem::get_area(mem::LOC_CART);

    // read up to the cartridge
    f.read_exact(buffer);
}

pub fn get_version() -> Version {
    let maj = mem::peek(mem::OFF_MAJOR.start);
    let min = mem::peek(mem::OFF_MINOR.start);
    Version {
        major: maj,
        minor: min,
    }
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
