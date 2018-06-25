use std::cmp;
use std::ops::Range;

pub type Memory = [i8; CART_LEN];
pub type MemLoc = Range<usize>;

pub const CART_LEN : usize = 0x50000; // End of cartridge
static mut MEM: Memory = [0; CART_LEN];

// Global Memory Constants
pub const LOC_CART: MemLoc = (0x00000..0x40000); // Cartridge
pub const LOC_HARD: MemLoc = (0x40000..0x40400); // Hardware Config
pub const LOC_SCRE: MemLoc = (0x40400..0x43A00); // Screen Buffer
pub const LOC_SPRI: MemLoc = (0x43A00..0x47000); // Sprite Sheets
pub const LOC_TILE: MemLoc = (0x47000..0x4DC00); // Tile Maps
pub const LOC_INST: MemLoc = (0x4DC00..0x4E000); // Instruments
pub const LOC_SAVE: MemLoc = (0x4E000..0x4E400); // Save
pub const LOC_EMPT: MemLoc = (0x4E400..0x4F800); // Empty
pub const LOC_MULT: MemLoc = (0x4F800..0x50000); // Multicart

// 8 Instruments
pub const LOC_INS1: MemLoc = (0x4DC00..0x4DC80);
pub const LOC_INS2: MemLoc = (0x4DC80..0x4DD00);
pub const LOC_INS3: MemLoc = (0x4DD00..0x4DD80);
pub const LOC_INS4: MemLoc = (0x4DD80..0x4DE00);
pub const LOC_INS5: MemLoc = (0x4DE00..0x4DE80);
pub const LOC_INS6: MemLoc = (0x4DE80..0x4DF00);
pub const LOC_INS7: MemLoc = (0x4DF00..0x4DF80);
pub const LOC_INS8: MemLoc = (0x4DF80..0x4E000);

pub const LOC_ALL:        MemLoc = (0x00000..0x50000);
pub const LOC_REBOOTABLE: MemLoc = (0x00000..0x4F800); // Memory that gets reset
pub const LOC_ROM:        MemLoc = (0x00000..0x40000);
pub const LOC_WRITABLE:   MemLoc = (0x40000..0x50000);

// Hardware Config Locations
pub const OFF_HARD_PAL: MemLoc = (0x00..0x30); // Multicart

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
pub fn get_area(area: MemLoc) -> &'static mut [i8] {
    unsafe { &mut MEM[area] }
}

pub fn get_sub_area(area: MemLoc, off: MemLoc) -> &'static mut [i8] {
    unsafe { &mut MEM[add_mems(area, off)] }
}

// Maps a vector to memory. This is safe if you mess up the parameters.
pub fn map_vector(start: usize, len: usize, vec: &[i8]) {
    let len = cmp::min(vec.len(), len);

    for vec_ind in 0..len {
        let mem_ind = start+vec_ind;
        if mem_ind >= CART_LEN { return }
        unsafe { MEM[mem_ind] = vec[vec_ind]; }
    }
}

// Maps one value to memory.
// You can use this for clearing memory.
fn mset(pos: usize, len: usize, val: i8, area: MemLoc) {
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
fn poke(pos: usize, val: i8, area: MemLoc) {
    if pos_in_bounds(pos, area) {
        unsafe { MEM[pos] = val; }
    }
}

// Read a memory location
pub fn peek(pos: usize) -> i8 {
    if pos < CART_LEN {
        unsafe { MEM[pos] }
    } else {
        0
    }
}

// for short/writable areas.
pub fn poke_w(pos: usize, val: i8)  { poke(pos, val, LOC_WRITABLE); }
pub fn mset_w(pos: usize, len: usize, val: i8) { mset(pos, len, val, LOC_WRITABLE); }
pub fn mcpy_w(dest: usize, pos: usize, len: usize) { mcpy(dest, pos, len, LOC_WRITABLE); }

pub fn poke_a(pos: usize, val: i8)  { poke(pos, val, LOC_ALL); }
pub fn mset_a(pos: usize, len: usize, val: i8) { mset(pos, len, val, LOC_ALL); }
pub fn mcpy_a(dest: usize, pos: usize, len: usize) { mcpy(dest, pos, len, LOC_ALL); }

pub fn reset_memory() {
    mset_a(0, CART_LEN, 0);
    map_vector(LOC_INS1.start, 128, &SQUARE_WAVE);
    map_vector(LOC_INS2.start, 128, &SAWTOOTH_WAVE);
    map_vector(LOC_INS3.start, 128, &TRIANGLE_WAVE);
    map_vector(LOC_INS4.start, 128, &NOISE_WAVE);
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

// Some defaults:
const INSTRUMENT_LEN:   usize = 128;
pub type Instrument = [i8; INSTRUMENT_LEN];

// used python to generate these :).
pub const SQUARE_WAVE: [i8; INSTRUMENT_LEN] =
[
    127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,
    127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,
    127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,
    127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,  127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
];

// [(i - math.floor(i * 1/2) ) * 4 - 128 for i in range(128)]
pub const SAWTOOTH_WAVE: [i8; INSTRUMENT_LEN] =
[
    -127, -124, -124, -120, -120, -116, -116, -112, -112, -108, -108, -104, -104, -100, -100, -96,
    -96, -92, -92, -88, -88, -84, -84, -80, -80, -76, -76, -72, -72, -68, -68, -64, -64, -60, -60,
    -56, -56, -52, -52, -48, -48, -44, -44, -40, -40, -36, -36, -32, -32, -28, -28, -24, -24, -20,
    -20, -16, -16, -12, -12, -8, -8, -4, -4, 0, 0, 4, 4, 8, 8, 12, 12, 16, 16, 20, 20, 24, 24, 28,
    28, 32, 32, 36, 36, 40, 40, 44, 44, 48, 48, 52, 52, 56, 56, 60, 60, 64, 64, 68, 68, 72, 72, 76,
    76, 80, 80, 84, 84, 88, 88, 92, 92, 96, 96, 100, 100, 104, 104, 108, 108, 112, 112, 116, 116,
    120, 120, 124, 124, 127
];

// [abs((i - math.floor(i * 1/2) ) * 4 - 128) * 2 - 128 for i in range(128)]
pub const TRIANGLE_WAVE: [i8; INSTRUMENT_LEN] =
[
    127, 120, 120, 112, 112, 104, 104, 96, 96, 88, 88, 80, 80, 72, 72, 64, 64, 56, 56, 48, 48, 40,
    40, 32, 32, 24, 24, 16, 16, 8, 8, 0, 0, -8, -8, -16, -16, -24, -24, -32, -32, -40, -40, -48,
    -48, -56, -56, -64, -64, -72, -72, -80, -80, -88, -88, -96, -96, -104, -104, -112, -112, -120,
    -120, -127, -127, -120, -120, -112, -112, -104, -104, -96, -96, -88, -88, -80, -80, -72, -72,
    -64, -64, -56, -56, -48, -48, -40, -40, -32, -32, -24, -24, -16, -16, -8, -8, 0, 0, 8, 8, 16,
    16, 24, 24, 32, 32, 40, 40, 48, 48, 56, 56, 64, 64, 72, 72, 80, 80, 88, 88, 96, 96, 104, 104,
    112, 112, 120, 120, 127
];

pub const NOISE_WAVE: [i8; INSTRUMENT_LEN] =
[
    -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128,
    -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128,
    -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128,
    -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128,
    -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128,
    -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128,
    -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128,
    -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128,
];

