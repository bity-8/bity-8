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

// Hardware Config Locations
pub const OFF_HARD_PAL: MemLoc = (0x00..0x30); // Multicart

// Helper funcs for getting certain areas of the cartridge.
pub fn get_area(loc: MemLoc) -> &'static mut [i8] {
    unsafe {
        &mut MEM[loc]
    }
}

pub fn get_sub_area(loc: MemLoc, off: MemLoc) -> &'static mut [i8] {
    unsafe {
        &mut MEM[(loc.start + off.start..loc.start + off.end)]
    }
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
pub fn map_val(start: usize, len: usize, val: i8) {
    for i in start..len {
        unsafe { MEM[i] = val; }
    }
}

// Read a memory location
pub fn peek(loc: usize) -> i8 {
    if loc < CART_LEN {
        unsafe { MEM[loc] }
    } else {
        0
    }
}

// Write to a memory location
pub fn poke(loc: usize, val: i8) {
    if loc < CART_LEN {
        unsafe { MEM[loc] = val; }
    }
}

// This is used when you load a new cartridge or reset your cartridge.
// But NOT when you reboot.
pub fn reset_memory() {
    map_val(0, LOC_MULT.start, 0)
}

pub fn reset_all_memory() {
    map_val(0, CART_LEN, 0)
}

#[test]
fn test_mem() {
    reset_all_memory();

    {
        let x = get_loc(LOC_CART);
        x[3] = 22;
    }

    map_vector(0, 20, &[2, 4, 5]);

    assert!(peek(0) == 2);
    assert!(peek(1) == 4);
    assert!(peek(2) == 5);
    assert!(peek(3) == 22);

    for i in 4..CART_LEN {
        assert!(peek(i) == 0);
    }
}
