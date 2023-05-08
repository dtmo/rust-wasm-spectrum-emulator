// Official Z80 documentation: https://www.zilog.com/docs/z80/um0080.pdf
// Unofficial undocumented functionality documentation: http://www.z80.info/zip/z80-documented.pdf
// Integration test suites: https://mdfs.net/Software/Z80/Exerciser/Spectrum/

use crate::flag_register::{FlagRegister, C_FLAG_BITMASK, H_FLAG_BITMASK};
use crate::register::Register;

pub trait MemoryAccessor {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

const OPCODE_FUNCTIONS: [fn(&mut Z80, &mut dyn MemoryAccessor) -> u8; 65] = [
    // 0b00000000 NOP
    |_, _| Z80::nop(),
    // 0b00000001
    // 0b00000010
    // 0b00000011
    // 0b00000100
    // 0b00000101
    // 0b00000110 LD B, n
    Z80::ld_b_n,
    // 0b00000111
    // 0b00001000
    // 0b00001001
    // 0b00001010
    // 0b00001011
    // 0b00001100
    // 0b00001101
    // 0b00001110 LD C, n
    Z80::ld_c_n,
    // 0b00001111
    // 0b00010000
    // 0b00010001
    // 0b00010010
    // 0b00010011
    // 0b00010100
    // 0b00010101
    // 0b00010110 LD D, n
    Z80::ld_d_n,
    // 0b00010111
    // 0b00011000
    // 0b00011001
    // 0b00011010
    // 0b00011011
    // 0b00011100
    // 0b00011101
    // 0b00011110 LD E, n
    Z80::ld_e_n,
    // 0b00011111
    // 0b00100000
    // 0b00100001
    // 0b00100010
    // 0b00100011
    // 0b00100100
    // 0b00100101
    // 0b00100110 LD H, n
    Z80::ld_h_n,
    // 0b00100111
    |z80, _| z80.daa(),
    // 0b00101000
    // 0b00101001
    // 0b00101010
    // 0b00101011
    // 0b00101100
    // 0b00101101
    // 0b00101110 LD L, n
    Z80::ld_l_n,
    // 0b00101111
    // 0b00110000
    // 0b00110001
    // 0b00110010
    // 0b00110011
    // 0b00110100
    // 0b00110101
    // 0b00110110
    // 0b00110111
    // 0b00111000
    // 0b00111001
    // 0b00111010
    // 0b00111011
    // 0b00111100
    // 0b00111101
    // 0b00111110 LD A, n
    Z80::ld_a_n,
    // 0b00111111
    // 0b01000000 LD B, B
    |z80, _| z80.ld_b_b(),
    // 0b01000001 LD B, C
    |z80, _| z80.ld_b_c(),
    // 0b01000010 LD B, D
    |z80, _| z80.ld_b_d(),
    // 0b01000011 LD B, E
    |z80, _| z80.ld_b_e(),
    // 0b01000100 LD B, H
    |z80, _| z80.ld_b_h(),
    // 0b01000101 LD B, L
    |z80, _| z80.ld_b_l(),
    // 0b01000110 LD B, (HL)
    |z80, memory_accessor| z80.ld_b_hl(memory_accessor),
    // 0b01000111 LD B, A
    |z80, _| z80.ld_b_a(),
    // 0b01001000 LD C, B
    |z80, _| z80.ld_c_b(),
    // 0b01001001 LD C, C
    |z80, _| z80.ld_c_c(),
    // 0b01001010 LD C, D
    |z80, _| z80.ld_c_d(),
    // 0b01001011 LD C, E
    |z80, _| z80.ld_c_e(),
    // 0b01001100 LD C, H
    |z80, _| z80.ld_c_h(),
    // 0b01001101 LD C, L
    |z80, _| z80.ld_c_l(),
    // 0b01001110 LD C, (HL)
    |z80, memory_accessor| z80.ld_c_hl(memory_accessor),
    // 0b01001111 LD C, A
    |z80, _| z80.ld_c_a(),
    // 0b01010000 LD D, B
    |z80, _| z80.ld_d_b(),
    // 0b01010001 LD D, C
    |z80, _| z80.ld_d_c(),
    // 0b01010010 LD D, D
    |z80, _| z80.ld_d_d(),
    // 0b01010011 LD D, E
    |z80, _| z80.ld_d_e(),
    // 0b01010100 LD D, H
    |z80, _| z80.ld_d_h(),
    // 0b01010101 LD D, L
    |z80, _| z80.ld_d_l(),
    // 0b01010110 LD D, (HL)
    |z80, memory_accessor| z80.ld_d_hl(memory_accessor),
    // 0b01010111 LD D, A
    |z80, _| z80.ld_d_a(),
    // 0b01011000 LD E, B
    |z80, _| z80.ld_e_b(),
    // 0b01011001 LD E, C
    |z80, _| z80.ld_e_c(),
    // 0b01011010 LD E, D
    |z80, _| z80.ld_e_d(),
    // 0b01011011 LD E, E
    |z80, _| z80.ld_e_e(),
    // 0b01011100 LD E, H
    |z80, _| z80.ld_e_h(),
    // 0b01011101 LD E, L
    |z80, _| z80.ld_e_l(),
    // 0b01011110 LD E, (HL)
    |z80, memory_accessor| z80.ld_e_hl(memory_accessor),
    // 0b01011111 LD E, A
    |z80, _| z80.ld_e_a(),
    // 0b01100000 LD H, B
    |z80, _| z80.ld_h_b(),
    // 0b01100001 LD H, C
    |z80, _| z80.ld_h_c(),
    // 0b01100010 LD H, D
    |z80, _| z80.ld_h_d(),
    // 0b01100011 LD H, E
    |z80, _| z80.ld_h_e(),
    // 0b01100100 LD H, H
    |z80, _| z80.ld_h_h(),
    // 0b01100101 LD H, L
    |z80, _| z80.ld_h_l(),
    // 0b01100110 LD H, (HL)
    |z80, memory_accessor| z80.ld_h_hl(memory_accessor),
    // 0b01100111 LD H, A
    |z80, _| z80.ld_h_a(),
    // 0b01101000 LD L, B
    |z80, _| z80.ld_l_b(),
    // 0b01101001 LD L, C
    |z80, _| z80.ld_l_c(),
    // 0b01101010 LD L, D
    |z80, _| z80.ld_l_d(),
    // 0b01101011 LD L, E
    |z80, _| z80.ld_l_e(),
    // 0b01101100 LD L, H
    |z80, _| z80.ld_l_h(),
    // 0b01101101 LD L, L
    |z80, _| z80.ld_l_l(),
    // 0b01101110 LD L, (HL)
    |z80, memory_accessor| z80.ld_l_hl(memory_accessor),
    // 0b01101111 LD L, A
    |z80, _| z80.ld_l_a(),
    // 0b01110000
    // 0b01110001
    // 0b01110010
    // 0b01110011
    // 0b01110100
    // 0b01110101
    // 0b01110110
    // 0b01110111
    // 0b01111000 LD A, B
    |z80, _| z80.ld_a_b(),
    // 0b01111001 LD A, C
    |z80, _| z80.ld_a_c(),
    // 0b01111010 LD A, D
    |z80, _| z80.ld_a_d(),
    // 0b01111011 LD A, E
    |z80, _| z80.ld_a_e(),
    // 0b01111100 LD A, H
    |z80, _| z80.ld_a_h(),
    // 0b01111101 LD A, L
    |z80, _| z80.ld_a_l(),
    // 0b01111110 LD A, (HL)
    |z80, memory_accessor| z80.ld_a_hl(memory_accessor),
    // 0b01111111 LD A, A
    |z80, _| z80.ld_a_a(),
    // 0b10000000
    // 0b10000001
    // 0b10000010
    // 0b10000011
    // 0b10000100
    // 0b10000101
    // 0b10000110
    // 0b10000111
    // 0b10001000
    // 0b10001001
    // 0b10001010
    // 0b10001011
    // 0b10001100
    // 0b10001101
    // 0b10001110
    // 0b10001111
    // 0b10010000
    // 0b10010001
    // 0b10010010
    // 0b10010011
    // 0b10010100
    // 0b10010101
    // 0b10010110
    // 0b10010111
    // 0b10011000
    // 0b10011001
    // 0b10011010
    // 0b10011011
    // 0b10011100
    // 0b10011101
    // 0b10011110
    // 0b10011111
    // 0b10100000
    // 0b10100001
    // 0b10100010
    // 0b10100011
    // 0b10100100
    // 0b10100101
    // 0b10100110
    // 0b10100111
    // 0b10101000
    // 0b10101001
    // 0b10101010
    // 0b10101011
    // 0b10101100
    // 0b10101101
    // 0b10101110
    // 0b10101111
    // 0b10110000
    // 0b10110001
    // 0b10110010
    // 0b10110011
    // 0b10110100
    // 0b10110101
    // 0b10110110
    // 0b10110111
    // 0b10111000
    // 0b10111001
    // 0b10111010
    // 0b10111011
    // 0b10111100
    // 0b10111101
    // 0b10111110
    // 0b10111111
    // 0b11000000
    // 0b11000001
    // 0b11000010
    // 0b11000011
    // 0b11000100
    // 0b11000101
    // 0b11000110
    // 0b11000111
    // 0b11001000
    // 0b11001001
    // 0b11001010
    // 0b11001011
    // 0b11001100
    // 0b11001101
    // 0b11001110
    // 0b11001111
    // 0b11010000
    // 0b11010001
    // 0b11010010
    // 0b11010011
    // 0b11010100
    // 0b11010101
    // 0b11010110
    // 0b11010111
    // 0b11011000
    // 0b11011001
    // 0b11011010
    // 0b11011011
    // 0b11011100
    // 0b11011101
    // 0b11011110
    // 0b11011111
    // 0b11100000
    // 0b11100001
    // 0b11100010
    // 0b11100011
    // 0b11100100
    // 0b11100101
    // 0b11100110
    // 0b11100111
    // 0b11101000
    // 0b11101001
    // 0b11101010
    // 0b11101011
    // 0b11101100
    // 0b11101101
    // 0b11101110
    // 0b11101111
    // 0b11110000
    // 0b11110001
    // 0b11110010
    // 0b11110011
    // 0b11110100
    // 0b11110101
    // 0b11110110
    // 0b11110111
    // 0b11111000
    // 0b11111001
    // 0b11111010
    // 0b11111011
    // 0b11111100
    // 0b11111101
    // 0b11111110
    // 0b11111111
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct GeneralPurposeRegister {
    value: u8,
}

impl Register for GeneralPurposeRegister {
    fn get(&self) -> u8 {
        self.value
    }

    fn load(&mut self, r_prime: u8) {
        self.value = r_prime;
    }
}

impl GeneralPurposeRegister {
    fn new() -> GeneralPurposeRegister {
        GeneralPurposeRegister { value: 0 }
    }
}

#[derive(Clone, Debug)]
pub struct Z80 {
    program_counter: u16,
    stack_pointer: u16,
    // Index registers
    ix: u16,
    iy: u16,
    // Interrupt vector
    i: u8,
    // Memory refresh
    r: u8,
    // Main accumulator register
    a: GeneralPurposeRegister,
    // Main flag register
    f: FlagRegister,
    // Alternate accumulator register
    a_prime: GeneralPurposeRegister,
    // Alternate flag register
    f_prime: FlagRegister,
    // General purpose registers
    b: GeneralPurposeRegister,
    c: GeneralPurposeRegister,
    d: GeneralPurposeRegister,
    e: GeneralPurposeRegister,
    h: GeneralPurposeRegister,
    l: GeneralPurposeRegister,
    // Alternate general purpose registers
    b_prime: GeneralPurposeRegister,
    c_prime: GeneralPurposeRegister,
    d_prime: GeneralPurposeRegister,
    e_prime: GeneralPurposeRegister,
    h_prime: GeneralPurposeRegister,
    l_prime: GeneralPurposeRegister,
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            program_counter: 0,
            stack_pointer: 0,
            ix: 0,
            iy: 0,
            i: 0,
            r: 0,
            a: GeneralPurposeRegister::new(),
            a_prime: GeneralPurposeRegister::new(),
            f: FlagRegister::new(),
            f_prime: FlagRegister::new(),
            b: GeneralPurposeRegister::new(),
            c: GeneralPurposeRegister::new(),
            d: GeneralPurposeRegister::new(),
            e: GeneralPurposeRegister::new(),
            h: GeneralPurposeRegister::new(),
            l: GeneralPurposeRegister::new(),
            b_prime: GeneralPurposeRegister::new(),
            c_prime: GeneralPurposeRegister::new(),
            d_prime: GeneralPurposeRegister::new(),
            e_prime: GeneralPurposeRegister::new(),
            h_prime: GeneralPurposeRegister::new(),
            l_prime: GeneralPurposeRegister::new(),
        }
    }

