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
pub const LOC_SAVE: MemLoc = (0x4DC00..0x4E000); // Save
pub const LOC_EMPT: MemLoc = (0x4E000..0x4F800); // Empty
pub const LOC_MULT: MemLoc = (0x4F800..0x50000); // Multicart

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
pub fn map_vector( start: usize, len: usize, vec: &[i8]) {
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
