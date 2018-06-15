use std::cmp;

pub type Memory = [i8; LOC_END];
static mut MEM: Memory = [0; LOC_END];

// constants
pub const LOC_CART: usize = 0x00000; // Cartridge
pub const LOC_HARD: usize = 0x40000; // Hardware Config
pub const LOC_SCRE: usize = 0x40400; // Screen Buffer
pub const LOC_SPRI: usize = 0x43A00; // Sprite Sheets
pub const LOC_TILE: usize = 0x47000; // Tile Maps
pub const LOC_SAVE: usize = 0x4DC00; // Save
pub const LOC_EMPT: usize = 0x4E000; // Empty
pub const LOC_MULT: usize = 0x4F800; // Multicart
pub const LOC_END : usize = 0x50000; // End of cartridge

// Hardware Config Locations
pub fn get_hard_pal() -> &'static mut [i8] { unsafe { &mut MEM[LOC_HARD..LOC_HARD+0x30] } }

// Helper funcs for getting certain areas of the cartridge.
// See how to use these in the test below.
pub fn get_cart() -> &'static mut [i8] { unsafe { &mut MEM[LOC_CART..LOC_HARD] } }
pub fn get_hard() -> &'static mut [i8] { unsafe { &mut MEM[LOC_HARD..LOC_SCRE] } }
pub fn get_scre() -> &'static mut [i8] { unsafe { &mut MEM[LOC_SCRE..LOC_SPRI] } }
pub fn get_spri() -> &'static mut [i8] { unsafe { &mut MEM[LOC_SPRI..LOC_TILE] } }
pub fn get_tile() -> &'static mut [i8] { unsafe { &mut MEM[LOC_TILE..LOC_SAVE] } }
pub fn get_save() -> &'static mut [i8] { unsafe { &mut MEM[LOC_SAVE..LOC_EMPT] } }
pub fn get_empt() -> &'static mut [i8] { unsafe { &mut MEM[LOC_EMPT..LOC_MULT] } }
pub fn get_mult() -> &'static mut [i8] { unsafe { &mut MEM[LOC_MULT..LOC_END ] } }

// Maps a vector to memory. This is safe if you mess up the parameters.
pub fn map_vector( start: usize, len: usize, vec: &[i8]) {
    let len = cmp::min(vec.len(), len);

    for vec_ind in 0..len {
        let mem_ind = start+vec_ind;
        if mem_ind >= LOC_END { return }
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
    if loc < LOC_END {
        unsafe { MEM[loc] }
    } else {
        0
    }
}

// Write to a memory location
pub fn poke(loc: usize, val: i8) {
    if loc < LOC_END {
        unsafe { MEM[loc] = val; }
    }
}

// This is used when you load a new cartridge or reset your cartridge.
// But NOT when you reboot.
pub fn reset_memory() {
    map_val(0, LOC_MULT, 0)
}

pub fn reset_all_memory() {
    map_val(0, LOC_END, 0)
}

#[test]
fn test_mem() {
    reset_all_memory();

    {
        let x = get_cart();
        x[3] = 22;
    }

    map_vector(0, 20, &[2, 4, 5]);

    assert!(peek(0) == 2);
    assert!(peek(1) == 4);
    assert!(peek(2) == 5);
    assert!(peek(3) == 22);

    for i in 4..LOC_EMPT {
        assert!(peek(i) == 0);
    }
}