    pub fn fetch_next_opcode(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let opcode = memory_accessor.read(self.program_counter);
        self.program_counter += 1;
        opcode
    }

    pub fn process_next_instruction(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let mut t_states: u8 = 0;

        // Spend 4 ticks fetching the next instruction
        let opcode = self.fetch_next_opcode(memory_accessor);
        t_states += 4;

        let opcode_function = OPCODE_FUNCTIONS[opcode as usize];
        t_states += opcode_function(self, memory_accessor);

        return t_states;
    }

    // 8-Bit Load Group

    /// ### Operation
    ///
    /// r, ← r′
    ///
    /// ### Op Code
    ///
    /// LD: `0 1 r r r r' r' r'`
    ///
    /// ### Operands
    ///
    /// r, r′
    ///
    /// ### Description
    ///
    /// The contents of any register r' are loaded to any other register r.
    /// r, r' identifies any of the registers A, B, C, D, E, H, or L, assembled
    /// as follows in the object code:
    ///
    /// | Register | r, C |
    /// | -------- | ---- |
    /// | A        | 111  |
    /// | B        | 000  |
    /// | C        | 001  |
    /// | D        | 010  |
    /// | E        | 011  |
    /// | H        | 100  |
    /// | L        | 101  |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the H Register contains the number 8Ah, and the E register contains 10h, the instruction
    /// LD H, E results in both registers containing 10h.
    ///
    fn ld_r_rp(r: &mut dyn Register, r_prime: u8) -> u8 {
        r.load(r_prime);

        // T states
        4
    }

