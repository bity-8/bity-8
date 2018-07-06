// Trait
trait Tilemap {
    fn new() -> Self;

    fn from_tilemap(&tilemap: Tilemap) -> Self {

    fn to_abstract_tilemap(&self) -> AbstractTilemap;

    fn to_raw_tilemap(&self) -> RawTilemap;

    fn get_sprite_coordinates_at(x: u8, y: u8) -> (u8, u8);

    fn set_sprite_coordinates_at(x: u8, y: u8, tile: Tile);
}

// Abstraction
struct AbstractTilemap {
    tiles: [Tile; TILEMAP_SIZE],
}

impl Tilemap for AbstractTilemap {
    // Implement trait here...
}

// Raw bytes
struct RawTilemap {
    bytes: [u8; TILEMAP_BYTE_COUNT],
}

impl Tilemap for RawTilemap {
    // Implement trait here...
}
