mod eight_bit_load_group;

// Official Z80 documentation: https://www.zilog.com/docs/z80/um0080.pdf
// Unofficial undocumented functionality documentation: http://www.z80.info/zip/z80-documented.pdf
// Integration test suites: https://mdfs.net/Software/Z80/Exerciser/Spectrum/

use crate::flag_register::{
    c_flag_set, h_flag_set, n_flag_set, set_c_flag, set_h_flag, set_z_flag, unset_c_flag,
    unset_h_flag,
};

pub trait MemoryAccessor {
    fn read(&self, address: &u16) -> u8;
    fn write(&mut self, address: &u16, data: &u8);
}

const MAIN_FUNCTIONS: [fn(&mut Z80, &mut dyn MemoryAccessor) -> u8; 79] = [
    // 00000000 NOP
    |_, _| Z80::nop(),
    // 00000001
    // 00000010 LD (BC), A
    Z80::ld_bc_a,
    // 00000011
    // 00000100
    // 00000101
    // 00000110 LD B, n
    Z80::ld_b_n,
    // 00000111
    // 00001000
    // 00001001
    // 00001010 LA A, (BC)
    |z80, memory_accessor| z80.ld_a_bc(memory_accessor),
    // 00001011
    // 00001100
    // 00001101
    // 00001110 LD C, n
    Z80::ld_c_n,
    // 00001111
    // 00010000
    // 00010001
    // 00010010 LD (DE), A
    Z80::ld_de_a,
    // 00010011
    // 00010100
    // 00010101
    // 00010110 LD D, n
    Z80::ld_d_n,
    // 00010111
    // 00011000
    // 00011001
    // 00011010 LA A, (DE)
    |z80, memory_accessor| z80.ld_a_de(memory_accessor),
    // 00011011
    // 00011100
    // 00011101
    // 00011110 LD E, n
    Z80::ld_e_n,
    // 00011111
    // 00100000
    // 00100001
    // 00100010
    // 00100011
    // 00100100
    // 00100101
    // 00100110 LD H, n
    Z80::ld_h_n,
    // 00100111
    |z80, _| z80.daa(),
    // 00101000
    // 00101001
    // 00101010
    // 00101011
    // 00101100
    // 00101101
    // 00101110 LD L, n
    Z80::ld_l_n,
    // 00101111
    // 00110000
    // 00110001
    // 00110010 LD (nn), A
    Z80::ld_nn_a,
    // 00110011
    // 00110100
    // 00110101
    // 00110110 LD (HL), n
    |z80, memory_accessor| z80.ld_hl_n(memory_accessor),
    // 00110111
    // 00111000
    // 00111001
    // 00111010 LD A, (nn)
    |z80, memory_accessor| z80.ld_a_nn(memory_accessor),
    // 00111011
    // 00111100
    // 00111101
    // 00111110 LD A, n
    Z80::ld_a_n,
    // 00111111
    // 01000000 LD B, B
    |z80, _| z80.ld_b_b(),
    // 01000001 LD B, C
    |z80, _| z80.ld_b_c(),
    // 01000010 LD B, D
    |z80, _| z80.ld_b_d(),
    // 01000011 LD B, E
    |z80, _| z80.ld_b_e(),
    // 01000100 LD B, H
    |z80, _| z80.ld_b_h(),
    // 01000101 LD B, L
    |z80, _| z80.ld_b_l(),
    // 01000110 LD B, (HL)
    |z80, memory_accessor| z80.ld_b_hl(memory_accessor),
    // 01000111 LD B, A
    |z80, _| z80.ld_b_a(),
    // 01001000 LD C, B
    |z80, _| z80.ld_c_b(),
    // 01001001 LD C, C
    |z80, _| z80.ld_c_c(),
    // 01001010 LD C, D
    |z80, _| z80.ld_c_d(),
    // 01001011 LD C, E
    |z80, _| z80.ld_c_e(),
    // 01001100 LD C, H
    |z80, _| z80.ld_c_h(),
    // 01001101 LD C, L
    |z80, _| z80.ld_c_l(),
    // 01001110 LD C, (HL)
    |z80, memory_accessor| z80.ld_c_hl(memory_accessor),
    // 01001111 LD C, A
    |z80, _| z80.ld_c_a(),
    // 01010000 LD D, B
    |z80, _| z80.ld_d_b(),
    // 01010001 LD D, C
    |z80, _| z80.ld_d_c(),
    // 01010010 LD D, D
    |z80, _| z80.ld_d_d(),
    // 01010011 LD D, E
    |z80, _| z80.ld_d_e(),
    // 01010100 LD D, H
    |z80, _| z80.ld_d_h(),
    // 01010101 LD D, L
    |z80, _| z80.ld_d_l(),
    // 01010110 LD D, (HL)
    |z80, memory_accessor| z80.ld_d_hl(memory_accessor),
    // 01010111 LD D, A
    |z80, _| z80.ld_d_a(),
    // 01011000 LD E, B
    |z80, _| z80.ld_e_b(),
    // 01011001 LD E, C
    |z80, _| z80.ld_e_c(),
    // 01011010 LD E, D
    |z80, _| z80.ld_e_d(),
    // 01011011 LD E, E
    |z80, _| z80.ld_e_e(),
    // 01011100 LD E, H
    |z80, _| z80.ld_e_h(),
    // 01011101 LD E, L
    |z80, _| z80.ld_e_l(),
    // 01011110 LD E, (HL)
    |z80, memory_accessor| z80.ld_e_hl(memory_accessor),
    // 01011111 LD E, A
    |z80, _| z80.ld_e_a(),
    // 01100000 LD H, B
    |z80, _| z80.ld_h_b(),
    // 01100001 LD H, C
    |z80, _| z80.ld_h_c(),
    // 01100010 LD H, D
    |z80, _| z80.ld_h_d(),
    // 01100011 LD H, E
    |z80, _| z80.ld_h_e(),
    // 01100100 LD H, H
    |z80, _| z80.ld_h_h(),
    // 01100101 LD H, L
    |z80, _| z80.ld_h_l(),
    // 01100110 LD H, (HL)
    |z80, memory_accessor| z80.ld_h_hl(memory_accessor),
    // 01100111 LD H, A
    |z80, _| z80.ld_h_a(),
    // 01101000 LD L, B
    |z80, _| z80.ld_l_b(),
    // 01101001 LD L, C
    |z80, _| z80.ld_l_c(),
    // 01101010 LD L, D
    |z80, _| z80.ld_l_d(),
    // 01101011 LD L, E
    |z80, _| z80.ld_l_e(),
    // 01101100 LD L, H
    |z80, _| z80.ld_l_h(),
    // 01101101 LD L, L
    |z80, _| z80.ld_l_l(),
    // 01101110 LD L, (HL)
    |z80, memory_accessor| z80.ld_l_hl(memory_accessor),
    // 01101111 LD L, A
    |z80, _| z80.ld_l_a(),
    // 01110000 LD (HL), B
    |z80, memory_accessor| z80.ld_hl_b(memory_accessor),
    // 01110001 LD (HL), C
    |z80, memory_accessor| z80.ld_hl_c(memory_accessor),
    // 01110010 LD (HL), D
    |z80, memory_accessor| z80.ld_hl_d(memory_accessor),
    // 01110011 LD (HL), E
    |z80, memory_accessor| z80.ld_hl_e(memory_accessor),
    // 01110100 LD (HL), H
    |z80, memory_accessor| z80.ld_hl_h(memory_accessor),
    // 01110101 LD (HL), L
    |z80, memory_accessor| z80.ld_hl_l(memory_accessor),
    // 01110110
    // 01110111 LD (HL), A
    |z80, memory_accessor| z80.ld_hl_a(memory_accessor),
    // 01111000 LD A, B
    |z80, _| z80.ld_a_b(),
    // 01111001 LD A, C
    |z80, _| z80.ld_a_c(),
    // 01111010 LD A, D
    |z80, _| z80.ld_a_d(),
    // 01111011 LD A, E
    |z80, _| z80.ld_a_e(),
    // 01111100 LD A, H
    |z80, _| z80.ld_a_h(),
    // 01111101 LD A, L
    |z80, _| z80.ld_a_l(),
    // 01111110 LD A, (HL)
    |z80, memory_accessor| z80.ld_a_hl(memory_accessor),
    // 01111111 LD A, A
    |z80, _| z80.ld_a_a(),
    // 10000000
    // 10000001
    // 10000010
    // 10000011
    // 10000100
    // 10000101
    // 10000110
    // 10000111
    // 10001000
    // 10001001
    // 10001010
    // 10001011
    // 10001100
    // 10001101
    // 10001110
    // 10001111
    // 10010000
    // 10010001
    // 10010010
    // 10010011
    // 10010100
    // 10010101
    // 10010110
    // 10010111
    // 10011000
    // 10011001
    // 10011010
    // 10011011
    // 10011100
    // 10011101
    // 10011110
    // 10011111
    // 10100000
    // 10100001
    // 10100010
    // 10100011
    // 10100100
    // 10100101
    // 10100110
    // 10100111
    // 10101000
    // 10101001
    // 10101010
    // 10101011
    // 10101100
    // 10101101
    // 10101110
    // 10101111
    // 10110000
    // 10110001
    // 10110010
    // 10110011
    // 10110100
    // 10110101
    // 10110110
    // 10110111
    // 10111000
    // 10111001
    // 10111010
    // 10111011
    // 10111100
    // 10111101
    // 10111110
    // 10111111
    // 11000000
    // 11000001
    // 11000010
    // 11000011
    // 11000100
    // 11000101
    // 11000110
    // 11000111
    // 11001000
    // 11001001
    // 11001010
    // 11001011
    // 11001100
    // 11001101
    // 11001110
    // 11001111
    // 11010000
    // 11010001
    // 11010010
    // 11010011
    // 11010100
    // 11010101
    // 11010110
    // 11010111
    // 11011000
    // 11011001
    // 11011010
    // 11011011
    // 11011100
    // 11011101
    // 11011110
    // 11011111
    // 11100000
    // 11100001
    // 11100010
    // 11100011
    // 11100100
    // 11100101
    // 11100110
    // 11100111
    // 11101000
    // 11101001
    // 11101010
    // 11101011
    // 11101100
    // 11101101
    // 11101110
    // 11101111
    // 11110000
    // 11110001
    // 11110010
    // 11110011
    // 11110100
    // 11110101
    // 11110110
    // 11110111
    // 11111000
    // 11111001
    // 11111010
    // 11111011
    // 11111100
    // 11111101
    // 11111110
    // 11111111
];

