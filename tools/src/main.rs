extern crate bity_8_tools;
extern crate image;

use bity_8_tools::spritesheet::Spritesheet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <in> <out>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let image = get_image(&input_path);

    let spritesheet = Spritesheet::from_image(image).unwrap_or_else(|err| {
        println!("ERROR: Could not create spritesheet from image");
        println!("{}", err);
        process::exit(1);
    });

    save_spritesheet(&spritesheet, &output_path);
}

fn get_image(path: &str) -> image::RgbImage {
    let loaded_image = image::open(path).unwrap_or_else(|err| {
        println!("ERROR: Could not open image");
        println!("{}", err);
        process::exit(1);
    });

    let image = loaded_image.to_rgb();

    image
}

fn save_spritesheet(spritesheet: &Spritesheet, path: &str) {
    let mut file = File::create(&path).unwrap_or_else(|err| {
        println!("ERROR: Could not open file to write to");
        println!("{}", err);
        process::exit(1);
    });

    file.write_all(&spritesheet.bytes).unwrap_or_else(|err| {
        println!("ERROR: Could not write to file");
        println!("{}", err);
        process::exit(1);
    });
}
