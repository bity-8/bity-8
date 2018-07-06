// Trait
trait Sprite {
    fn new -> Self;

    fn from_sprite(sprite: &Sprite) -> Self;

    fn to_abstract_sprite(&self) -> AbstractSprite;

    fn to_raw_sprite(&self) -> RawSprite;

    // Copy sprite to framebuffer at coordinates (x, y)
    fn copy_to(&self, fb: &mut [u8], x: u8, y: u8);
}

// Abstraction
struct AbstractSprite {
    pixels: [Pixel; SPRITE_SIZE],
    spritesheet_index: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    transparent_color: Option<Pixel>,
}

impl Sprite for AbstractSprite {
    // Implement trait here...
}

// Raw bytes
struct RawSprite {
    bytes: [u8; SPRITE_BYTE_COUNT],
    spritesheet_index: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    transparent_color: Option<Pixel>,
}

impl Sprite for RawSprite {
    // Implement trait here...
}
