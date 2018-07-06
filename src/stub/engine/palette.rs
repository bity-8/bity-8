// Trait
trait Palette {
    fn new() -> Self;

    fn from_palette(palette: &Palette) -> Self;

    // Handle errors for failed reads and invalid inputs
    fn from_image(image: &image::RgbImage) -> Result<Self, String>;

    fn to_abstract_palette(&self) -> AbstractPalette;

    fn to_raw_palette(&self) -> RawPalette;

    // Returns Some(u8) if it exists, None if it doesn't
    fn index_of(&self, color: &RgbColor) -> Option<u8>;
}

// Abstraction
struct AbstractPalette {
    colors: [RgbColor; PALETTE_SIZE],
}

impl Palette for AbstractPalette {
    // Implement trait here...
}

// Raw bytes
struct RawPalette {
    bytes: [u8; PALETTE_BYTE_COUNT],
}

impl Palette for RawPalette {
    // Implement trait here...
}
