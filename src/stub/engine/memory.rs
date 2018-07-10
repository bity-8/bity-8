struct Memory {
    bytes: [u8; MEMORY_SIZE],
}

impl Memory {
    fn new() -> Memory;

    // Memory layout
    fn insert_cartridge(cartridge: &RawCartridge);
    fn insert_palette(palette: &RawPalette);
    fn insert_spritesheet(spritesheet: &RawSpritesheet, index: u8);
    fn insert_tilemap(tilemap: &RawTilemap, index: u8);

    // Utilty functions
    fn draw_sprite(src_x: u8, src_y: u8, dst_x: u8, dst_y: u8);
    fn draw_tile(x: u8, y: u8, tilemap_index: u8, spritesheet_index: u8);
    fn get_memory(address: u32, count: u32);
    fn write_memory(buffer: [u8]);
    fn get_framebuffer() -> Framebuffer;
}
