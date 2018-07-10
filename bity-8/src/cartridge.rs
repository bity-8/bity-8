// NOTE: FOR JOSH
// This is what I started doing for reading the cartridge. You can change whatever if you wanna
// make tools or sumthin here. You prob know what you want better than me.

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::str;

use memory as mem;

const HEADER_SIZE: usize = 0x20;

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

    // By default, code goes until beginning of sprite data
    let mut offset_buffer =
        mem::get_sub_area(mem::LOC_CART, mem::OFF_SPRITE_DATA);
    let mut end = offset_buffer[2] as usize;
    end |= (offset_buffer[1] as usize) << 8;
    end |= (offset_buffer[0] as usize) << 16;

    // If sprite data doesn't exist, code goes until beginning of tile data
    if end == 0 {
        offset_buffer = mem::get_sub_area(mem::LOC_CART, mem::OFF_TILE_DATA);
        end = offset_buffer[2] as usize;
        end |= (offset_buffer[1] as usize) << 8;
        end |= (offset_buffer[0] as usize) << 16;
    }

    // If tile data doesn't exist, code goes until beginning of audio data
    if end == 0 {
        offset_buffer = mem::get_sub_area(mem::LOC_CART, mem::OFF_AUDIO_DATA);
        end = offset_buffer[2] as usize;
        end |= (offset_buffer[1] as usize) << 8;
        end |= (offset_buffer[0] as usize) << 16;
    }

    // If audio data doesn't exist, code goes until end of cartridge file
    let cartridge_buffer = mem::get_area(mem::LOC_CART);
    if end == 0 {
        // Code is the only thing in the cartridge, so either a null character
        // or the end of the cartridge will indicate the end of the code section
        end = cartridge_buffer.len();
        for i in HEADER_SIZE..end {
            if cartridge_buffer[i] == 0 {
                end = i;
                break;
            }
        }
    }

    // Ignore header and only read up to end of code section
    let s = str::from_utf8(&cartridge_buffer[HEADER_SIZE..end]).unwrap();
    let mut s = String::from(s);
    println!("{}", s);
    s
}
