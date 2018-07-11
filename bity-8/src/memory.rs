use std::cmp;
use std::ops::Range;

pub type Memory = [u8; CART_LEN];
pub type MemLoc = Range<usize>;

pub const CART_LEN : usize = 0x50000; // End of cartridge
static mut MEM: Memory = [0; CART_LEN];

// --- Global Memory Constants ---
pub const LOC_NULL: MemLoc = (0x00000..0x00000); // Used for addressing null locations.
pub const LOC_CART: MemLoc = (0x00000..0x40000); // Cartridge
pub const LOC_HARD: MemLoc = (0x40000..0x40400); // Hardware Config
pub const LOC_SCRE: MemLoc = (0x40400..0x43A00); // Screen Buffer
pub const LOC_SPRI: MemLoc = (0x43A00..0x47000); // Sprite Sheets
pub const LOC_TILE: MemLoc = (0x47000..0x4DC00); // Tile Maps
pub const LOC_INST: MemLoc = (0x4DC00..0x4E000); // Instruments
pub const LOC_FONT: MemLoc = (0x4E000..0x4ED80); // Font
pub const LOC_SAVE: MemLoc = (0x4ED80..0x4F180); // Save
pub const LOC_EMPT: MemLoc = (0x4F180..0x4F800); // Empty
pub const LOC_MULT: MemLoc = (0x4F800..0x50000); // Multicart

// -- Global helpers that may be useful --
pub const LOC_ALL:        MemLoc = (0x00000..0x50000);
pub const LOC_REBOOTABLE: MemLoc = (0x00000..0x4F800); // Memory that gets reset
pub const LOC_ROM:        MemLoc = (0x00000..0x40000);
pub const LOC_WRITABLE:   MemLoc = (0x40000..0x50000);

// --- Cartridge locations ---
pub const LOC_HEAD:          MemLoc = (0x00000..0x00036); // header of cartridge
pub const COFF_MAGIC_NUM:    MemLoc = (0x00000..0x00006); // BITY-8
pub const COFF_MAJOR:        MemLoc = (0x00006..0x00007);
pub const COFF_MINOR:        MemLoc = (0x00007..0x00008);
pub const COFF_RESERVED:     MemLoc = (0x00008..0x00018); // probably a checksum.
pub const COFF_SPRITE:       MemLoc = (0x00018..0x0001B); // sprite, TODO: maybe delete.
pub const COFF_PALETTE:      MemLoc = (0x0001B..0x0001E);
pub const COFF_TILE_MAP:     MemLoc = (0x0001E..0x00021); // map
pub const COFF_INSTRUMENT:   MemLoc = (0x00021..0x00024); // audio
pub const COFF_MEASURE:      MemLoc = (0x00024..0x00027); // audio
pub const COFF_MEASURE_DATA: MemLoc = (0x00027..0x00030);
pub const COFF_SONG:         MemLoc = (0x00030..0x00033);
pub const COFF_CODE:         MemLoc = (0x00033..0x00036);

// --- Hardware Config Locations ---
pub const OFF_PALETTE:   MemLoc = (0x00..0x30); // Pallete
pub const OFF_INPUT:     MemLoc = (0x31..0x32); // Input
pub const OFF_NOTES:     MemLoc = (0x32..0x42); // Current Notes. 2 bytes per note, 2 notes per channel (prev and next).
pub const OFF_MEAS_META: MemLoc = (0x42..0x4e); // loaded from cart, looping + volume, tempo
pub const OFF_MEAS_CTRL: MemLoc = (0x4e..0x56); // only in memory, 1 byte for current note length left, 5 bits for current note . 3 bits reserved
pub const OFF_CHAN_FLAG: MemLoc = (0x56..0x57); // only in memory, reserved . music playing . sfx playing, 000      . 0             . 0000
pub const OFF_SONG_META: MemLoc = (0x57..0x5D); // loaded from cart
pub const OFF_MEAS:      MemLoc = (0x100..0x200); // loaded from cart, 64 bytes per measure, 4 channels

// --- 8 Instruments ---
pub const OFF_INS1: MemLoc = (0x000..0x080);
pub const OFF_INS2: MemLoc = (0x080..0x100);
pub const OFF_INS3: MemLoc = (0x100..0x180);
pub const OFF_INS4: MemLoc = (0x180..0x200);
pub const OFF_INS5: MemLoc = (0x200..0x280);
pub const OFF_INS6: MemLoc = (0x280..0x300);
pub const OFF_INS7: MemLoc = (0x300..0x380);
pub const OFF_INS8: MemLoc = (0x380..0x400);

