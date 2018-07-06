// Trait
trait Tile {
    fn new(sprite_x: u8, sprite_y: u8) -> Self;

    // Return copy of sprite stored at referenced coordinates from spritesheet
    fn get_sprite(&spritesheet, Spritesheet) -> Sprite;
}

// Abstraction
struct AbstractTile {
    pub sprite_x: u8,
    pub sprite_y: u8,
}

impl Tile for AbstractTile {
    // Implement trait here...
}

// Raw bytes
struct RawTile {
    byte: u8,
}

impl Tile for RawTile {
    // Implement trait here...
}
