// NOTE: FOR JOSH
// This is what I started doing for reading the cartridge. You can change whatever if you wanna
// make tools or sumthin here. You prob know what you want better than me.

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
