extern crate image;

use palette::Palette;

// Spritesheet dimensions
pub const SPRITESHEET_PIXEL_WIDTH: usize = 96;
pub const SPRITESHEET_PIXEL_HEIGHT: usize = 72;
pub const SPRITESHEET_PIXEL_COUNT: usize
    = SPRITESHEET_PIXEL_WIDTH * SPRITESHEET_PIXEL_HEIGHT;
pub const SPRITESHEET_BYTE_WIDTH: usize = SPRITESHEET_PIXEL_WIDTH / 2;
pub const SPRITESHEET_BYTE_HEIGHT: usize = SPRITESHEET_PIXEL_HEIGHT;
pub const SPRITESHEET_BYTE_COUNT: usize
    = SPRITESHEET_BYTE_WIDTH * SPRITESHEET_BYTE_HEIGHT;

fn create_rgb_pixel(r: u8, g: u8, b: u8) -> image::Rgb<u8> {
    image::Rgb {
        data: [r, g, b],
    }
}

pub struct Spritesheet {
    pub bytes: [u8; SPRITESHEET_BYTE_COUNT],
}

impl Spritesheet {
    fn new() -> Spritesheet {
        Spritesheet {
            bytes: [0; SPRITESHEET_BYTE_COUNT],
        }
    }

    pub fn from_image(image: image::RgbImage) -> Result<Spritesheet, String> {
        let width = image.width() as usize;
        let height = image.height() as usize;
        if width != SPRITESHEET_PIXEL_WIDTH || height != SPRITESHEET_PIXEL_HEIGHT {
            return Err(format!(
                "Spritesheet must have dimensions of {}w x {}h pixels",
                SPRITESHEET_PIXEL_WIDTH,
                SPRITESHEET_PIXEL_HEIGHT,
            ))
        }
        let palette = Palette::from_image(&image)?;

        let image_buffer = image.into_raw();

        let mut spritesheet = Spritesheet::new();

        // Spritesheet stores pixels as nibbles, so there are half
        // as many bytes as pixels. Because of this, we will iterate
        // through the bytes and fill them with two pixels at a time.
        let mut invalid_pixels = Vec::new();
        for (i, byte) in spritesheet.bytes.iter_mut().enumerate() {
            let i = i * 2;

            // High pixel
            let image_buffer_offset = i * 3;
            let r = image_buffer[image_buffer_offset];
            let g = image_buffer[image_buffer_offset + 1];
            let b = image_buffer[image_buffer_offset + 2];
            let rgb_pixel_high = create_rgb_pixel(r, g, b);

            // Low pixel
            let image_buffer_offset = (i + 1) * 3;
            let r = image_buffer[image_buffer_offset];
            let g = image_buffer[image_buffer_offset + 1];
            let b = image_buffer[image_buffer_offset + 2];
            let rgb_pixel_low = create_rgb_pixel(r, g, b);

            // u8 value stored in low nibble
            let spritesheet_pixel_value_high
                = palette.index_of(&rgb_pixel_high);
            let spritesheet_pixel_value_low
                = palette.index_of(&rgb_pixel_low);

            // Store invalid pixel indices for error reporting
            if spritesheet_pixel_value_high.is_none() {
                let x = i % SPRITESHEET_PIXEL_WIDTH;
                let y = i / SPRITESHEET_PIXEL_WIDTH;
                invalid_pixels.push((x, y));
            }
            if spritesheet_pixel_value_low.is_none() {
                let x = (i + 1) % SPRITESHEET_PIXEL_WIDTH;
                let y = (i + 1) / SPRITESHEET_PIXEL_WIDTH;
                invalid_pixels.push((x, y));
            }

            // Default value of 0 for invalid pixels allows us to get a list of
            // all invalid pixels at once
            let spritesheet_pixel_value_high
                = spritesheet_pixel_value_high.unwrap_or(0);
            let spritesheet_pixel_value_low
                = spritesheet_pixel_value_low.unwrap_or(0);

            // Store pixel in spritesheet
            let high_nibble = spritesheet_pixel_value_high << 4;
            *byte = high_nibble | spritesheet_pixel_value_low;
        }

        if !invalid_pixels.is_empty() {
            return Err(format!(
                "Pixels in spritesheet do not match palette: {:?}",
                invalid_pixels,
            ))
        }

        Ok(spritesheet)
    }
}
