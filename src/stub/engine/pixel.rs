// Pixel
struct Pixel {
    pub palette_index: u8,
}

impl Pixel {
    fn new(palette_index: u8) -> Pixel;

    fn to_rgb_pixel(palette: &Palette) -> RgbPixel;
}