// CB prefix
const BIT_INSTRUCTIONS: [fn(&mut Z80, &mut dyn MemoryAccessor) -> u8; 0] = [];

// DD prefix
const IX_FUNCTIONS: [fn(&mut Z80, &mut dyn MemoryAccessor) -> u8; 15] = [
    // 00000000
    // 00000001
    // 00000010
    // 00000011
    // 00000100
    // 00000101
    // 00000110
    // 00000111
    // 00001000
    // 00001001
    // 00001010
    // 00001011
    // 00001100
    // 00001101
    // 00001110
    // 00001111
    // 00010000
    // 00010001
    // 00010010
    // 00010011
    // 00010100
    // 00010101
    // 00010110
    // 00010111
    // 00011000
    // 00011001
    // 00011010
    // 00011011
    // 00011100
    // 00011101
    // 00011110
    // 00011111
    // 00100000
    // 00100001
    // 00100010
    // 00100011
    // 00100100
    // 00100101
    // 00100110
    // 00100111
    // 00101000
    // 00101001
    // 00101010
    // 00101011
    // 00101100
    // 00101101
    // 00101110
    // 00101111
    // 00110000
    // 00110001
    // 00110010
    // 00110011
    // 00110100
    // 00110101
    // 00110110 LD (IX+d), n
    |z80, memory_accessor| z80.ld_ixd_n(memory_accessor),
    // 00110111
    // 00111000
    // 00111001
    // 00111010
    // 00111011
    // 00111100
    // 00111101
    // 00111110
    // 00111111
    // 01000000
    // 01000001
    // 01000010
    // 01000011
    // 01000100
    // 01000101
    // 01000110 LD B, (IX+d)
    |z80, memory_accessor| z80.ld_b_ixd(memory_accessor),
    // 01000111
    // 01001000
    // 01001001
    // 01001010
    // 01001011
    // 01001100
    // 01001101
    // 01001110 LD C, (IX+d)
    |z80, memory_accessor| z80.ld_c_ixd(memory_accessor),
    // 01001111
    // 01010000
    // 01010001
    // 01010010
    // 01010011
    // 01010100
    // 01010101
    // 01010110 LD D, (IX+d)
    |z80, memory_accessor| z80.ld_d_ixd(memory_accessor),
    // 01010111
    // 01011000
    // 01011001
    // 01011010
    // 01011011
    // 01011100
    // 01011101
    // 01011110 LD E, (IX+d)
    |z80, memory_accessor| z80.ld_e_ixd(memory_accessor),
    // 01011111
    // 01100000
    // 01100001
    // 01100010
    // 01100011
    // 01100100
    // 01100101
    // 01100110 LD H, (IX+d)
    |z80, memory_accessor| z80.ld_h_ixd(memory_accessor),
    // 01100111
    // 01101000
    // 01101001
    // 01101010
    // 01101011
    // 01101100
    // 01101101
    // 01101110 LD L, (IX+d)
    |z80, memory_accessor| z80.ld_l_ixd(memory_accessor),
    // 01101111
    // 01110000 LD (IX+d), B
    |z80, memory_accessor| z80.ld_ixd_b(memory_accessor),
    // 01110001 LD (IX+d), C
    |z80, memory_accessor| z80.ld_ixd_c(memory_accessor),
    // 01110010 LD (IX+d), D
    |z80, memory_accessor| z80.ld_ixd_d(memory_accessor),
    // 01110011 LD (IX+d), E
    |z80, memory_accessor| z80.ld_ixd_e(memory_accessor),
    // 01110100 LD (IX+d), H
    |z80, memory_accessor| z80.ld_ixd_h(memory_accessor),
    // 01110101 LD (IX+d), L
    |z80, memory_accessor| z80.ld_ixd_l(memory_accessor),
    // 01110110
    // 01110111 LD (IX+d), A
    |z80, memory_accessor| z80.ld_ixd_a(memory_accessor),
    // 01111000
    // 01111001
    // 01111010
    // 01111011
    // 01111100
    // 01111101
    // 01111110 LD A, (IX+d)
    |z80, memory_accessor| z80.ld_a_ixd(memory_accessor),
    // 01111111
    // 10000000
    // 10000001
    // 10000010
    // 10000011
    // 10000100
    // 10000101
    // 10000110
    // 10000111
    // 10001000
    // 10001001
    // 10001010
    // 10001011
    // 10001100
    // 10001101
    // 10001110
    // 10001111
    // 10010000
    // 10010001
    // 10010010
    // 10010011
    // 10010100
    // 10010101
    // 10010110
    // 10010111
    // 10011000
    // 10011001
    // 10011010
    // 10011011
    // 10011100
    // 10011101
    // 10011110
    // 10011111
    // 10100000
    // 10100001
    // 10100010
    // 10100011
    // 10100100
    // 10100101
    // 10100110
    // 10100111
    // 10101000
    // 10101001
    // 10101010
    // 10101011
    // 10101100
    // 10101101
    // 10101110
    // 10101111
    // 10110000
    // 10110001
    // 10110010
    // 10110011
    // 10110100
    // 10110101
    // 10110110
    // 10110111
    // 10111000
    // 10111001
    // 10111010
    // 10111011
    // 10111100
    // 10111101
    // 10111110
    // 10111111
    // 11000000
    // 11000001
    // 11000010
    // 11000011
    // 11000100
    // 11000101
    // 11000110
    // 11000111
    // 11001000
    // 11001001
    // 11001010
    // 11001011
    // 11001100
    // 11001101
    // 11001110
    // 11001111
    // 11010000
    // 11010001
    // 11010010
    // 11010011
    // 11010100
    // 11010101
    // 11010110
    // 11010111
    // 11011000
    // 11011001
    // 11011010
    // 11011011
    // 11011100
    // 11011101
    // 11011110
    // 11011111
    // 11100000
    // 11100001
    // 11100010
    // 11100011
    // 11100100
    // 11100101
    // 11100110
    // 11100111
    // 11101000
    // 11101001
    // 11101010
    // 11101011
    // 11101100
    // 11101101
    // 11101110
    // 11101111
    // 11110000
    // 11110001
    // 11110010
    // 11110011
    // 11110100
    // 11110101
    // 11110110
    // 11110111
    // 11111000
    // 11111001
    // 11111010
    // 11111011
    // 11111100
    // 11111101
    // 11111110
    // 11111111
];