// private worker functions
fn add_mems(r1: MemLoc, r2: MemLoc) -> MemLoc {
    (r1.start + r2.start .. r1.start + r2.end)
}

fn pos_in_bounds(pos: usize, area: MemLoc) -> bool {
    pos >= area.start && pos < area.end && pos < CART_LEN
}

fn area_intersect(a1: MemLoc, a2: MemLoc) -> MemLoc {
    let start = cmp::min(cmp::max(a1.start, a2.start), CART_LEN);
    let end   = cmp::min(cmp::min(a1.end,   a2.end),   CART_LEN);

    if start >= end {
        start..start
    } else {
        start..end
    }
}

// Helper funcs for getting certain areas of the cartridge.
pub fn get_area(area: MemLoc) -> &'static mut [u8] {
    unsafe { &mut MEM[area] }
}

pub fn get_sub_area(area: MemLoc, off: MemLoc) -> &'static mut [u8] {
    unsafe { &mut MEM[add_mems(area, off)] }
}

// Maps a vector to memory. This is safe if you mess up the parameters.
pub fn map_vector(start: usize, len: usize, vec: &[u8]) {
    let len = cmp::min(vec.len(), len);

    for vec_ind in 0..len {
        let mem_ind = start+vec_ind;
        if mem_ind >= CART_LEN { return }
        unsafe { MEM[mem_ind] = vec[vec_ind]; }
    }
}

// Maps one value to memory.
// You can use this for clearing memory.
fn mset(pos: usize, len: usize, val: u8, area: MemLoc) {
    for i in area_intersect(pos..pos+len, area) {
        unsafe { MEM[i] = val; }
    }
}

fn mcpy(dest: usize, pos: usize, len: usize, area: MemLoc) {
    // if the section of memory you write to is read only, then don't write on that section, but
    // also don't stop!

    let area = area_intersect(dest..dest+len, area);
    let area = area.start - dest .. area.end - area.start;

    for i in area {
        unsafe {
            MEM[i+dest] = MEM[i+pos];
        }
    }
}

// Write to a memory location
fn poke(pos: usize, val: u8, area: MemLoc) {
    if pos_in_bounds(pos, area) {
        unsafe { MEM[pos] = val; }
    }
}

// Read a memory location
pub fn peek(pos: usize) -> u8 {
    if pos < CART_LEN {
        unsafe { MEM[pos] }
    } else {
        0
    }
}

// for short/writable areas.
pub fn poke_w(pos: usize, val: u8)  { poke(pos, val, LOC_WRITABLE); }
pub fn mset_w(pos: usize, len: usize, val: u8) { mset(pos, len, val, LOC_WRITABLE); }
pub fn mcpy_w(dest: usize, pos: usize, len: usize) { mcpy(dest, pos, len, LOC_WRITABLE); }

pub fn poke_a(pos: usize, val: u8)  { poke(pos, val, LOC_ALL); }
pub fn mset_a(pos: usize, len: usize, val: u8) { mset(pos, len, val, LOC_ALL); }
pub fn mcpy_a(dest: usize, pos: usize, len: usize) { mcpy(dest, pos, len, LOC_ALL); }

pub fn reset_memory() {
    mset_a(0, CART_LEN, 0);
    map_vector(LOC_INST.start, DEF_INST_LEN, &WAVE_DATA);
    map_vector(LOC_FONT.start, DEF_FONT_LEN, &FONT_DATA);
}

#[test]
fn test_mem() {
    mset_a(0, CART_LEN, 0);

    {
        let x = get_area(LOC_CART);
        x[3] = 22;
    }

    map_vector(0, 20, &[2, 4, 5]);
    mcpy_a(4, 0, 4);

    assert_eq!(peek(0), 2);
    assert_eq!(peek(1), 4);
    assert_eq!(peek(2), 5);
    assert_eq!(peek(3), 22);

    assert_eq!(peek(4), 2);
    assert_eq!(peek(5), 4);
    assert_eq!(peek(6), 5);
    assert_eq!(peek(7), 22);

    for i in 8..CART_LEN {
        assert_eq!(peek(i), 0);
    }
}


