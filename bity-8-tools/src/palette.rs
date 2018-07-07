extern crate image;

const PALETTE_PIXEL_WIDTH: usize = 4;
const PALETTE_PIXEL_HEIGHT: usize = 4;

pub struct Palette {
    colors: [image::Rgb<u8>; 16],
}

impl Palette {
    pub fn from_image(image: &image::RgbImage) -> Result<Palette, String> {
        let width = image.width() as usize;
        let height = image.height() as usize;
        if width < PALETTE_PIXEL_WIDTH || height < PALETTE_PIXEL_HEIGHT {
            return Err(format!(
                "Palette image dimensions must be at least {}w x {}h pixels",
                PALETTE_PIXEL_WIDTH,
                PALETTE_PIXEL_HEIGHT,
            ))
        }

        Ok(Palette {
            colors: [
                image.get_pixel(0, 0).clone(),
                image.get_pixel(1, 0).clone(),
                image.get_pixel(2, 0).clone(),
                image.get_pixel(3, 0).clone(),
                image.get_pixel(0, 1).clone(),
                image.get_pixel(1, 1).clone(),
                image.get_pixel(2, 1).clone(),
                image.get_pixel(3, 1).clone(),
                image.get_pixel(0, 2).clone(),
                image.get_pixel(1, 2).clone(),
                image.get_pixel(2, 2).clone(),
                image.get_pixel(3, 2).clone(),
                image.get_pixel(0, 3).clone(),
                image.get_pixel(1, 3).clone(),
                image.get_pixel(2, 3).clone(),
                image.get_pixel(3, 3).clone(),
            ],
        })
    }

    pub fn index_of(&self, color: &image::Rgb<u8>) -> Option<u8> {
        for (i, palette_color) in self.colors.iter().enumerate() {
            if color[0] == palette_color[0]
                && color[1] == palette_color[1]
                && color[2] == palette_color[2]
            {
                return Some(i as u8)
            }
        }
        None
    }
}
