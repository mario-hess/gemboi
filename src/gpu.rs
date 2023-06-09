pub const VRAM_SIZE: usize = 8192;

pub struct Gpu {
    tile_set: [[u8; 8]; 384],
    video_ram: [u8; VRAM_SIZE],
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            tile_set: [[0; 8]; 384],
            video_ram: [0; VRAM_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.video_ram[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.video_ram[address as usize] = value;
    }
}