// DDCB prefix
const IX_BIT_INSTRUCTIONS: [fn(&mut Z80, &mut dyn MemoryAccessor) -> u8; 0] = [];

// ED prefix
const MISC_INSTRUCTIONS: [fn(&mut Z80, &mut dyn MemoryAccessor) -> u8; 4] = [
    // 01000111 LD I, A
    |z80, _| z80.ld_i_a(),
    // 01001111 LD R, A
    |z80, _| z80.ld_r_a(),
    // 01010111 LD A, I
    |z80, _| z80.ld_a_i(),
    // 01011111 LD A, R
    |z80, _| z80.ld_a_r(),
];

// FD prefix
const IY_FUNCTIONS: [fn(&mut Z80, &mut dyn MemoryAccessor) -> u8; 15] = [
    // 00000000
    // 00000001
    // 00000010
    // 00000011
    // 00000100
    // 00000101
    // 00000110
    // 00000111
    // 00001000
    // 00001001
    // 00001010
    // 00001011
    // 00001100
    // 00001101
    // 00001110
    // 00001111
    // 00010000
    // 00010001
    // 00010010
    // 00010011
    // 00010100
    // 00010101
    // 00010110
    // 00010111
    // 00011000
    // 00011001
    // 00011010
    // 00011011
    // 00011100
    // 00011101
    // 00011110
    // 00011111
    // 00100000
    // 00100001
    // 00100010
    // 00100011
    // 00100100
    // 00100101
    // 00100110
    // 00100111
    // 00101000
    // 00101001
    // 00101010
    // 00101011
    // 00101100
    // 00101101
    // 00101110
    // 00101111
    // 00110000
    // 00110001
    // 00110010
    // 00110011
    // 00110100
    // 00110101
    // 00110110 LD (IY+d), n
    |z80, memory_accessor| z80.ld_iyd_n(memory_accessor),
    // 00110111
    // 00111000
    // 00111001
    // 00111010
    // 00111011
    // 00111100
    // 00111101
    // 00111110
    // 00111111
    // 01000000
    // 01000001
    // 01000010
    // 01000011
    // 01000100
    // 01000101
    // 01000110 LD B, (IY+d)
    |z80, memory_accessor| z80.ld_b_iyd(memory_accessor),
    // 01000111
    // 01001000
    // 01001001
    // 01001010
    // 01001011
    // 01001100
    // 01001101
    // 01001110 LD C, (IY+d)
    |z80, memory_accessor| z80.ld_c_iyd(memory_accessor),
    // 01001111
    // 01010000
    // 01010001
    // 01010010
    // 01010011
    // 01010100
    // 01010101
    // 01010110 LD D, (IY+d)
    |z80, memory_accessor| z80.ld_d_iyd(memory_accessor),
    // 01010111
    // 01011000
    // 01011001
    // 01011010
    // 01011011
    // 01011100
    // 01011101
    // 01011110 LD E, (IY+d)
    |z80, memory_accessor| z80.ld_e_iyd(memory_accessor),
    // 01011111
    // 01100000
    // 01100001
    // 01100010
    // 01100011
    // 01100100
    // 01100101
    // 01100110 LD H, (IY+d)
    |z80, memory_accessor| z80.ld_h_iyd(memory_accessor),
    // 01100111
    // 01101000
    // 01101001
    // 01101010
    // 01101011
    // 01101100
    // 01101101
    // 01101110 LD L, (IY+d)
    |z80, memory_accessor| z80.ld_l_iyd(memory_accessor),
    // 01101111
    // 01110000 LD (IY+d), B
    |z80, memory_accessor| z80.ld_iyd_b(memory_accessor),
    // 01110001 LD (IY+d), C
    |z80, memory_accessor| z80.ld_iyd_c(memory_accessor),
    // 01110010 LD (IY+d), D
    |z80, memory_accessor| z80.ld_iyd_d(memory_accessor),
    // 01110011 LD (IY+d), E
    |z80, memory_accessor| z80.ld_iyd_e(memory_accessor),
    // 01110100 LD (IY+d), H
    |z80, memory_accessor| z80.ld_iyd_h(memory_accessor),
    // 01110101 LD (IY+d), L
    |z80, memory_accessor| z80.ld_iyd_l(memory_accessor),
    // 01110110
    // 01110111 LD (IY+d), A
    |z80, memory_accessor| z80.ld_iyd_a(memory_accessor),
    // 01111000
    // 01111001
    // 01111010
    // 01111011
    // 01111100
    // 01111101
    // 01111110 LD A, (IY+d)
    |z80, memory_accessor| z80.ld_a_iyd(memory_accessor),
    // 01111111
    // 10000000
    // 10000001
    // 10000010
    // 10000011
    // 10000100
    // 10000101
    // 10000110
    // 10000111
    // 10001000
    // 10001001
    // 10001010
    // 10001011
    // 10001100
    // 10001101
    // 10001110
    // 10001111
    // 10010000
    // 10010001
    // 10010010
    // 10010011
    // 10010100
    // 10010101
    // 10010110
    // 10010111
    // 10011000
    // 10011001
    // 10011010
    // 10011011
    // 10011100
    // 10011101
    // 10011110
    // 10011111
    // 10100000
    // 10100001
    // 10100010
    // 10100011
    // 10100100
    // 10100101
    // 10100110
    // 10100111
    // 10101000
    // 10101001
    // 10101010
    // 10101011
    // 10101100
    // 10101101
    // 10101110
    // 10101111
    // 10110000
    // 10110001
    // 10110010
    // 10110011
    // 10110100
    // 10110101
    // 10110110
    // 10110111
    // 10111000
    // 10111001
    // 10111010
    // 10111011
    // 10111100
    // 10111101
    // 10111110
    // 10111111
    // 11000000
    // 11000001
    // 11000010
    // 11000011
    // 11000100
    // 11000101
    // 11000110
    // 11000111
    // 11001000
    // 11001001
    // 11001010
    // 11001011
    // 11001100
    // 11001101
    // 11001110
    // 11001111
    // 11010000
    // 11010001
    // 11010010
    // 11010011
    // 11010100
    // 11010101
    // 11010110
    // 11010111
    // 11011000
    // 11011001
    // 11011010
    // 11011011
    // 11011100
    // 11011101
    // 11011110
    // 11011111
    // 11100000
    // 11100001
    // 11100010
    // 11100011
    // 11100100
    // 11100101
    // 11100110
    // 11100111
    // 11101000
    // 11101001
    // 11101010
    // 11101011
    // 11101100
    // 11101101
    // 11101110
    // 11101111
    // 11110000
    // 11110001
    // 11110010
    // 11110011
    // 11110100
    // 11110101
    // 11110110
    // 11110111
    // 11111000
    // 11111001
    // 11111010
    // 11111011
    // 11111100
    // 11111101
    // 11111110
    // 11111111
];