// used python to generate these :).
const DEF_INST_LEN: usize = 128*4;
const WAVE_DATA: [u8; DEF_INST_LEN] =
[
    // SQUARE_WAVE
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,

    // SAWTOOTH_WAVE
    0x01, 0x04, 0x04, 0x08, 0x08, 0x0c, 0x0c, 0x10, 0x10, 0x14, 0x14, 0x18, 0x18, 0x1c, 0x1c, 0x20,
    0x20, 0x24, 0x24, 0x28, 0x28, 0x2c, 0x2c, 0x30, 0x30, 0x34, 0x34, 0x38, 0x38, 0x3c, 0x3c, 0x40,
    0x40, 0x44, 0x44, 0x48, 0x48, 0x4c, 0x4c, 0x50, 0x50, 0x54, 0x54, 0x58, 0x58, 0x5c, 0x5c, 0x60,
    0x60, 0x64, 0x64, 0x68, 0x68, 0x6c, 0x6c, 0x70, 0x70, 0x74, 0x74, 0x78, 0x78, 0x7c, 0x7c, 0x80,
    0x80, 0x84, 0x84, 0x88, 0x88, 0x8c, 0x8c, 0x90, 0x90, 0x94, 0x94, 0x98, 0x98, 0x9c, 0x9c, 0xa0,
    0xa0, 0xa4, 0xa4, 0xa8, 0xa8, 0xac, 0xac, 0xb0, 0xb0, 0xb4, 0xb4, 0xb8, 0xb8, 0xbc, 0xbc, 0xc0,
    0xc0, 0xc4, 0xc4, 0xc8, 0xc8, 0xcc, 0xcc, 0xd0, 0xd0, 0xd4, 0xd4, 0xd8, 0xd8, 0xdc, 0xdc, 0xe0,
    0xe0, 0xe4, 0xe4, 0xe8, 0xe8, 0xec, 0xec, 0xf0, 0xf0, 0xf4, 0xf4, 0xf8, 0xf8, 0xfc, 0xfc, 0xff,

    // TRIANGLE_WAVE
    0xff, 0xf8, 0xf8, 0xf0, 0xf0, 0xe8, 0xe8, 0xe0, 0xe0, 0xd8, 0xd8, 0xd0, 0xd0, 0xc8, 0xc8, 0xc0,
    0xc0, 0xb8, 0xb8, 0xb0, 0xb0, 0xa8, 0xa8, 0xa0, 0xa0, 0x98, 0x98, 0x90, 0x90, 0x88, 0x88, 0x80,
    0x80, 0x78, 0x78, 0x70, 0x70, 0x68, 0x68, 0x60, 0x60, 0x58, 0x58, 0x50, 0x50, 0x48, 0x48, 0x40,
    0x40, 0x38, 0x38, 0x30, 0x30, 0x28, 0x28, 0x20, 0x20, 0x18, 0x18, 0x10, 0x10, 0x08, 0x08, 0x01,
    0x01, 0x08, 0x08, 0x10, 0x10, 0x18, 0x18, 0x20, 0x20, 0x28, 0x28, 0x30, 0x30, 0x38, 0x38, 0x40,
    0x40, 0x48, 0x48, 0x50, 0x50, 0x58, 0x58, 0x60, 0x60, 0x68, 0x68, 0x70, 0x70, 0x78, 0x78, 0x80,
    0x80, 0x88, 0x88, 0x90, 0x90, 0x98, 0x98, 0xa0, 0xa0, 0xa8, 0xa8, 0xb0, 0xb0, 0xb8, 0xb8, 0xc0,
    0xc0, 0xc8, 0xc8, 0xd0, 0xd0, 0xd8, 0xd8, 0xe0, 0xe0, 0xe8, 0xe8, 0xf0, 0xf0, 0xf8, 0xf8, 0xff,

    // NOISE_WAVE

    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];


