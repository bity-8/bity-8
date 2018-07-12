use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::str;

use memory as mem;

// Each max constraint is: [0-x).
pub const MAX_SPRITE:     usize = 64;
pub const MAX_PALETTE:    usize = 16;
pub const MAX_TILE_MAP:   usize = 32;
pub const MAX_INSTRUMENT: usize = 4;
pub const MAX_MEASURE:    usize = 256;
pub const MAX_SONG:       usize = 1024;

// Some Sizes
pub const SIZ_MEASURE_DATA: usize = 64;
pub const SIZ_MEASURE_META: usize = 3;
pub const SIZ_OFFSET: usize = 3;

// Section Sizes
pub const SIZ_SPRITE:     usize = 3456;
pub const SIZ_PALETTE:    usize = 48;
pub const SIZ_TILE_MAP:   usize = 6912;
pub const SIZ_INSTRUMENT: usize = 128;
pub const SIZ_MEASURE:    usize = SIZ_MEASURE_DATA + SIZ_MEASURE_META; // 67
pub const SIZ_SONG:       usize = 6;

// Assumes the memory offset has a length of three.
fn get_off(offset: mem::MemLoc) -> usize {
    assert!(offset.end - offset.start == SIZ_OFFSET);
    let arr = mem::get_area(offset);
    assert!(arr.len() == SIZ_OFFSET);
    // Big Endian
    let mut end  =  arr[2] as usize;
    end |= (arr[1] as usize) << 8;
    end |= (arr[0] as usize) << 16;
    end + mem::LOC_HEAD.end // All offsets of 0 mean that it starts right after the header.
}

// Returns the length of the offset, and the modulus (for errors).
fn get_off_info(loc: mem::MemLoc, siz: usize) -> (usize, usize) {
    let nxt_loc = (loc.start+SIZ_OFFSET)..(loc.end+SIZ_OFFSET); // Assuming all the offsets have a size of SIZ_OFFSET.
    let nxt = get_off(nxt_loc);
    let loc = get_off(loc);
    calc_off_data(loc, nxt, siz)
}

// Returns the length of the offset, and the modulus (for errors).
fn calc_off_data(loc: usize, nxt: usize, siz: usize) -> (usize, usize) {
    if nxt > loc {
        let len = nxt - loc;
        (len / siz, len % siz)
    } else {
        (0, 0)
    }
}

// Wow, is it that simple?
pub fn open(file: &Path) {
    let mut f = File::open(file).expect("cart not found");
    let buffer = mem::get_area(mem::LOC_CART);

    // read up to the cartridge
    match f.read_exact(buffer) {
        Ok(_) => ..,
        Err(_) => ..,
    };
}

pub fn get_version() -> (u8, u8) {
    let maj = mem::peek(mem::COFF_MAJOR.start);
    let min = mem::peek(mem::COFF_MINOR.start);
    (maj, min)
}