    fn ld_b_b(&mut self) -> u8 {
        let r_prime = self.b.get();
        Z80::ld_r_rp(&mut self.b, r_prime)
    }

    fn ld_b_c(&mut self) -> u8 {
        let r_prime = self.c.get();
        Z80::ld_r_rp(&mut self.b, r_prime)
    }

    fn ld_b_d(&mut self) -> u8 {
        let r_prime = self.d.get();
        Z80::ld_r_rp(&mut self.b, r_prime)
    }

    fn ld_b_e(&mut self) -> u8 {
        let r_prime = self.e.get();
        Z80::ld_r_rp(&mut self.b, r_prime)
    }

    fn ld_b_h(&mut self) -> u8 {
        let r_prime = self.h.get();
        Z80::ld_r_rp(&mut self.b, r_prime)
    }

    fn ld_b_l(&mut self) -> u8 {
        let r_prime = self.l.get();
        Z80::ld_r_rp(&mut self.b, r_prime)
    }

    fn ld_b_a(&mut self) -> u8 {
        let r_prime = self.a.get();
        Z80::ld_r_rp(&mut self.b, r_prime)
    }

    fn ld_c_b(&mut self) -> u8 {
        let r_prime = self.b.get();
        Z80::ld_r_rp(&mut self.c, r_prime)
    }

    fn ld_c_c(&mut self) -> u8 {
        let r_prime = self.c.get();
        Z80::ld_r_rp(&mut self.c, r_prime)
    }

    fn ld_c_d(&mut self) -> u8 {
        let r_prime = self.d.get();
        Z80::ld_r_rp(&mut self.c, r_prime)
    }

    fn ld_c_e(&mut self) -> u8 {
        let r_prime = self.e.get();
        Z80::ld_r_rp(&mut self.c, r_prime)
    }

    fn ld_c_h(&mut self) -> u8 {
        let r_prime = self.h.get();
        Z80::ld_r_rp(&mut self.c, r_prime)
    }

    fn ld_c_l(&mut self) -> u8 {
        let r_prime = self.l.get();
        Z80::ld_r_rp(&mut self.c, r_prime)
    }

    fn ld_c_a(&mut self) -> u8 {
        let r_prime = self.a.get();
        Z80::ld_r_rp(&mut self.c, r_prime)
    }

    fn ld_d_b(&mut self) -> u8 {
        let r_prime = self.b.get();
        Z80::ld_r_rp(&mut self.d, r_prime)
    }

    fn ld_d_c(&mut self) -> u8 {
        let r_prime = self.c.get();
        Z80::ld_r_rp(&mut self.d, r_prime)
    }

    fn ld_d_d(&mut self) -> u8 {
        let r_prime = self.d.get();
        Z80::ld_r_rp(&mut self.d, r_prime)
    }

