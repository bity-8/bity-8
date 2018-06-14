use std::cmp;

pub type Memory = [i8; 0x50000];

pub static LOC_CART: usize = 0x00000; // Cartridge
pub static LOC_HARD: usize = 0x40000; // Hardware Config
pub static LOC_SCRE: usize = 0x40400; // Screen Buffer
pub static LOC_SPRI: usize = 0x43A00; // Sprite Sheets
pub static LOC_TILE: usize = 0x47000; // Tile Maps
pub static LOC_SAVE: usize = 0x4DC00; // Save
pub static LOC_EMPT: usize = 0x4E000; // Empty
pub static LOC_MULT: usize = 0x4F800; // Multicart
pub static LOC_END : usize = 0x50000; // Multicart

// Gotta do this first.
pub fn create_memory() -> Memory {
    [0; 0x50000]
}

// Maps a vector to memory. This is safe.
pub fn map_vector(mem: &mut Memory, start: usize, len: usize, vec: &[i8]) {
    let len = cmp::min(vec.len(), len);

    for vec_ind in 0..len {
        let mem_ind = start+vec_ind;
        if mem_ind >= LOC_END { return }
        mem[mem_ind] = vec[vec_ind];
    }
}

// Maps one value to memory.
pub fn map_memory(mem: &mut Memory, start: usize, len: usize, val: i8) {
    for i in start..len {
        mem[i] = val;
    }
}

// This is used when you load a new cartridge or reset your cartridge.
// But NOT when you reboot.
pub fn reset_memory(mem: &mut Memory) {
    map_memory(mem, 0, LOC_MULT, 0)
}

pub fn reset_all_memory(mem: &mut Memory) {
    map_memory(mem, 0, LOC_END, 0)
}

#[test]
fn test_mem() {
    let mut mem = create_memory();

    reset_memory(&mut mem);
    map_vector(&mut mem, 0, 20, &[2, 4, 5]);

    assert!(mem[0] == 2);
    assert!(mem[1] == 4);
    assert!(mem[2] == 5);
    for i in 3..LOC_EMPT {
        assert!(mem[i] == 0);
    }
}
