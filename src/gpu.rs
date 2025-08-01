pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;
const TILE_DATA_SIZE: usize = 0x1800;

#[derive(Copy,Clone)]
enum TilePixelValue {
    Zero,   // Black
    One,    // Dark Gray
    Two,    // Light Gray
    Three,  // White
}

type Tile = [[TilePixelValue; 8]; 8];
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}

impl GPU {
    pub fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }
    pub fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;
        // check bounds for tile decoding
        if index >= TILE_DATA_SIZE { return }

        // normalize index by setting lsb to 0
        let index = index & 0xFFFE;
        let byte1 = self.vram[index];
        let byte2 = self.vram[index + 1];

        // entire tile is 8 rows, therefore 16 bytes
        let tile_index = index / 16;
        // since every 2 bytes is a new row
        let row_index = (index % 16) / 2;

        for pixel_index in 0..8 {
            /* bits representing pixels are as such:
                1111_1111
                0123 4567

                meaning msb from each byte combine to represent the first pixel in the row
                while lsb from each byte combine to represent the last pixel in the row
            */ 
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;
            let value = match (msb != 0, lsb != 0) {
                (true, true) => TilePixelValue::Three,
                (true, false) => TilePixelValue::Two,
                (false, true) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero,
            };
            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }
}