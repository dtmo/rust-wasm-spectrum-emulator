use crate::z80::{MemoryAccessor, Z80};

static ROM_48: &[u8; 0x4000] = include_bytes!("48.rom");
// static ROM_128_0: [u8] = include_bytes!("128-0.rom");
// static ROM_128_1: [u8] = include_bytes!("128-1.rom");

struct Memory {
    rom: &'static [u8; 0x4000],
    ram: [u8; 0x8000],
}

impl Memory {
    fn new(rom: &'static [u8; 0x4000], ram: [u8; 0x8000]) -> Memory {
        Memory { rom, ram }
    }
}

impl MemoryAccessor for Memory {
    fn read(&self, address: &u16) -> u8 {
        let index = *address as usize;
        if index < self.rom.len() {
            u8::from_le(self.rom[index])
        } else {
            u8::from_le(self.ram[index - self.rom.len()])
        }
    }

    fn write(&mut self, address: &u16, data: &u8) {
        let index = *address as usize;
        if index >= self.rom.len() {
            self.ram[index - self.rom.len()] = data.to_le();
        }
    }
}

pub struct ZxSpectrum {
    memory: Memory,
    processor: Z80,
}

impl ZxSpectrum {
    pub fn new() -> ZxSpectrum {
        ZxSpectrum {
            memory: Memory::new(ROM_48, [0; 0x8000]),
            processor: Z80::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialise() {
        let computer = ZxSpectrum::new();

        assert_eq!(ROM_48, computer.memory.rom);
        assert!(computer.memory.ram.iter().all(|&b| b == 0));
    }
}
