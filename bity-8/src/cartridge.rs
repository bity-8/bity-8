// NOTE: FOR JOSH
// This is what I started doing for reading the cartridge. You can change whatever if you wanna
// make tools or sumthin here. You prob know what you want better than me.
extern crate byteorder;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::str;

use self::byteorder::{BigEndian};
use memory as mem;

// Each max constraint is: [0-x).
pub const MAX_SPRITE_SHEET: usize = 64;
pub const MAX_TILE_MAP:     usize = 32;
pub const MAX_INSTRUMENT:   usize = 4;
pub const MAX_MEASURE:      usize = 256
pub const MAX_SONG:         usize = 1024;

// Measured in bytes
pub const SIZ_SPRITE_SHEET: usize = 3456;
pub const SIZ_SPRITE_PAL:   usize = 48;
pub const SIZ_TILE_MAP:     usize = 100; // IDK still, this is just a filler.
pub const SIZ_INSTRUMENT:   usize = 128;
pub const SIZ_MEASURE:      usize = 64;
pub const SIZ_MEASURE_DATA: usize = 3;
pub const SIZ_SONG:         usize = 6;

// Assumes the memory offset has a length of three.
fn get_offset(offset: mem::MemLoc) -> u32 {
    assert!(offset.end - offset.start == 3);
    let arr = mem::get_area(offset);
    assert!(arr.len() == 3);
    BigEndian::read_uint(arr, 3) as u32
}

// Returns the length of the offset, and the modulus (for errors).
fn get_offset_len(loc: mem::MemLoc, siz: usize) -> (usize, usize) {
    let len = get_offset(loc);
    (len / size, len % size)
}

pub fn get_sprite_sheet_len() {
}

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
    let maj = mem::peek(mem::COFF_MAJOR.start);
    let min = mem::peek(mem::COFF_MINOR.start);
    Version {
        major: maj,
        minor: min,
    }
}

// Makes sure that there are only
pub fn check_allignment

pub fn get_sprite_sheet(ind: usize) ->  {
    let ss = get_offset_len(mem::COFF_SPRITE_SHEET, SIZ_SPRITE_SHEET);
    let sp = get_offset_len(mem::COFF_SPRITE_PAL,   SIZ_SPRITE_PAL);

    // TODO: Replace these with nice error messages instead.
    // Yeah, I'm getting hacky for the deadline I guess.
    assert!(ss.0 == sp.0);
    assert!(ss.1 == 0);
    assert!(sp.1 == 0);

    let size = ss.0;
    if size > 0 {
        let ind = ind % size;
        let beg = mem::COFF_SPRITE_SHEET.start;
        let end = beg + ind * SIZ_SPRITE_SHEET; mem::COFF_SPRITE_SHEET.start;
        (mem::COFF_SPRITE_SHEET.start..ind
    } else {
        mem::LOC_NULL
    }
}
pub fn get_sprite_pal(ind: usize) {
}
pub fn get_tile_map(ind: usize) {
}
pub fn get_measure(ind: usize) {
}
pub fn get_measure_data(ind: usize) {
}
pub fn get_song(ind: usize) {
}

pub fn get_code_string() -> String {
    println!("{} is the NUMBER.", get_offset(mem::COFF_MEASURE));
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