// FDCB prefix
const IY_BIT_INSTRUCTIONS: [fn(&mut Z80, &mut dyn MemoryAccessor) -> u8; 0] = [];

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
    a: u8,
    // Main flag register
    f: u8,
    // Alternate accumulator register
    a_prime: u8,
    // Alternate flag register
    f_prime: u8,
    // General purpose registers
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    // Alternate general purpose registers
    b_prime: u8,
    c_prime: u8,
    d_prime: u8,
    e_prime: u8,
    h_prime: u8,
    l_prime: u8,

    /// Interrupt enable flip flop 1
    iff1: bool,

    /// Interrupt enable flip flop 2
    iff2: bool,
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
            a: 0,
            a_prime: 0,
            f: 0,
            f_prime: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            b_prime: 0,
            c_prime: 0,
            d_prime: 0,
            e_prime: 0,
            h_prime: 0,
            l_prime: 0,
            iff1: true,
            iff2: true,
        }
    }

    pub fn fetch_next_opcode(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let opcode = memory_accessor.read(&self.program_counter);
        self.program_counter += 1;
        opcode
    }

    pub fn process_next_instruction(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let mut t_states: u8 = 0;

        // Spend 4 ticks fetching the next instruction
        let opcode = self.fetch_next_opcode(memory_accessor);
        t_states += 4;

        let opcode_function = MAIN_FUNCTIONS[opcode as usize];
        t_states += opcode_function(self, memory_accessor);

        return t_states;
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
        let a_high = self.a & 0xF0;
        let a_low = self.a & 0x0F;
        let diff: u8 = match (c_flag_set(&self.f), a_high, h_flag_set(&self.f), a_low) {
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

        let new_c_flag: bool = match (c_flag_set(&self.f), a_high, a_low) {
            (false, a_high, a_low) if a_high <= 0x09 && a_low <= 0x09 => false,
            (false, a_high, a_low) if a_high <= 0x08 && a_low >= 0x0A => false,
            (false, a_high, a_low) if a_high >= 0x09 && a_low >= 0x0A => true,
            (false, a_high, a_low) if a_high >= 0x0A && a_low <= 0x09 => true,
            (true, _, _) => true,
            _ => c_flag_set(&self.f),
        };

        let new_h_flag: bool = match (n_flag_set(&self.f), h_flag_set(&self.f), a_low) {
            (false, _, a_low) if a_low <= 0x09 => false,
            (false, _, a_low) if a_low >= 0x0A => true,
            (true, false, _) => false,
            (true, true, a_low) if a_low >= 0x06 => false,
            (true, true, a_low) if a_low <= 0x05 => true,
            _ => h_flag_set(&self.f),
        };

        self.a = self.a + diff;

        if new_c_flag {
            set_c_flag(&mut self.f);
        } else {
            unset_c_flag(&mut self.f);
        }

        if new_h_flag {
            set_h_flag(&mut self.f);
        } else {
            unset_h_flag(&mut self.f);
        }

        // SF, YF, XF are copies of bit 7,5,3 of the result respectively;
        self.f = self.f & 0b10101011u8 + self.a & 0b01010100u8;

        // ZF is set according to the result and NF is always unchanged.
        if self.a == 0 {
            set_z_flag(&mut self.f);
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

    pub struct Ram<'a> {
        bytes: &'a mut [u8],
    }

    impl<'a> Ram<'a> {
        pub fn new(bytes: &'a mut [u8]) -> Ram {
            Ram { bytes }
        }
    }

    impl<'a> MemoryAccessor for Ram<'a> {
        fn read(&self, address: &u16) -> u8 {
            self.bytes[*address as usize]
        }

        fn write(&mut self, address: &u16, data: &u8) {
            self.bytes[*address as usize] = *data;
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
            // Until all instructions are populated, the opcode array will have
            // LD H, n at the wrong location.
            0x09, // LD H, n
            0xDD,
        ];

        let ram = &mut Ram::new(&mut bytes);

        let mut z80 = Z80::new();

        let t_states = z80.process_next_instruction(ram);

        assert_eq!(11, t_states);
        assert_eq!(bytes[1], z80.h);
    }
}