// Makes sure that different offsets aren't off.
// If they are off, then this panics.
pub fn check_offsets() {
    // TODO: Replace these with nice error messages instead.
    // Yeah, I'm getting hacky for the deadline I guess.

    // -------- MAGIC NUMBER --------
    let mn = mem::get_area(mem::COFF_MAGIC_NUM);
    assert!(mem::COFF_MAGIC_NUM.end - mem::COFF_MAGIC_NUM.start == 6);

    // BITY-8
    if mn[0] != 0x42 || mn[1] != 0x49 || mn[2] != 0x54 || mn[3] != 0x59 || mn[4] != 0x2D || mn[5] != 0x38  {
        panic!("The magic number is wrong! It should be: \"BITY-8\"!");
    }

    // -------- BAD MODS --------
    let sp = calc_off_data(mem::LOC_HEAD.end, mem::COFF_PALETTE.start, SIZ_SPRITE);
    let pa = get_off_info(mem::COFF_PALETTE,    SIZ_PALETTE);
    let tm = get_off_info(mem::COFF_TILE_MAP,   SIZ_TILE_MAP);
    let is = get_off_info(mem::COFF_INSTRUMENT, SIZ_INSTRUMENT);
    let me = get_off_info(mem::COFF_MEASURE,    SIZ_MEASURE);
    let so = get_off_info(mem::COFF_SONG,       SIZ_SONG); // uses the code offset here.

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
    
    // -------- ORDERED REFS --------
    let pa = get_off(mem::COFF_PALETTE);
    let tm = get_off(mem::COFF_TILE_MAP);
    let is = get_off(mem::COFF_INSTRUMENT);
    let me = get_off(mem::COFF_MEASURE);
    let so = get_off(mem::COFF_SONG);
    let co = get_off(mem::COFF_CODE);

    if pa > tm { panic!("Palette is past a further offset."); }
    if tm > is { panic!("Tile map is past a further offset."); }
    if is > me { panic!("Instrument is past a further offset."); }
    if me > so { panic!("Measure is past a further offset."); }
    if so > co { panic!("Song is past a further offset."); }

    // -------- PAST CART END --------
    let end = mem::LOC_CART.end;

    if pa > end { panic!("Palette is past cart size."); }
    if tm > end { panic!("Tile map is past cart size."); }
    if is > end { panic!("Instrument is past cart size."); }
    if me > end { panic!("Measure is past cart size."); }
    if so > end { panic!("Song is past cart size."); }
    if co > end { panic!("Code is past cart size."); }
}

// helper function for getting locations
fn get_data_loc(ind: usize, loc: mem::MemLoc, data_size: usize) -> mem::MemLoc {
    let nxt_loc = (loc.start+SIZ_OFFSET)..(loc.end+SIZ_OFFSET);
    let off  = get_off(loc);
    let nxt_off = get_off(nxt_loc);

    calc_data_loc(ind, off, nxt_off, data_size)
}

// another helper function
fn calc_data_loc(ind: usize, off: usize, nxt_off: usize, data_size: usize) -> mem::MemLoc {
    let size = (nxt_off - off) / data_size;

    if size > 0 && ind < size {
        let end = off + ind * data_size;
        (off..end)
    } else {
        mem::LOC_NULL
    }
}

pub fn get_sprite_loc(ind: usize)     -> mem::MemLoc { calc_data_loc(ind, mem::LOC_HEAD.end, get_off(mem::COFF_PALETTE), SIZ_SPRITE) }
pub fn get_palette_loc(ind: usize)    -> mem::MemLoc { get_data_loc(ind, mem::COFF_PALETTE,    SIZ_PALETTE)    }
pub fn get_tile_map_loc(ind: usize)   -> mem::MemLoc { get_data_loc(ind, mem::COFF_TILE_MAP,   SIZ_TILE_MAP)   }
pub fn get_instrument_loc(ind: usize) -> mem::MemLoc { get_data_loc(ind, mem::COFF_INSTRUMENT, SIZ_INSTRUMENT) }
pub fn get_measure_loc(ind: usize)    -> mem::MemLoc { get_data_loc(ind, mem::COFF_MEASURE,    SIZ_MEASURE)    }
pub fn get_song_loc(ind: usize)       -> mem::MemLoc { get_data_loc(ind, mem::COFF_SONG,       SIZ_SONG)       }
pub fn get_code_loc()                 -> mem::MemLoc { let code = get_off(mem::COFF_CODE); (code..mem::LOC_CART.end) }

// The audio format has things mixed into it.
pub fn get_measure_data_loc(ind: usize) -> mem::MemLoc {
    let m = get_measure_loc(ind);
    if m != mem::LOC_NULL {
        assert!(m.end - m.start > 0);
        assert!(m.end -  (m.start + SIZ_MEASURE_META) == SIZ_MEASURE_DATA);
        (m.start+SIZ_MEASURE_META)..(m.end)
    } else {
        m
    }
}

// The audio format has things mixed into it.
pub fn get_measure_meta_loc(ind: usize) -> mem::MemLoc {
    let m = get_measure_loc(ind);
    if m != mem::LOC_NULL {
        assert!(m.end - m.start > 0);
        (m.start)..(m.start+SIZ_MEASURE_META)
    } else {
        m
    }
}

pub fn get_code_string() -> String {
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
    let s = String::from(s);
    s
}