    fn ld_d_e(&mut self) -> u8 {
        let r_prime = self.e.get();
        Z80::ld_r_rp(&mut self.d, r_prime)
    }

    fn ld_d_h(&mut self) -> u8 {
        let r_prime = self.h.get();
        Z80::ld_r_rp(&mut self.d, r_prime)
    }

    fn ld_d_l(&mut self) -> u8 {
        let r_prime = self.l.get();
        Z80::ld_r_rp(&mut self.d, r_prime)
    }

    fn ld_d_a(&mut self) -> u8 {
        let r_prime = self.a.get();
        Z80::ld_r_rp(&mut self.d, r_prime)
    }

    fn ld_e_b(&mut self) -> u8 {
        let r_prime = self.b.get();
        Z80::ld_r_rp(&mut self.e, r_prime)
    }

    fn ld_e_c(&mut self) -> u8 {
        let r_prime = self.c.get();
        Z80::ld_r_rp(&mut self.e, r_prime)
    }

    fn ld_e_d(&mut self) -> u8 {
        let r_prime = self.d.get();
        Z80::ld_r_rp(&mut self.e, r_prime)
    }

    fn ld_e_e(&mut self) -> u8 {
        let r_prime = self.e.get();
        Z80::ld_r_rp(&mut self.e, r_prime)
    }

    fn ld_e_h(&mut self) -> u8 {
        let r_prime = self.h.get();
        Z80::ld_r_rp(&mut self.e, r_prime)
    }

    fn ld_e_l(&mut self) -> u8 {
        let r_prime = self.l.get();
        Z80::ld_r_rp(&mut self.e, r_prime)
    }

    fn ld_e_a(&mut self) -> u8 {
        let r_prime = self.a.get();
        Z80::ld_r_rp(&mut self.e, r_prime)
    }

    fn ld_h_b(&mut self) -> u8 {
        let r_prime = self.b.get();
        Z80::ld_r_rp(&mut self.h, r_prime)
    }

    fn ld_h_c(&mut self) -> u8 {
        let r_prime = self.c.get();
        Z80::ld_r_rp(&mut self.h, r_prime)
    }

    fn ld_h_d(&mut self) -> u8 {
        let r_prime = self.d.get();
        Z80::ld_r_rp(&mut self.h, r_prime)
    }

    fn ld_h_e(&mut self) -> u8 {
        let r_prime = self.e.get();
        Z80::ld_r_rp(&mut self.h, r_prime)
    }

    fn ld_h_h(&mut self) -> u8 {
        let r_prime = self.h.get();
        Z80::ld_r_rp(&mut self.h, r_prime)
    }

    fn ld_h_l(&mut self) -> u8 {
        let r_prime = self.l.get();
        Z80::ld_r_rp(&mut self.h, r_prime)
    }

    fn ld_h_a(&mut self) -> u8 {
        let r_prime = self.a.get();
        Z80::ld_r_rp(&mut self.h, r_prime)
    }

    fn ld_l_b(&mut self) -> u8 {
        let r_prime = self.b.get();
        Z80::ld_r_rp(&mut self.l, r_prime)
    }

    fn ld_l_c(&mut self) -> u8 {
        let r_prime = self.c.get();
        Z80::ld_r_rp(&mut self.l, r_prime)
    }

    fn ld_l_d(&mut self) -> u8 {
        let r_prime = self.d.get();
        Z80::ld_r_rp(&mut self.l, r_prime)
    }

    fn ld_l_e(&mut self) -> u8 {
        let r_prime = self.e.get();
        Z80::ld_r_rp(&mut self.l, r_prime)
    }

    fn ld_l_h(&mut self) -> u8 {
        let r_prime = self.h.get();
        Z80::ld_r_rp(&mut self.l, r_prime)
    }

    fn ld_l_l(&mut self) -> u8 {
        let r_prime = self.l.get();
        Z80::ld_r_rp(&mut self.l, r_prime)
    }

    fn ld_l_a(&mut self) -> u8 {
        let r_prime = self.a.get();
        Z80::ld_r_rp(&mut self.l, r_prime)
    }

    fn ld_a_b(&mut self) -> u8 {
        let r_prime = self.b.get();
        Z80::ld_r_rp(&mut self.a, r_prime)
    }

    fn ld_a_c(&mut self) -> u8 {
        let r_prime = self.c.get();
        Z80::ld_r_rp(&mut self.a, r_prime)
    }

    fn ld_a_d(&mut self) -> u8 {
        let r_prime = self.d.get();
        Z80::ld_r_rp(&mut self.a, r_prime)
    }

    fn ld_a_e(&mut self) -> u8 {
        let r_prime = self.e.get();
        Z80::ld_r_rp(&mut self.a, r_prime)
    }

    fn ld_a_h(&mut self) -> u8 {
        let r_prime = self.h.get();
        Z80::ld_r_rp(&mut self.a, r_prime)
    }

    fn ld_a_l(&mut self) -> u8 {
        let r_prime = self.l.get();
        Z80::ld_r_rp(&mut self.a, r_prime)
    }

    fn ld_a_a(&mut self) -> u8 {
        let r_prime = self.a.get();
        Z80::ld_r_rp(&mut self.a, r_prime)
    }

    /// ### Operation
    ///
    /// r ← n
    ///
    /// ### Op Code
    ///
    /// LD: `0 0 r r r 1 1 0`
    /// `n n n n n n n n`
    ///
    /// ### Operands
    ///
    /// r, n
    ///
    /// ### Description
    ///
    /// The 8-bit integer n is loaded to any register r, in which r identifies
    /// registers A, B, C, D, E, H, or L, assembled as follows in the object
    /// code:
    ///
    /// | Register | r   |
    /// | -------- | --- |
    /// | A        | 111 |
    /// | B        | 000 |
    /// | C        | 001 |
    /// | D        | 010 |
    /// | E        | 011 |
    /// | H        | 100 |
    /// | L        | 101 |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.

