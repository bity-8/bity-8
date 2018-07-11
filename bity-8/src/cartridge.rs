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
pub const MAX_SPRITE:     usize = 64;
pub const MAX_PALETTE:    usize = 16;
pub const MAX_TILE_MAP:   usize = 32;
pub const MAX_INSTRUMENT: usize = 4;
pub const MAX_MEASURE:    usize = 256
pub const MAX_SONG:       usize = 1024;

// Measured in bytes
pub const SIZ_SPRITE:    usize = 3456;
pub const SIZ_PALETTE:    usize = 48;
pub const SIZ_TILE_MAP:   usize = 6912;
pub const SIZ_INSTRUMENT: usize = 128;
pub const SIZ_MEASURE:    usize = 67;
pub const SIZ_SONG:       usize = 6;

// Assumes the memory offset has a length of three.
fn get_off(offset: mem::MemLoc) -> usize {
    assert!(offset.end - offset.start == 3);
    let arr = mem::get_area(offset);
    assert!(arr.len() == 3);
    BigEndian::read_uint(arr, 3) as usize
}

// Returns the length of the offset, and the modulus (for errors).
fn get_off_info(loc: mem::MemLoc, siz: usize) -> (usize, usize) {
    let nxt_loc = (loc.start+3..loc.end+3); // Assuming all the offsets have a size of 3.
    let len = get_off(nxt_loc) - get_off(loc);
    (len / size, len % size, off)
}

fn get_off_len(loc: mem::MemLoc, siz: usize) -> usize {
    get_off_info(loc, siz).0
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

// Makes sure that different offsets aren't off.
// If they are off, then this panics.
pub fn check_allignment() ->  {
    // TODO: Replace these with nice error messages instead.
    // Yeah, I'm getting hacky for the deadline I guess.

    let sp = get_off_info(mem::COFF_SPRITE,     SIZE_SPRITE);
    let pa = get_off_info(mem::COFF_PALETTE,    SIZ_PALETTE);
    let tm = get_off_info(mem::COFF_TILE_MAP,   SIZ_TILE_MAP);
    let is = get_off_info(mem::COFF_INSTRUMENT, SIZ_INSTRUMENT);
    let me = get_off_info(mem::COFF_MEASURE,    SIZ_MEASURE);
    let so = get_off_info(mem::COFF_SONG,       SIZ_SONG); // uses the code offset here.

    // -------- BAD MODS --------
    if sp.1 != 0 { panic!("Sprite Sheet is not divisible by size"); }
    if pa.1 != 0 { panic!("Palette is not divisible by size"); }
    if tm.1 != 0 { panic!("Tile Map is not divisible by size"); }
    if is.1 != 0 { panic!("Instrument is not divisible by size"); }
    if me.1 != 0 { panic!("Measure is not divisible by size"); }
    if so.1 != 0 { panic!("Song is not divisible by size"); }

    // -------- OVERSIZED REFS --------
    if sp.0 > MAX_SPRITE     { panic!("Sprite Sheet is too big"); }
    if pa.0 > MAX_PALETTE    { panic!("Palette is too big"); }
    if tm.0 > MAX_TILE_MAP   { panic!("Tile Map is too big"); }
    if is.0 > MAX_INSTRUMENT { panic!("Instrument is too big"); }
    if me.0 > MAX_MEASURE    { panic!("Measure is too big"); }
    if so.0 > MAX_SONG       { panic!("Song is too big"); }
    
    // -------- MAGIC NUMBER --------
    let mn = mem::get_area(mem::COFF_MAGIC_NUM);
    assert!(mem::COFF_MAGIC_NUM.end - mem::COFF_MAGIC_NUM.start == 6);

    // -------- ORDERED REFS --------
    let sp = get_off(mem::COFF_SPRITE);
    let pa = get_off(mem::COFF_PALETTE);
    let tm = get_off(mem::COFF_TILE_MAP);
    let is = get_off(mem::COFF_INSTRUMENT);
    let me = get_off(mem::COFF_MEASURE);
    let so = get_off(mem::COFF_SONG);
    let co = get_off(mem::COFF_CODE);

    if sp > pa { panic!("Sprite sheet is past a further offset."); }
    if pa > tm { panic!("Palette is past a further offset."); }
    if tm > is { panic!("Tile map is past a further offset."); }
    if is > me { panic!("Instrument is past a further offset."); }
    if me > so { panic!("Measure is past a further offset."); }
    if so > co { panic!("Song is past a further offset."); }

    // -------- PAST CART BOUNDARY --------
    if sp > mem::LOC_CART.end { panic!("Sprite sheet is past cart size."); }
    if pa > mem::LOC_CART.end { panic!("Palette is past cart size."); }
    if tm > mem::LOC_CART.end { panic!("Tile map is past cart size."); }
    if is > mem::LOC_CART.end { panic!("Instrument is past cart size."); }
    if me > mem::LOC_CART.end { panic!("Measure is past cart size."); }
    if so > mem::LOC_CART.end { panic!("Song is past cart size."); }
    if co > mem::LOC_CART.end { panic!("Code is past cart size."); }

    // BITY-8
    if mn[0] != 0x41 || mn[1] != 0x49 || mn[2] != 0x54 || mn[3] != 0x59 || mn[4] != 0x2D || mn[5] != 0x38  {
        panic!("The magic number is wrong! It should be: \"BITY-8\"!");
    }
}

fn get_data_loc(ind: usize, data_off: mem::MemLoc, data_size: usize) -> mem::MemLoc {
    let off  = get_off(data_off);
    let size = get_off_len(data_off, data_size);

    if size > 0 && ind < size {
        let beg = data_off.start;
        let end = beg + ind * data_size;
        (beg, end)
    } else {
        mem::LOC_NULL
    }
}

pub fn get_sprite_loc(ind: usize)     -> mem::MemLoc { get_data_loc(ind, mem::COFF_SPRITE,     SIZ_SPRITE)     }
pub fn get_palette_loc(ind: usize)    -> mem::MemLoc { get_data_loc(ind, mem::COFF_PALETTE,    SIZ_PALETTE)    }
pub fn get_tile_map_loc(ind: usize)   -> mem::MemLoc { get_data_loc(ind, mem::COFF_TILE_MAP,   SIZ_TILE_MAP)   }
pub fn get_instrument_loc(ind: usize) -> mem::MemLoc { get_data_loc(ind, mem::COFF_INSTRUMENT, SIZ_INSTRUMENT) }
pub fn get_measure_loc(ind: usize)    -> mem::MemLoc { get_data_loc(ind, mem::COFF_MEASURE,    SIZ_MEASURE)    }
pub fn get_song_loc(ind: usize)       -> mem::MemLoc { get_data_loc(ind, mem::COFF_SONG,       SIZ_SONG)       }
pub fn get_code_loc()                 -> mem::MemLoc { let code = get_off(mem::COFF_CODE); (code, mem::LOC_CART.end) }

pub fn get_code_string() -> String {
    println!("{} is the NUMBER.", get_off(mem::COFF_MEASURE));
    // TODO: make this more professional for the cartridge, and use the offsets specified at the
    // beginning.

    // for now, we will go until a null character.
    let mut ind = 0;
    let buffer = mem::get_area(get_code_loc());
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
