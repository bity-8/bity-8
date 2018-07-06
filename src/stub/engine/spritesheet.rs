// Trait
trait Spritesheet {
    fn new() -> Self;

    fn from_spritesheet(&sheet: Spritesheet) -> Self;

    fn to_abstract_spritesheet(&self) -> AbstractSpritesheet;

    fn to_raw_spritesheet(&self) -> RawSpritesheet;

    fn get_sprite_at(x: u8, y: u8) -> Sprite;
}

// Abstraction
struct AbstractSpritesheet {
    pixels: [Pixel; SPRITESHEET_SIZE],
}

impl Spritesheet for AbstractSpritesheet {
    // Implement trait here...
}

// Raw bytes
struct RawSpritesheet {
    bytes: [u8; SPRITESHEET_BYTE_COUNT],
}

impl Spritesheet for RawSpritesheet {
    // Implement trait here...
}