    fn ld_r_n(r: &mut dyn Register, n: u8) -> u8 {
        r.load(n);

        // T states
        7
    }

    fn ld_a_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        self.a.load(n);
        Z80::ld_r_n(&mut self.a, n)
    }

    fn ld_b_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.b, n)
    }

    fn ld_c_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.c, n)
    }

    fn ld_d_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.d, n)
    }

    fn ld_e_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.e, n)
    }

    fn ld_h_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.h, n)
    }

    fn ld_l_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.l, n)
    }

    /// ### Operation
    /// 
    /// r ← (HL)
    /// 
    /// ### Op Code
    /// 
    /// LD
    /// 
    /// ### Operands
    /// 
    /// r, (HL)  `0 1 r r r 1 1 0`
    /// 
    /// ### Description
    /// 
    /// The 8-bit contents of memory location (HL) are loaded to register r, in
    /// which r identifies registers A, B, C, D, E, H, or L, assembled as
    /// follows in the object code:
    /// 
    /// | Register | r   |
    /// | -------- | --- |
    /// | A        | 111 |
    /// | B        | 000 |
    /// | C        | 001 |
    /// | D        | 010 |
    /// | E        | 011 |
    /// | H        | 100 |
    /// | L        | 101 |
    /// 
    /// ### Condition Bits Affected
    /// 
    /// None.
    /// 
    /// ### Example
    /// 
    /// If register pair HL contains the number 75A1h, and memory address 75A1h
    /// contains byte 58h, the execution of LD C, (HL) results in 58h in
    /// Register C.
    fn ld_r_hl(r: &mut dyn Register, memory_accessor: &dyn MemoryAccessor, address: u16) -> u8 {
        let data = memory_accessor.read(address);

        r.load(data);

        // T states
        7
    }

    fn ld_a_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let address: u16 = ((self.h.get() as u16) << 8) | self.l.get() as u16;
        Z80::ld_r_hl(&mut self.a, memory_accessor, address)
    }

    fn ld_b_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let address: u16 = ((self.h.get() as u16) << 8) | self.l.get() as u16;
        Z80::ld_r_hl(&mut self.b, memory_accessor, address)
    }

    fn ld_c_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let address: u16 = ((self.h.get() as u16) << 8) | self.l.get() as u16;
        Z80::ld_r_hl(&mut self.c, memory_accessor, address)
    }

    fn ld_d_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let address: u16 = ((self.h.get() as u16) << 8) | self.l.get() as u16;
        Z80::ld_r_hl(&mut self.d, memory_accessor, address)
    }

    fn ld_e_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let address: u16 = ((self.h.get() as u16) << 8) | self.l.get() as u16;
        Z80::ld_r_hl(&mut self.e, memory_accessor, address)
    }

    fn ld_h_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let address: u16 = ((self.h.get() as u16) << 8) | self.l.get() as u16;
        Z80::ld_r_hl(&mut self.h, memory_accessor, address)
    }

    fn ld_l_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let address: u16 = ((self.h.get() as u16) << 8) | self.l.get() as u16;
        Z80::ld_r_hl(&mut self.l, memory_accessor, address)
    }

    // General-Purpose Arithmetic and CPU Control Groups

    /// ### Operation
    ///
    /// @
    ///
    /// ### Op Code
    ///
    /// DAA: `0 0 1 0 0 1 1 1` (0x27)
    ///
    /// ### Operands
    ///
    /// None.
    ///
    /// ### Description
    ///
    /// This instruction conditionally adjusts the Accumulator for BCD addition
    /// and subtraction operations. For addition (ADD, ADC, INC) or subtraction
    /// (SUB, SBC, DEC, NEG), the following table indicates the operation being
    /// performed:
    ///
    /// | Operation | C Before DAA | Hex Value In Upper Digit (Bits 7–4) | H Before DAA | Hex Value In Lower Digit (Bits 3–0) | Number Added To Byte | C After DAA |
    /// | --------- | ------------ | ----------------------------------- | ------------ | ----------------------------------- | -------------------- | ----------- |
    /// |           | 0            | 9–0                                 | 0            | 0–9                                 | 00                   | 0           |
    /// |           | 0            | 0–8                                 | 0            | A–F                                 | 06                   | 0           |
    /// |           | 0            | 0–9                                 | 1            | 0–3                                 | 06                   | 0           |
    /// | ADD       | 0            | A–F                                 | 0            | 0–9                                 | 60                   | 1           |
    /// | ADC       | 0            | 9–F                                 | 0            | A–F                                 | 66                   | 1           |
    /// | INC       | 0            | A–F                                 | 1            | 0–3                                 | 66                   | 1           |
    /// |           | 1            | 0–2                                 | 0            | 0–9                                 | 60                   | 1           |
    /// |           | 1            | 0–2                                 | 0            | A–F                                 | 66                   | 1           |
    /// |           | 1            | 0–3                                 | 1            | 0–3                                 | 66                   | 1           |
    /// | SUB       | 0            | 0–9                                 | 0            | 0–9                                 | 00                   | 0           |
    /// | SBC       | 0            | 0–8                                 | 1            | 6–F                                 | FA                   | 0           |
    /// | DEC       | 1            | 7–F                                 | 0            | 0–9                                 | A0                   | 1           |
    /// | NEG       | 1            | 6–7                                 | 1            | 6–F                                 | 9A                   | 1           |
    ///
    /// ### Condition Bits Affected
    ///
    /// S is set if most-significant bit of the
    /// Accumulator is 1 after an operation; otherwise, it is reset.
    ///
    /// Z is set if the Accumulator is 0 after an operation; otherwise, it is
    /// reset.
    ///
    /// H: see the DAA instruction table on the previous page.
    ///
    /// P/V is set if the Accumulator is at even parity after an operation;
    /// otherwise, it is reset.
    ///
    /// N is not affected.
    ///
    /// C: see the DAA instruction table on the previous page.
    ///
    /// ### Example
    ///
    /// An addition operation is performed between 15 (BCD) and 27 (BCD); simple
    /// decimal arithmetic provides the following result:
    /// ```
    ///   15
    /// + 27
    ///   --
    ///   42
    /// ```
    ///
    /// The binary representations are added in the Accumulator according to
    /// standard binary arithmetic, as follows:
    /// ```
    ///   0001 0101
    /// + 0010 0111
    ///   ---- ----
    ///   0011 1100 = 3C
    /// ```
    ///
    /// The sum is ambiguous. The DAA instruction adjusts this result so that
    /// the correct BCD representation is obtained, as follows:
    /// ```
    ///   0011 1100
    /// + 0000 0110
    ///   ---- ----
    ///   0100 0010 = 42
    /// ```
    fn daa(&mut self) -> u8 {
        let a_high = self.a.get() & 0xF0;
        let a_low = self.a.get() & 0x0F;
        let diff: u8 = match (self.f.c_flag(), a_high, self.f.h_flag(), a_low) {
            (false, a_high, false, a_low) if a_high <= 0x09 && a_low <= 0x09 => 0x00,
            (false, a_high, true, a_low) if a_high <= 0x09 && a_low <= 0x09 => 0x06,
            (false, a_high, _, a_low) if a_high <= 0x08 && a_low >= 0x0A => 0x06,
            (false, a_high, false, a_low) if a_high >= 0x0A && a_low <= 0x09 => 0x60,
            (true, _, false, a_low) if a_low <= 0x09 => 0x60,
            (true, _, true, a_low) if a_low <= 0x09 => 0x66,
            (true, _, _, a_low) if a_low >= 0x0A => 0x66,
            (false, a_high, _, a_low) if a_high >= 0x09 && a_low >= 0x0A => 0x66,
            (false, a_high, true, a_low) if a_high >= 0x0A && a_low <= 0x09 => 0x66,
            _ => 0x00,
        };

        let new_c_flag: bool = match (self.f.c_flag(), a_high, a_low) {
            (false, a_high, a_low) if a_high <= 0x09 && a_low <= 0x09 => false,
            (false, a_high, a_low) if a_high <= 0x08 && a_low >= 0x0A => false,
            (false, a_high, a_low) if a_high >= 0x09 && a_low >= 0x0A => true,
            (false, a_high, a_low) if a_high >= 0x0A && a_low <= 0x09 => true,
            (true, _, _) => true,
            _ => self.f.c_flag(),
        };

        let new_h_flag: bool = match (self.f.n_flag(), self.f.h_flag(), a_low) {
            (false, _, a_low) if a_low <= 0x09 => false,
            (false, _, a_low) if a_low >= 0x0A => true,
            (true, false, _) => false,
            (true, true, a_low) if a_low >= 0x06 => false,
            (true, true, a_low) if a_low <= 0x05 => true,
            _ => self.f.h_flag(),
        };

        self.a.load(self.a.get() + diff);

        if new_c_flag {
            self.f.set_c_flag();
        } else {
            self.f.unset_c_flag();
        }

        if new_h_flag {
            self.f.set_h_flag();
        } else {
            self.f.unset_h_flag();
        }

        // SF, YF, XF are copies of bit 7,5,3 of the result respectively;
        self.f
            .load(self.f.get() & 0b10101011u8 + self.a.get() & 0b01010100u8);

        // ZF is set according to the result and NF is always unchanged.
        if self.a.get() == 0 {
            self.f.set_z_flag();
        }

        // T states
        4
    }

    /// Hex: 00
    /// Op Code: NOP
    /// T-States: 4
    fn nop() -> u8 {
        // T states
        4
    }
}

