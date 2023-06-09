use crate::z80::{Z80Memory, Z80};

static ROM_48: &[u8; 0x4000] = include_bytes!("48.rom");
// static ROM_128_0: [u8] = include_bytes!("128-0.rom");
// static ROM_128_1: [u8] = include_bytes!("128-1.rom");

struct ZxSpectrumMemory {
    rom: &'static [u8; 0x4000],
    ram: [u8; 0x8000],
}

impl ZxSpectrumMemory {
    fn new(rom: &'static [u8; 0x4000], ram: [u8; 0x8000]) -> ZxSpectrumMemory {
        ZxSpectrumMemory { rom, ram }
    }
}

impl Z80Memory for ZxSpectrumMemory {
    fn read(&self, address: u16) -> u8 {
        let index = address as usize;
        if index < self.rom.len() {
            u8::from_le(self.rom[index])
        } else {
            u8::from_le(self.ram[index - self.rom.len()])
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        let index = address as usize;
        if index >= self.rom.len() {
            self.ram[index - self.rom.len()] = data.to_le();
        }
    }
}

pub struct ZxSpectrum {
    memory: ZxSpectrumMemory,
    processor: Z80,
}

impl ZxSpectrum {
    pub fn new() -> ZxSpectrum {
        ZxSpectrum {
            memory: ZxSpectrumMemory::new(ROM_48, [0; 0x8000]),
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
