use crate::cartridge::Cartridge;
use crate::gpu::Gpu;

pub const CARTRIDGE_ROM_START: u16 = 0x0000;
pub const CARTRIDGE_ROM_END: u16 = 0x7FFF;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;

pub const CARTRIDGE_RAM_START: u16 = 0xA000;
pub const CARTRIDGE_RAM_END: u16 = 0xBFFF;

pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xDFFF;

pub const BOOT_ROM_END: u16 = 0x100;

/*
  0000-3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)
  4000-7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)
  8000-9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
  A000-BFFF   8KB External RAM     (in cartridge, switchable bank, if any)
  C000-CFFF   4KB Work RAM Bank 0 (WRAM)
  D000-DFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)
  E000-FDFF   Same as C000-DDFF (ECHO)    (typically not used)
  FE00-FE9F   Sprite Attribute Table (OAM)
  FEA0-FEFF   Not Usable
  FF00-FF7F   I/O Ports
  FF80-FFFE   High RAM (HRAM)
  FFFF        Interrupt Enable Register
*/

pub struct MemoryBus {
    cartridge: Cartridge,
    gpu: Gpu,
}

impl MemoryBus {
    pub fn new(rom_data: Vec<u8>) -> Self {
        let cartridge = Cartridge::build(rom_data);

        Self {
            cartridge,
            gpu: Gpu::new(),
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END => {
                // read from CARTRIDGE_ROM
                self.cartridge.read(address)
            }
            /*
            VRAM_START..=VRAM_END => {
                // read from VRAM
            }
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => {
                // read from CARTRIDGE_RAM
            }
            WRAM_START..=WRAM_END => {
                // read from WRAM
            }
            */
            _ => {
                println!("Unknown address: Can't read byte.");
                0x00
            },
        }
    }
}