// the font :), maybe this shouldn't go here. But it should be part of the executable. Okay, yeah,
// let's put it in here for now.
const DEF_FONT_LEN: usize = 96 * 72 / 2;
const FONT_DATA: [u8; DEF_FONT_LEN] =
[
    0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x11, 0x00, 0x11,
    0x00, 0x10, 0x00, 0x11, 0x00, 0x00, 0x10, 0x00, 0x00, 0x11, 0x00, 0x01, 0x10, 0x00, 0x10, 0x10,
    0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
    0x11, 0x11, 0x00, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x11, 0x11, 0x10, 0x01, 0x10, 0x00, 0x10,
    0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x11, 0x00, 0x01, 0x11,
    0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
    0x11, 0x11, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x11, 0x00, 0x00,
    0x10, 0x00, 0x11, 0x11, 0x00, 0x00, 0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x11, 0x00, 0x11, 0x01,
    0x10, 0x01, 0x11, 0x00, 0x00, 0x00, 0x00, 0x01, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
    0x11, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x01,
    0x00, 0x10, 0x10, 0x01, 0x10, 0x00, 0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x11, 0x00, 0x01, 0x11,
    0x00, 0x00, 0x10, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x10, 0x00, 0x10,
    0x01, 0x10, 0x11, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x10, 0x00, 0x10, 0x10,
    0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x01, 0x10, 0x11,
    0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x10, 0x00, 0x00, 0x00, 0x11, 0x10, 0x00, 0x11, 0x11, 0x10,
    0x11, 0x01, 0x10, 0x11, 0x11, 0x00, 0x00, 0x01, 0x10, 0x00, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11,
    0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x00, 0x10,
    0x00, 0x00, 0x10, 0x00, 0x01, 0x10, 0x00, 0x01, 0x11, 0x00, 0x00, 0x11, 0x00, 0x10, 0x01, 0x10,
    0x11, 0x01, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11,
    0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x01, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x10, 0x00, 0x11, 0x10,
    0x11, 0x01, 0x10, 0x01, 0x11, 0x00, 0x11, 0x00, 0x00, 0x00, 0x01, 0x10, 0x00, 0x01, 0x10, 0x00,
    0x01, 0x10, 0x11, 0x01, 0x10, 0x00, 0x01, 0x10, 0x11, 0x01, 0x10, 0x00, 0x01, 0x10, 0x00, 0x10,
    0x00, 0x00, 0x10, 0x00, 0x01, 0x10, 0x00, 0x01, 0x11, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x01, 0x10, 0x11,
    0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x01, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x00,
    0x00, 0x01, 0x00, 0x00, 0x00, 0x11, 0x10, 0x00, 0x00, 0x00, 0x11, 0x10, 0x00, 0x00, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x00, 0x11,
    0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x00, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11,
    0x10, 0x11, 0x01, 0x10, 0x11, 0x00, 0x00, 0x10, 0x00, 0x10, 0x11, 0x00, 0x10, 0x01, 0x11, 0x00,
    0x10, 0x00, 0x00, 0x11, 0x00, 0x10, 0x11, 0x01, 0x00, 0x11, 0x00, 0x00, 0x11, 0x01, 0x10, 0x11,
    0x00, 0x00, 0x11, 0x00, 0x00, 0x11, 0x00, 0x00, 0x11, 0x00, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11,
    0x10, 0x11, 0x11, 0x00, 0x11, 0x00, 0x00, 0x11, 0x01, 0x10, 0x11, 0x10, 0x10, 0x11, 0x01, 0x10,
    0x10, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x00, 0x00, 0x11, 0x00, 0x10, 0x11,
    0x11, 0x00, 0x11, 0x11, 0x00, 0x11, 0x01, 0x10, 0x11, 0x11, 0x10, 0x00, 0x10, 0x00, 0x00, 0x10,
    0x00, 0x11, 0x11, 0x00, 0x11, 0x00, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x01, 0x10,
    0x10, 0x11, 0x10, 0x11, 0x00, 0x10, 0x11, 0x00, 0x10, 0x11, 0x00, 0x00, 0x11, 0x01, 0x10, 0x11,
    0x00, 0x00, 0x11, 0x00, 0x00, 0x11, 0x00, 0x10, 0x11, 0x00, 0x10, 0x00, 0x10, 0x00, 0x00, 0x10,
    0x00, 0x11, 0x01, 0x10, 0x11, 0x00, 0x00, 0x10, 0x10, 0x10, 0x10, 0x11, 0x10, 0x11, 0x01, 0x10,
    0x01, 0x11, 0x10, 0x11, 0x00, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x00, 0x11,
    0x11, 0x10, 0x11, 0x00, 0x00, 0x11, 0x11, 0x10, 0x11, 0x00, 0x10, 0x11, 0x11, 0x10, 0x11, 0x10,
    0x00, 0x11, 0x01, 0x10, 0x11, 0x11, 0x10, 0x10, 0x00, 0x10, 0x10, 0x01, 0x10, 0x01, 0x11, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x10,
    0x01, 0x10, 0x11, 0x01, 0x10, 0x10, 0x00, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x11,
    0x10, 0x01, 0x11, 0x00, 0x10, 0x00, 0x00, 0x01, 0x11, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x00, 0x10, 0x11, 0x01, 0x10, 0x11, 0x00, 0x10, 0x11, 0x00, 0x00, 0x11, 0x11, 0x10, 0x10,
    0x01, 0x10, 0x11, 0x01, 0x10, 0x10, 0x10, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x00, 0x11,
    0x10, 0x01, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x11, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x11, 0x10, 0x10, 0x01, 0x10, 0x11, 0x11, 0x00, 0x11, 0x11, 0x10, 0x00, 0x10, 0x00, 0x10,
    0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x00, 0x01, 0x10, 0x01, 0x11,
    0x00, 0x01, 0x10, 0x00, 0x00, 0x10, 0x00, 0x00, 0x11, 0x00, 0x11, 0x01, 0x10, 0x00, 0x00, 0x00,
    0x11, 0x00, 0x00, 0x11, 0x11, 0x00, 0x11, 0x00, 0x10, 0x00, 0x01, 0x10, 0x00, 0x10, 0x00, 0x11,
    0x11, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x01, 0x10, 0x11, 0x11, 0x10, 0x11, 0x00,
    0x00, 0x01, 0x10, 0x00, 0x00, 0x01, 0x00, 0x00, 0x11, 0x00, 0x10, 0x00, 0x10, 0x00, 0x00, 0x00,
    0x11, 0x00, 0x00, 0x01, 0x11, 0x10, 0x11, 0x00, 0x10, 0x11, 0x11, 0x10, 0x00, 0x10, 0x00, 0x11,
    0x11, 0x10, 0x00, 0x10, 0x00, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11,
    0x10, 0x01, 0x11, 0x00, 0x00, 0x00, 0x10, 0x01, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x10,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x10, 0x00, 0x11, 0x11, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x10, 0x01,
    0x11, 0x00, 0x01, 0x11, 0x10, 0x11, 0x10, 0x10, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11,
    0x00, 0x11, 0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x01, 0x00, 0x00, 0x01, 0x10, 0x11, 0x00, 0x00, 0x11, 0x11, 0x10, 0x00, 0x01, 0x10, 0x11,
    0x00, 0x10, 0x01, 0x10, 0x00, 0x10, 0x01, 0x10, 0x11, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
    0x00, 0x11, 0x01, 0x00, 0x01, 0x10, 0x00, 0x11, 0x01, 0x10, 0x10, 0x11, 0x10, 0x01, 0x11, 0x00,
    0x00, 0x00, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x00, 0x00, 0x01, 0x11, 0x10, 0x11,
    0x11, 0x10, 0x11, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x00, 0x00, 0x00, 0x11,
    0x00, 0x11, 0x11, 0x00, 0x01, 0x10, 0x00, 0x11, 0x11, 0x10, 0x11, 0x10, 0x10, 0x11, 0x01, 0x10,
    0x00, 0x00, 0x00, 0x10, 0x01, 0x10, 0x11, 0x00, 0x10, 0x11, 0x00, 0x00, 0x10, 0x01, 0x10, 0x11,
    0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x01, 0x10, 0x11, 0x00, 0x10, 0x01, 0x11, 0x00, 0x00, 0x11,
    0x00, 0x11, 0x01, 0x10, 0x01, 0x10, 0x00, 0x10, 0x10, 0x10, 0x11, 0x00, 0x10, 0x11, 0x01, 0x10,
    0x00, 0x00, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x01,
    0x11, 0x10, 0x01, 0x10, 0x00, 0x11, 0x11, 0x10, 0x11, 0x00, 0x10, 0x00, 0x11, 0x00, 0x01, 0x11,
    0x00, 0x11, 0x01, 0x10, 0x01, 0x11, 0x00, 0x10, 0x00, 0x10, 0x11, 0x00, 0x10, 0x01, 0x11, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x10, 0x11, 0x10, 0x11, 0x10, 0x10, 0x10, 0x11, 0x10, 0x01, 0x11, 0x10, 0x00, 0x11, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x11, 0x00, 0x00, 0x10, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x00, 0x10, 0x10, 0x01, 0x10, 0x11, 0x10, 0x10, 0x11, 0x00, 0x00, 0x00, 0x11, 0x00, 0x10,
    0x01, 0x10, 0x11, 0x01, 0x10, 0x10, 0x00, 0x10, 0x11, 0x01, 0x10, 0x10, 0x01, 0x10, 0x11, 0x11,
    0x10, 0x01, 0x10, 0x00, 0x00, 0x10, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x00, 0x00, 0x11, 0x11, 0x00, 0x11, 0x11, 0x10, 0x10,
    0x01, 0x10, 0x11, 0x01, 0x10, 0x10, 0x10, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x00, 0x11,
    0x00, 0x00, 0x11, 0x00, 0x00, 0x10, 0x00, 0x01, 0x10, 0x00, 0x11, 0x10, 0x10, 0x00, 0x00, 0x00,
    0x11, 0x00, 0x00, 0x00, 0x01, 0x10, 0x11, 0x00, 0x00, 0x00, 0x01, 0x10, 0x00, 0x11, 0x00, 0x10,
    0x01, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x00, 0x01, 0x10, 0x01, 0x10,
    0x00, 0x01, 0x10, 0x00, 0x00, 0x10, 0x00, 0x00, 0x11, 0x00, 0x10, 0x11, 0x10, 0x00, 0x00, 0x00,
    0x11, 0x00, 0x00, 0x00, 0x01, 0x10, 0x11, 0x00, 0x00, 0x11, 0x11, 0x10, 0x00, 0x11, 0x10, 0x11,
    0x11, 0x10, 0x00, 0x10, 0x00, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11,
    0x10, 0x00, 0x11, 0x00, 0x00, 0x10, 0x00, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11,
    0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x00, 0x10, 0x10, 0x01, 0x10, 0x11, 0x01, 0x10, 0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x10,
    0x10, 0x10, 0x11, 0x01, 0x10, 0x11, 0x11, 0x10, 0x00, 0x11, 0x00, 0x01, 0x10, 0x00, 0x00, 0x10,
    0x00, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00,
    0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x10, 0x10, 0x10, 0x11,
    0x01, 0x10, 0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x01, 0x11,
    0x00, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x00, 0x10, 0x00, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00,
    0x11, 0x00, 0x10, 0x10, 0x01, 0x10, 0x10, 0x00, 0x10, 0x11, 0x01, 0x10, 0x10, 0x00, 0x10, 0x10,
    0x10, 0x10, 0x11, 0x01, 0x10, 0x11, 0x11, 0x10, 0x00, 0x11, 0x00, 0x01, 0x10, 0x00, 0x01, 0x11,
    0x00, 0x00, 0x10, 0x00, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11,
    0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x10, 0x00, 0x00, 0x10,
    0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x10, 0x00, 0x00, 0x10, 0x00, 0x00, 0x10, 0x00, 0x00, 0x10,
    0x00, 0x00, 0x10, 0x00, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x11, 0x10, 0x10, 0x00, 0x10, 0x11, 0x11, 0x10, 0x10, 0x00, 0x10, 0x11, 0x00, 0x00, 0x11,
    0x00, 0x00, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x01, 0x11,
    0x00, 0x01, 0x01, 0x00, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10,
    0x11, 0x11, 0x10, 0x10, 0x00, 0x10, 0x11, 0x11, 0x10, 0x10, 0x00, 0x10, 0x11, 0x10, 0x00, 0x10,
    0x10, 0x00, 0x11, 0x11, 0x10, 0x10, 0x00, 0x10, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x11, 0x11,
    0x10, 0x10, 0x00, 0x10, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x11, 0x11, 0x10, 0x10, 0x00, 0x10,
    0x11, 0x11, 0x10, 0x10, 0x00, 0x10, 0x11, 0x11, 0x10, 0x10, 0x00, 0x10, 0x11, 0x11, 0x00, 0x10,
    0x01, 0x00, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x11, 0x11, 0x10, 0x10, 0x00, 0x10, 0x11, 0x11,
    0x10, 0x10, 0x00, 0x10, 0x01, 0x11, 0x00, 0x01, 0x01, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10,
    0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11,
    0x11, 0x10, 0x00, 0x10, 0x00, 0x00, 0x10, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x01, 0x11,
    0x00, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x10, 0x10, 0x10, 0x01, 0x01, 0x00, 0x10, 0x01, 0x00, 0x11, 0x10, 0x10, 0x01, 0x00, 0x10, 0x10,
    0x11, 0x10, 0x10, 0x10, 0x10, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x01, 0x01, 0x00, 0x10, 0x00,
    0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10,
    0x01, 0x01, 0x00, 0x10, 0x10, 0x10, 0x01, 0x00, 0x10, 0x10, 0x11, 0x00, 0x10, 0x01, 0x00, 0x01,
    0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01,
    0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00,
    0x10, 0x10, 0x10, 0x01, 0x01, 0x00, 0x00, 0x10, 0x00, 0x11, 0x01, 0x10, 0x00, 0x10, 0x00, 0x11,
    0x01, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01,
    0x10, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x01, 0x00, 0x10, 0x10, 0x10, 0x10, 0x01, 0x00, 0x01, 0x10, 0x10, 0x01, 0x00, 0x10, 0x10,
    0x11, 0x00, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01,
    0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00,
    0x10, 0x10, 0x10, 0x01, 0x01, 0x00, 0x01, 0x00, 0x10, 0x10, 0x11, 0x10, 0x10, 0x01, 0x00, 0x11,
    0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x01, 0x01, 0x00, 0x10, 0x00,
    0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01,
    0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01,
    0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10,
    0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01,
    0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01,
    0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x10, 0x10, 0x10, 0x01,
    0x01, 0x00, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11,
    0x10, 0x11, 0x11, 0x10, 0x10, 0x10, 0x10, 0x01, 0x01, 0x00, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00,
    0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x10,
    0x10, 0x10, 0x00, 0x11, 0x00, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11,
    0x10, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x10, 0x10, 0x10, 0x00, 0x11, 0x00, 0x01, 0x11, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11,
    0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01,
    0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10,
    0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x11,
    0x01, 0x10, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x10, 0x00,
    0x10, 0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x10, 0x00, 0x10,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x10, 0x10, 0x10, 0x01,
    0x01, 0x00, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x11, 0x11,
    0x10, 0x11, 0x11, 0x10, 0x10, 0x10, 0x10, 0x01, 0x01, 0x00, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00,
    0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x10,
    0x10, 0x10, 0x00, 0x11, 0x00, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x11, 0x11, 0x10, 0x11, 0x11,
    0x10, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x10, 0x10, 0x10, 0x00, 0x11, 0x00, 0x01, 0x11, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x11, 0x01, 0x10, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x10,
    0x00, 0x10, 0x00, 0x10, 0x00, 0x01, 0x11, 0x00, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x11,
    0x10, 0x01, 0x11, 0x10, 0x11, 0x01, 0x10, 0x01, 0x01, 0x00, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10,
    0x11, 0x11, 0x10, 0x11, 0x01, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x01, 0x11, 0x00, 0x11,
    0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x00, 0x01, 0x00, 0x01, 0x01, 0x00, 0x00, 0x10,
    0x00, 0x01, 0x11, 0x10, 0x10, 0x00, 0x10, 0x11, 0x01, 0x10, 0x00, 0x10, 0x00, 0x01, 0x01, 0x00,
    0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x10, 0x00, 0x10, 0x11, 0x11, 0x10, 0x10,
    0x10, 0x10, 0x01, 0x11, 0x00, 0x01, 0x11, 0x00, 0x00, 0x10, 0x00, 0x11, 0x11, 0x10, 0x00, 0x10,
    0x00, 0x01, 0x00, 0x10, 0x00, 0x10, 0x00, 0x00, 0x10, 0x00, 0x11, 0x01, 0x10, 0x00, 0x00, 0x00,
    0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x01, 0x11, 0x00, 0x11,
    0x11, 0x10, 0x11, 0x11, 0x10, 0x10, 0x10, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x10,
    0x00, 0x11, 0x01, 0x10, 0x10, 0x00, 0x10, 0x11, 0x01, 0x10, 0x00, 0x10, 0x00, 0x01, 0x01, 0x00,
    0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x01, 0x01, 0x00, 0x01,
    0x11, 0x00, 0x01, 0x01, 0x00, 0x10, 0x10, 0x10, 0x11, 0x11, 0x10, 0x11, 0x11, 0x10, 0x11, 0x10,
    0x00, 0x11, 0x01, 0x10, 0x11, 0x01, 0x10, 0x01, 0x01, 0x00, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];