mod tests {
    use super::*;

    struct Ram<'a> {
        bytes: &'a mut [u8],
    }

    impl<'a> Ram<'a> {
        fn new(bytes: &'a mut [u8]) -> Ram {
            Ram {
                bytes
            }
        }
    }

    impl<'a> MemoryAccessor for Ram<'a> {
        fn read(&self, address: u16) -> u8 {
            self.bytes[address as usize]
        }

        fn write(&mut self, address: u16, data: u8) {
            self.bytes[address as usize] = data;
        }
    }

    #[test]
    fn test_fetch_next_opcode() {
        let mut bytes = [0x00, 0x01, 0x02];

        let ram = Ram::new(&mut bytes);

        let mut z80 = Z80::new();

        assert_eq!(0, z80.program_counter);

        assert_eq!(ram.bytes[0], z80.fetch_next_opcode(&ram));

        assert_eq!(1, z80.program_counter);

        assert_eq!(ram.bytes[1], z80.fetch_next_opcode(&ram));

        assert_eq!(2, z80.program_counter);

        assert_eq!(ram.bytes[2], z80.fetch_next_opcode(&ram));

        assert_eq!(3, z80.program_counter);
    }

    #[test]
    fn test_process_next_instruction() {
        let mut bytes = [
            // 0x26, // LD H, n
            0x05, // LD H, n
            0xDD,
        ];

        let mut ram = Ram::new(&mut bytes);

        let mut z80 = Z80::new();

        let t_states = z80.process_next_instruction(&mut ram);

        assert_eq!(11, t_states);
        assert_eq!(bytes[1], z80.h.get());
    }

    #[test]
    fn test_ld_b_b() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.b.get());

        z80.b.load(0xDD);

        z80.ld_b_b();

        assert_eq!(0xDD, z80.b.get());
    }

    #[test]
    fn test_ld_b_c() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.b.get());

        z80.c.load(0xDD);

        z80.ld_b_c();

        assert_eq!(0xDD, z80.b.get());
    }

    #[test]
    fn test_ld_b_d() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.b.get());

        z80.d.load(0xDD);

        z80.ld_b_d();

        assert_eq!(0xDD, z80.b.get());
    }

    #[test]
    fn test_ld_b_e() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.b.get());

        z80.e.load(0xDD);

        z80.ld_b_e();

        assert_eq!(0xDD, z80.b.get());
    }

    #[test]
    fn test_ld_b_h() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.b.get());

        z80.h.load(0xDD);

        z80.ld_b_h();

        assert_eq!(0xDD, z80.b.get());
    }

    #[test]
    fn test_ld_b_l() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.b.get());

        z80.l.load(0xDD);

        z80.ld_b_l();

        assert_eq!(0xDD, z80.b.get());
    }

    #[test]
    fn test_ld_b_a() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.b.get());

        z80.a.load(0xDD);

        z80.ld_b_a();

        assert_eq!(0xDD, z80.b.get());
    }

    #[test]
    fn test_ld_c_b() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.c.get());

        z80.b.load(0xDD);

        z80.ld_c_b();

        assert_eq!(0xDD, z80.c.get());
    }

    #[test]
    fn test_ld_c_c() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.c.get());

        z80.c.load(0xDD);

        z80.ld_c_c();

        assert_eq!(0xDD, z80.c.get());
    }

    #[test]
    fn test_ld_c_d() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.c.get());

        z80.d.load(0xDD);

        z80.ld_c_d();

        assert_eq!(0xDD, z80.c.get());
    }

    #[test]
    fn test_ld_c_e() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.c.get());

        z80.e.load(0xDD);

        z80.ld_c_e();

        assert_eq!(0xDD, z80.c.get());
    }

    #[test]
    fn test_ld_c_h() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.c.get());

        z80.h.load(0xDD);

        z80.ld_c_h();

        assert_eq!(0xDD, z80.c.get());
    }

    #[test]
    fn test_ld_c_l() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.c.get());

        z80.l.load(0xDD);

        z80.ld_c_l();

        assert_eq!(0xDD, z80.c.get());
    }

    #[test]
    fn test_ld_c_a() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.c.get());

        z80.a.load(0xDD);

        z80.ld_c_a();

        assert_eq!(0xDD, z80.c.get());
    }

    #[test]
    fn test_ld_d_b() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.d.get());

        z80.b.load(0xDD);

        z80.ld_d_b();

        assert_eq!(0xDD, z80.d.get());
    }

    #[test]
    fn test_ld_d_c() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.d.get());

        z80.c.load(0xDD);

        z80.ld_d_c();

        assert_eq!(0xDD, z80.d.get());
    }

    #[test]
    fn test_ld_d_d() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.d.get());

        z80.c.load(0xDD);

        z80.ld_d_c();

        assert_eq!(0xDD, z80.d.get());
    }

    #[test]
    fn test_ld_d_e() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.d.get());

        z80.e.load(0xDD);

        z80.ld_d_e();

        assert_eq!(0xDD, z80.d.get());
    }

    #[test]
    fn test_ld_d_h() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.d.get());

        z80.h.load(0xDD);

        z80.ld_d_h();

        assert_eq!(0xDD, z80.d.get());
    }

    #[test]
    fn test_ld_d_l() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.d.get());

        z80.l.load(0xDD);

        z80.ld_d_l();

        assert_eq!(0xDD, z80.d.get());
    }

    #[test]
    fn test_ld_d_a() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.d.get());

        z80.a.load(0xDD);

        z80.ld_d_a();

        assert_eq!(0xDD, z80.d.get());
    }

    #[test]
    fn test_ld_e_b() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.e.get());

        z80.b.load(0xDD);

        z80.ld_e_b();

        assert_eq!(0xDD, z80.e.get());
    }

    #[test]
    fn test_ld_e_c() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.e.get());

        z80.c.load(0xDD);

        z80.ld_e_c();

        assert_eq!(0xDD, z80.e.get());
    }

    #[test]
    fn test_ld_e_d() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.e.get());

        z80.d.load(0xDD);

        z80.ld_e_d();

        assert_eq!(0xDD, z80.e.get());
    }

    #[test]
    fn test_ld_e_e() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.e.get());

        z80.e.load(0xDD);

        z80.ld_e_e();

        assert_eq!(0xDD, z80.e.get());
    }

    #[test]
    fn test_ld_e_h() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.e.get());

        z80.h.load(0xDD);

        z80.ld_e_h();

        assert_eq!(0xDD, z80.e.get());
    }

    #[test]
    fn test_ld_e_l() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.e.get());

        z80.l.load(0xDD);

        z80.ld_e_l();

        assert_eq!(0xDD, z80.e.get());
    }

    #[test]
    fn test_ld_e_a() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.e.get());

        z80.a.load(0xDD);

        z80.ld_e_a();

        assert_eq!(0xDD, z80.e.get());
    }

    #[test]
    fn test_ld_h_b() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.h.get());

        z80.b.load(0xDD);

        z80.ld_h_b();

        assert_eq!(0xDD, z80.h.get());
    }

    #[test]
    fn test_ld_h_c() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.h.get());

        z80.c.load(0xDD);

        z80.ld_h_c();

        assert_eq!(0xDD, z80.h.get());
    }

    #[test]
    fn test_ld_h_d() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.h.get());

        z80.d.load(0xDD);

        z80.ld_h_d();

        assert_eq!(0xDD, z80.h.get());
    }

    #[test]
    fn test_ld_h_e() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.h.get());

        z80.e.load(0xDD);

        z80.ld_h_e();

        assert_eq!(0xDD, z80.h.get());
    }

    #[test]
    fn test_ld_h_h() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.h.get());

        z80.h.load(0xDD);

        z80.ld_h_h();

        assert_eq!(0xDD, z80.h.get());
    }

    #[test]
    fn test_ld_h_l() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.h.get());

        z80.l.load(0xDD);

        z80.ld_h_l();

        assert_eq!(0xDD, z80.h.get());
    }

    #[test]
    fn test_ld_h_a() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.h.get());

        z80.a.load(0xDD);

        z80.ld_h_a();

        assert_eq!(0xDD, z80.h.get());
    }

    #[test]
    fn test_ld_l_b() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.l.get());

        z80.b.load(0xDD);

        z80.ld_l_b();

        assert_eq!(0xDD, z80.l.get());
    }

    #[test]
    fn test_ld_l_c() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.l.get());

        z80.c.load(0xDD);

        z80.ld_l_c();

        assert_eq!(0xDD, z80.l.get());
    }

    #[test]
    fn test_ld_l_d() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.l.get());

        z80.d.load(0xDD);

        z80.ld_l_d();

        assert_eq!(0xDD, z80.l.get());
    }

    #[test]
    fn test_ld_l_e() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.l.get());

        z80.e.load(0xDD);

        z80.ld_l_e();

        assert_eq!(0xDD, z80.l.get());
    }

    #[test]
    fn test_ld_l_h() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.l.get());

        z80.h.load(0xDD);

        z80.ld_l_h();

        assert_eq!(0xDD, z80.l.get());
    }

    #[test]
    fn test_ld_l_l() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.l.get());

        z80.l.load(0xDD);

        z80.ld_l_l();

        assert_eq!(0xDD, z80.l.get());
    }

    #[test]
    fn test_ld_l_a() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.l.get());

        z80.a.load(0xDD);

        z80.ld_l_a();

        assert_eq!(0xDD, z80.l.get());
    }

    #[test]
    fn test_ld_a_b() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.a.get());

        z80.b.load(0xDD);

        z80.ld_a_b();

        assert_eq!(0xDD, z80.a.get());
    }

    #[test]
    fn test_ld_a_c() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.a.get());

        z80.c.load(0xDD);

        z80.ld_a_c();

        assert_eq!(0xDD, z80.a.get());
    }

    #[test]
    fn test_ld_a_d() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.a.get());

        z80.d.load(0xDD);

        z80.ld_a_d();

        assert_eq!(0xDD, z80.a.get());
    }

    #[test]
    fn test_ld_a_e() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.a.get());

        z80.e.load(0xDD);

        z80.ld_a_e();

        assert_eq!(0xDD, z80.a.get());
    }

    #[test]
    fn test_ld_a_h() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.a.get());

        z80.h.load(0xDD);

        z80.ld_a_h();

        assert_eq!(0xDD, z80.a.get());
    }

    #[test]
    fn test_ld_a_l() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.a.get());

        z80.l.load(0xDD);

        z80.ld_a_l();

        assert_eq!(0xDD, z80.a.get());
    }

    #[test]
    fn test_ld_a_a() {
        let mut z80 = Z80::new();

        assert_eq!(0x00, z80.a.get());

        z80.a.load(0xDD);

        z80.ld_a_a();

        assert_eq!(0xDD, z80.a.get());
    }

    #[test]
    fn test_ld_a_hl() {
        let mut bytes = [0xAA, 0xBB, 0xCC];
        let ram = Ram::new(&mut bytes);

        let mut z80 = Z80::new();
        z80.h.load(0x00);
        z80.l.load(0x02);

        z80.ld_a_hl(&ram);

        assert_eq!(bytes[2], z80.a.get());
    }

    #[test]
    fn test_ld_b_hl() {
        let mut bytes = [0xAA, 0xBB, 0xCC];
        let ram = Ram::new(&mut bytes);

        let mut z80 = Z80::new();
        z80.h.load(0x00);
        z80.l.load(0x02);

        z80.ld_b_hl(&ram);

        assert_eq!(bytes[2], z80.b.get());
    }

    #[test]
    fn test_ld_c_hl() {
        let mut bytes = [0xAA, 0xBB, 0xCC];
        let ram = Ram::new(&mut bytes);

        let mut z80 = Z80::new();
        z80.h.load(0x00);
        z80.l.load(0x02);

        z80.ld_c_hl(&ram);

        assert_eq!(bytes[2], z80.c.get());
    }

    #[test]
    fn test_ld_d_hl() {
        let mut bytes = [0xAA, 0xBB, 0xCC];
        let ram = Ram::new(&mut bytes);

        let mut z80 = Z80::new();
        z80.h.load(0x00);
        z80.l.load(0x02);

        z80.ld_d_hl(&ram);

        assert_eq!(bytes[2], z80.d.get());
    }

    #[test]
    fn test_ld_e_hl() {
        let mut bytes = [0xAA, 0xBB, 0xCC];
        let ram = Ram::new(&mut bytes);

        let mut z80 = Z80::new();
        z80.h.load(0x00);
        z80.l.load(0x02);

        z80.ld_e_hl(&ram);

        assert_eq!(bytes[2], z80.e.get());
    }

    #[test]
    fn test_ld_h_hl() {
        let mut bytes = [0xAA, 0xBB, 0xCC];
        let ram = Ram::new(&mut bytes);

        let mut z80 = Z80::new();
        z80.h.load(0x00);
        z80.l.load(0x02);

        z80.ld_h_hl(&ram);

        assert_eq!(bytes[2], z80.h.get());
    }

    #[test]
    fn test_ld_l_hl() {
        let mut bytes = [0xAA, 0xBB, 0xCC];
        let ram = Ram::new(&mut bytes);

        let mut z80 = Z80::new();
        z80.h.load(0x00);
        z80.l.load(0x02);

        z80.ld_l_hl(&ram);

        assert_eq!(bytes[2], z80.l.get());
    }

}
