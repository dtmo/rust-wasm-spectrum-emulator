mod eight_bit_load_group;
mod exchange_block_transfer;
mod general_purpose_group;
mod register_flags;
mod sixteen_bit_load_group;

// Official Z80 documentation: https://www.zilog.com/docs/z80/um0080.pdf
// Unofficial undocumented functionality documentation: http://www.z80.info/zip/z80-documented.pdf
// Integration test suites: https://mdfs.net/Software/Z80/Exerciser/Spectrum/

const MAIN_FUNCTIONS: [fn(&mut Z80, &mut dyn Z80Memory) -> u8; 256] = [
    // 00000000 00: NOP
    |z80, _| z80.nop(),
    // 00000001 01: LD BC nn
    |z80, mem| z80.ld_bc_nn(mem),
    // 00000010 02: LD (BC), A
    Z80::ld_mem_bc_a,
    // 00000011 03:
    |_, _| Z80::unsupported_operation(),
    // 00000100 04:
    |_, _| Z80::unsupported_operation(),
    // 00000101 05:
    |_, _| Z80::unsupported_operation(),
    // 00000110 06: LD B, n
    Z80::ld_b_n,
    // 00000111 07:
    |_, _| Z80::unsupported_operation(),
    // 00001000 08: LD AF, AF'
    |z80, _| z80.ex_af_afp(),
    // 00001001 09:
    |_, _| Z80::unsupported_operation(),
    // 00001010 0A: LA A, (BC)
    |z80, mem| z80.ld_a_mem_bc(mem),
    // 00001011 0B:
    |_, _| Z80::unsupported_operation(),
    // 00001100 0C:
    |_, _| Z80::unsupported_operation(),
    // 00001101 0D:
    |_, _| Z80::unsupported_operation(),
    // 00001110 0E: LD C, n
    Z80::ld_c_n,
    // 00001111 0F:
    |_, _| Z80::unsupported_operation(),
    // 00010000 10:
    |_, _| Z80::unsupported_operation(),
    // 00010001 11: LD DE nn
    |z80, mem| z80.ld_de_nn(mem),
    // 00010010 12: LD (DE), A
    Z80::ld_mem_de_a,
    // 00010011 13:
    |_, _| Z80::unsupported_operation(),
    // 00010100 14:
    |_, _| Z80::unsupported_operation(),
    // 00010101 15:
    |_, _| Z80::unsupported_operation(),
    // 00010110 16: LD D, n
    Z80::ld_d_n,
    // 00010111 17:
    |_, _| Z80::unsupported_operation(),
    // 00011000 18:
    |_, _| Z80::unsupported_operation(),
    // 00011001 19:
    |_, _| Z80::unsupported_operation(),
    // 00011010 1A: LA A, (DE)
    |z80, mem| z80.ld_a_mem_de(mem),
    // 00011011 1B:
    |_, _| Z80::unsupported_operation(),
    // 00011100 1C:
    |_, _| Z80::unsupported_operation(),
    // 00011101 1D:
    |_, _| Z80::unsupported_operation(),
    // 00011110 1E: LD E, n
    Z80::ld_e_n,
    // 00011111 1F:
    |_, _| Z80::unsupported_operation(),
    // 00100000 20:
    |_, _| Z80::unsupported_operation(),
    // 00100001 21: LD HL nn
    |z80, mem| z80.ld_hl_nn(mem),
    // 00100010 22: LD (nn), HL
    Z80::ld_mem_nn_hl,
    // 00100011 23:
    |_, _| Z80::unsupported_operation(),
    // 00100100 24:
    |_, _| Z80::unsupported_operation(),
    // 00100101 25:
    |_, _| Z80::unsupported_operation(),
    // 00100110 26: LD H, n
    Z80::ld_h_n,
    // 00100111 27:
    |z80, _| z80.daa(),
    // 00101000 28:
    |_, _| Z80::unsupported_operation(),
    // 00101001 29:
    |_, _| Z80::unsupported_operation(),
    // 00101010 2A: LD HL, (nn)
    |z80, mem| z80.ld_hl_mem_nn(mem),
    // 00101011 2B:
    |_, _| Z80::unsupported_operation(),
    // 00101100 2C:
    |_, _| Z80::unsupported_operation(),
    // 00101101 2D:
    |_, _| Z80::unsupported_operation(),
    // 00101110 2E: LD L, n
    Z80::ld_l_n,
    // 00101111 2F: CPL
    |z80, _| z80.cpl(),
    // 00110000 30:
    |_, _| Z80::unsupported_operation(),
    // 00110001 31: LD SP nn
    |z80, mem| z80.ld_sp_nn(mem),
    // 00110010 32: LD (nn), A
    Z80::ld_mem_nn_a,
    // 00110011 33:
    |_, _| Z80::unsupported_operation(),
    // 00110100 34:
    |_, _| Z80::unsupported_operation(),
    // 00110101 35:
    |_, _| Z80::unsupported_operation(),
    // 00110110 36: LD (HL), n
    |z80, mem| z80.ld_mem_hl_n(mem),
    // 00110111 37:
    |_, _| Z80::unsupported_operation(),
    // 00111000 38:
    |_, _| Z80::unsupported_operation(),
    // 00111001 39:
    |_, _| Z80::unsupported_operation(),
    // 00111010 3A: LD A, (nn)
    |z80, mem| z80.ld_a_mem_nn(mem),
    // 00111011 3B:
    |_, _| Z80::unsupported_operation(),
    // 00111100 3C:
    |_, _| Z80::unsupported_operation(),
    // 00111101 3D:
    |_, _| Z80::unsupported_operation(),
    // 00111110 3E: LD A, n
    Z80::ld_a_n,
    // 00111111 3F:
    |_, _| Z80::unsupported_operation(),
    // 01000000 40: LD B, B
    |z80, _| z80.ld_b_b(),
    // 01000001 41: LD B, C
    |z80, _| z80.ld_b_c(),
    // 01000010 42: LD B, D
    |z80, _| z80.ld_b_d(),
    // 01000011 43: LD B, E
    |z80, _| z80.ld_b_e(),
    // 01000100 44: LD B, H
    |z80, _| z80.ld_b_h(),
    // 01000101 45: LD B, L
    |z80, _| z80.ld_b_l(),
    // 01000110 46: LD B, (HL)
    |z80, mem| z80.ld_b_mem_hl(mem),
    // 01000111 47: LD B, A
    |z80, _| z80.ld_b_a(),
    // 01001000 48: LD C, B
    |z80, _| z80.ld_c_b(),
    // 01001001 49: LD C, C
    |z80, _| z80.ld_c_c(),
    // 01001010 4A: LD C, D
    |z80, _| z80.ld_c_d(),
    // 01001011 4B: LD C, E
    |z80, _| z80.ld_c_e(),
    // 01001100 4C: LD C, H
    |z80, _| z80.ld_c_h(),
    // 01001101 4D: LD C, L
    |z80, _| z80.ld_c_l(),
    // 01001110 4E: LD C, (HL)
    |z80, mem| z80.ld_c_mem_hl(mem),
    // 01001111 4F: LD C, A
    |z80, _| z80.ld_c_a(),
    // 01010000 50: LD D, B
    |z80, _| z80.ld_d_b(),
    // 01010001 51: LD D, C
    |z80, _| z80.ld_d_c(),
    // 01010010 52: LD D, D
    |z80, _| z80.ld_d_d(),
    // 01010011 53: LD D, E
    |z80, _| z80.ld_d_e(),
    // 01010100 54: LD D, H
    |z80, _| z80.ld_d_h(),
    // 01010101 55: LD D, L
    |z80, _| z80.ld_d_l(),
    // 01010110 56: LD D, (HL)
    |z80, mem| z80.ld_d_mem_hl(mem),
    // 01010111 57: LD D, A
    |z80, _| z80.ld_d_a(),
    // 01011000 58: LD E, B
    |z80, _| z80.ld_e_b(),
    // 01011001 59: LD E, C
    |z80, _| z80.ld_e_c(),
    // 01011010 5A: LD E, D
    |z80, _| z80.ld_e_d(),
    // 01011011 5B: LD E, E
    |z80, _| z80.ld_e_e(),
    // 01011100 5C: LD E, H
    |z80, _| z80.ld_e_h(),
    // 01011101 5D: LD E, L
    |z80, _| z80.ld_e_l(),
    // 01011110 5E: LD E, (HL)
    |z80, mem| z80.ld_e_mem_hl(mem),
    // 01011111 5F: LD E, A
    |z80, _| z80.ld_e_a(),
    // 01100000 60: LD H, B
    |z80, _| z80.ld_h_b(),
    // 01100001 61: LD H, C
    |z80, _| z80.ld_h_c(),
    // 01100010 62: LD H, D
    |z80, _| z80.ld_h_d(),
    // 01100011 63: LD H, E
    |z80, _| z80.ld_h_e(),
    // 01100100 64: LD H, H
    |z80, _| z80.ld_h_h(),
    // 01100101 65: LD H, L
    |z80, _| z80.ld_h_l(),
    // 01100110 66: LD H, (HL)
    |z80, mem| z80.ld_h_mem_hl(mem),
    // 01100111 67: LD H, A
    |z80, _| z80.ld_h_a(),
    // 01101000 68: LD L, B
    |z80, _| z80.ld_l_b(),
    // 01101001 69: LD L, C
    |z80, _| z80.ld_l_c(),
    // 01101010 6A: LD L, D
    |z80, _| z80.ld_l_d(),
    // 01101011 6B: LD L, E
    |z80, _| z80.ld_l_e(),
    // 01101100 6C: LD L, H
    |z80, _| z80.ld_l_h(),
    // 01101101 6D: LD L, L
    |z80, _| z80.ld_l_l(),
    // 01101110 6E: LD L, (HL)
    |z80, mem| z80.ld_l_mem_hl(mem),
    // 01101111 6F: LD L, A
    |z80, _| z80.ld_l_a(),
    // 01110000 70: LD (HL), B
    |z80, mem| z80.ld_mem_hl_b(mem),
    // 01110001 71: LD (HL), C
    |z80, mem| z80.ld_mem_hl_c(mem),
    // 01110010 72: LD (HL), D
    |z80, mem| z80.ld_mem_hl_d(mem),
    // 01110011 73: LD (HL), E
    |z80, mem| z80.ld_mem_hl_e(mem),
    // 01110100 74: LD (HL), H
    |z80, mem| z80.ld_mem_hl_h(mem),
    // 01110101 75: LD (HL), L
    |z80, mem| z80.ld_mem_hl_l(mem),
    // 01110110 76:
    |_, _| Z80::unsupported_operation(),
    // 01110111 77: LD (HL), A
    |z80, mem| z80.ld_mem_hl_a(mem),
    // 01111000 78: LD A, B
    |z80, _| z80.ld_a_b(),
    // 01111001 79: LD A, C
    |z80, _| z80.ld_a_c(),
    // 01111010 7A: LD A, D
    |z80, _| z80.ld_a_d(),
    // 01111011 7B: LD A, E
    |z80, _| z80.ld_a_e(),
    // 01111100 7C: LD A, H
    |z80, _| z80.ld_a_h(),
    // 01111101 7D: LD A, L
    |z80, _| z80.ld_a_l(),
    // 01111110 7E: LD A, (HL)
    |z80, mem| z80.ld_a_mem_hl(mem),
    // 01111111 7F: LD A, A
    |z80, _| z80.ld_a_a(),
    // 10000000 80:
    |_, _| Z80::unsupported_operation(),
    // 10000001 81:
    |_, _| Z80::unsupported_operation(),
    // 10000010 82:
    |_, _| Z80::unsupported_operation(),
    // 10000011 83:
    |_, _| Z80::unsupported_operation(),
    // 10000100 84:
    |_, _| Z80::unsupported_operation(),
    // 10000101 85:
    |_, _| Z80::unsupported_operation(),
    // 10000110 86:
    |_, _| Z80::unsupported_operation(),
    // 10000111 87:
    |_, _| Z80::unsupported_operation(),
    // 10001000 88:
    |_, _| Z80::unsupported_operation(),
    // 10001001 89:
    |_, _| Z80::unsupported_operation(),
    // 10001010 8A:
    |_, _| Z80::unsupported_operation(),
    // 10001011 8B:
    |_, _| Z80::unsupported_operation(),
    // 10001100 8C:
    |_, _| Z80::unsupported_operation(),
    // 10001101 8D:
    |_, _| Z80::unsupported_operation(),
    // 10001110 8E:
    |_, _| Z80::unsupported_operation(),
    // 10001111 8F:
    |_, _| Z80::unsupported_operation(),
    // 10010000 90:
    |_, _| Z80::unsupported_operation(),
    // 10010001 91:
    |_, _| Z80::unsupported_operation(),
    // 10010010 92:
    |_, _| Z80::unsupported_operation(),
    // 10010011 93:
    |_, _| Z80::unsupported_operation(),
    // 10010100 94:
    |_, _| Z80::unsupported_operation(),
    // 10010101 95:
    |_, _| Z80::unsupported_operation(),
    // 10010110 96:
    |_, _| Z80::unsupported_operation(),
    // 10010111 97:
    |_, _| Z80::unsupported_operation(),
    // 10011000 98:
    |_, _| Z80::unsupported_operation(),
    // 10011001 99:
    |_, _| Z80::unsupported_operation(),
    // 10011010 9A:
    |_, _| Z80::unsupported_operation(),
    // 10011011 9B:
    |_, _| Z80::unsupported_operation(),
    // 10011100 9C:
    |_, _| Z80::unsupported_operation(),
    // 10011101 9D:
    |_, _| Z80::unsupported_operation(),
    // 10011110 9E:
    |_, _| Z80::unsupported_operation(),
    // 10011111 9F:
    |_, _| Z80::unsupported_operation(),
    // 10100000 A0:
    |_, _| Z80::unsupported_operation(),
    // 10100001 A1:
    |_, _| Z80::unsupported_operation(),
    // 10100010 A2:
    |_, _| Z80::unsupported_operation(),
    // 10100011 A3:
    |_, _| Z80::unsupported_operation(),
    // 10100100 A4:
    |_, _| Z80::unsupported_operation(),
    // 10100101 A5:
    |_, _| Z80::unsupported_operation(),
    // 10100110 A6:
    |_, _| Z80::unsupported_operation(),
    // 10100111 A7:
    |_, _| Z80::unsupported_operation(),
    // 10101000 A8:
    |_, _| Z80::unsupported_operation(),
    // 10101001 A9:
    |_, _| Z80::unsupported_operation(),
    // 10101010 AA:
    |_, _| Z80::unsupported_operation(),
    // 10101011 AB:
    |_, _| Z80::unsupported_operation(),
    // 10101100 AC:
    |_, _| Z80::unsupported_operation(),
    // 10101101 AD:
    |_, _| Z80::unsupported_operation(),
    // 10101110 AE:
    |_, _| Z80::unsupported_operation(),
    // 10101111 AF:
    |_, _| Z80::unsupported_operation(),
    // 10110000 B0:
    |_, _| Z80::unsupported_operation(),
    // 10110001 B1:
    |_, _| Z80::unsupported_operation(),
    // 10110010 B2:
    |_, _| Z80::unsupported_operation(),
    // 10110011 B3:
    |_, _| Z80::unsupported_operation(),
    // 10110100 B4:
    |_, _| Z80::unsupported_operation(),
    // 10110101 B5:
    |_, _| Z80::unsupported_operation(),
    // 10110110 B6:
    |_, _| Z80::unsupported_operation(),
    // 10110111 B7:
    |_, _| Z80::unsupported_operation(),
    // 10111000 B8:
    |_, _| Z80::unsupported_operation(),
    // 10111001 B9:
    |_, _| Z80::unsupported_operation(),
    // 10111010 BA:
    |_, _| Z80::unsupported_operation(),
    // 10111011 BB:
    |_, _| Z80::unsupported_operation(),
    // 10111100 BC:
    |_, _| Z80::unsupported_operation(),
    // 10111101 BD:
    |_, _| Z80::unsupported_operation(),
    // 10111110 BE:
    |_, _| Z80::unsupported_operation(),
    // 10111111 BF:
    |_, _| Z80::unsupported_operation(),
    // 11000000 C0:
    |_, _| Z80::unsupported_operation(),
    // 11000001 C1: POP BC
    |z80, mem| z80.pop_qqbc(mem),
    // 11000010 C2:
    |_, _| Z80::unsupported_operation(),
    // 11000011 C3:
    |_, _| Z80::unsupported_operation(),
    // 11000100 C4:
    |_, _| Z80::unsupported_operation(),
    // 11000101 C5: PUSH BC
    Z80::push_qqbc,
    // 11000110 C6:
    |_, _| Z80::unsupported_operation(),
    // 11000111 C7:
    |_, _| Z80::unsupported_operation(),
    // 11001000 C8:
    |_, _| Z80::unsupported_operation(),
    // 11001001 C9:
    |_, _| Z80::unsupported_operation(),
    // 11001010 CA:
    |_, _| Z80::unsupported_operation(),
    // 11001011 CB:
    |_, _| Z80::unsupported_operation(),
    // 11001100 CC:
    |_, _| Z80::unsupported_operation(),
    // 11001101 CD:
    |_, _| Z80::unsupported_operation(),
    // 11001110 CE:
    |_, _| Z80::unsupported_operation(),
    // 11001111 CF:
    |_, _| Z80::unsupported_operation(),
    // 11010000 D0:
    |_, _| Z80::unsupported_operation(),
    // 11010001 D1: POP DE
    |z80, mem| z80.pop_qqde(mem),
    // 11010010 D2:
    |_, _| Z80::unsupported_operation(),
    // 11010011 D3:
    |_, _| Z80::unsupported_operation(),
    // 11010100 D4:
    |_, _| Z80::unsupported_operation(),
    // 11010101 D5: PUSH DE
    Z80::push_qqde,
    // 11010110 D6:
    |_, _| Z80::unsupported_operation(),
    // 11010111 D7:
    |_, _| Z80::unsupported_operation(),
    // 11011000 D8:
    |_, _| Z80::unsupported_operation(),
    // 11011001 D9: EXX
    |z80, _| z80.exx(),
    // 11011010 DA:
    |_, _| Z80::unsupported_operation(),
    // 11011011 DB:
    |_, _| Z80::unsupported_operation(),
    // 11011100 DC:
    |_, _| Z80::unsupported_operation(),
    // 11011101 DD:
    |_, _| Z80::unsupported_operation(),
    // 11011110 DE:
    |_, _| Z80::unsupported_operation(),
    // 11011111 DF:
    |_, _| Z80::unsupported_operation(),
    // 11100000 E0:
    |_, _| Z80::unsupported_operation(),
    // 11100001 E1: POP HL
    |z80, mem| z80.pop_qqhl(mem),
    // 11100010 E2:
    |_, _| Z80::unsupported_operation(),
    // 11100011 E3: EX (SP), HL
    Z80::ex_mem_sp_hl,
    // 11100100 E4:
    |_, _| Z80::unsupported_operation(),
    // 11100101 E5: PUSH HL
    Z80::push_qqhl,
    // 11100110 E6:
    |_, _| Z80::unsupported_operation(),
    // 11100111 E7:
    |_, _| Z80::unsupported_operation(),
    // 11101000 E8:
    |_, _| Z80::unsupported_operation(),
    // 11101001 E9:
    |_, _| Z80::unsupported_operation(),
    // 11101010 EA:
    |_, _| Z80::unsupported_operation(),
    // 11101011 EB: EX DE, HL
    |z80, _| z80.ex_de_hl(),
    // 11101100 EC:
    |_, _| Z80::unsupported_operation(),
    // 11101101 ED: Extended function prefix
    |z80, mem| z80.process_extended_instruction(mem),
    // 11101110 EE:
    |_, _| Z80::unsupported_operation(),
    // 11101111 EF:
    |_, _| Z80::unsupported_operation(),
    // 11110000 F0:
    |_, _| Z80::unsupported_operation(),
    // 11110001 F1: POP AF
    |z80, mem| z80.pop_qqaf(mem),
    // 11110010 F2:
    |_, _| Z80::unsupported_operation(),
    // 11110011 F3:
    |_, _| Z80::unsupported_operation(),
    // 11110100 F4:
    |_, _| Z80::unsupported_operation(),
    // 11110101 F5: PUSH AF
    Z80::push_qqaf,
    // 11110110 F6:
    |_, _| Z80::unsupported_operation(),
    // 11110111 F7:
    |_, _| Z80::unsupported_operation(),
    // 11111000 F8:
    |_, _| Z80::unsupported_operation(),
    // 11111001 F9: LD SP, HL
    |z80, _| z80.ld_sp_hl(),
    // 11111010 FA:
    |_, _| Z80::unsupported_operation(),
    // 11111011 FB:
    |_, _| Z80::unsupported_operation(),
    // 11111100 FC:
    |_, _| Z80::unsupported_operation(),
    // 11111101 FD:
    |_, _| Z80::unsupported_operation(),
    // 11111110 FE:
    |_, _| Z80::unsupported_operation(),
    // 11111111 FF:
    |_, _| Z80::unsupported_operation(),
];

// CB prefix
const BIT_INSTRUCTIONS: [fn(&mut Z80, &mut dyn Z80Memory) -> u8; 256] = [
    // 00000000 00:
    |_, _| Z80::unsupported_operation(),
    // 00000001 01:
    |_, _| Z80::unsupported_operation(),
    // 00000010 02:
    |_, _| Z80::unsupported_operation(),
    // 00000011 03:
    |_, _| Z80::unsupported_operation(),
    // 00000100 04:
    |_, _| Z80::unsupported_operation(),
    // 00000101 05:
    |_, _| Z80::unsupported_operation(),
    // 00000110 06:
    |_, _| Z80::unsupported_operation(),
    // 00000111 07:
    |_, _| Z80::unsupported_operation(),
    // 00001000 08:
    |_, _| Z80::unsupported_operation(),
    // 00001001 09:
    |_, _| Z80::unsupported_operation(),
    // 00001010 0A:
    |_, _| Z80::unsupported_operation(),
    // 00001011 0B:
    |_, _| Z80::unsupported_operation(),
    // 00001100 0C:
    |_, _| Z80::unsupported_operation(),
    // 00001101 0D:
    |_, _| Z80::unsupported_operation(),
    // 00001110 0E:
    |_, _| Z80::unsupported_operation(),
    // 00001111 0F:
    |_, _| Z80::unsupported_operation(),
    // 00010000 10:
    |_, _| Z80::unsupported_operation(),
    // 00010001 11:
    |_, _| Z80::unsupported_operation(),
    // 00010010 12:
    |_, _| Z80::unsupported_operation(),
    // 00010011 13:
    |_, _| Z80::unsupported_operation(),
    // 00010100 14:
    |_, _| Z80::unsupported_operation(),
    // 00010101 15:
    |_, _| Z80::unsupported_operation(),
    // 00010110 16:
    |_, _| Z80::unsupported_operation(),
    // 00010111 17:
    |_, _| Z80::unsupported_operation(),
    // 00011000 18:
    |_, _| Z80::unsupported_operation(),
    // 00011001 19:
    |_, _| Z80::unsupported_operation(),
    // 00011010 1A:
    |_, _| Z80::unsupported_operation(),
    // 00011011 1B:
    |_, _| Z80::unsupported_operation(),
    // 00011100 1C:
    |_, _| Z80::unsupported_operation(),
    // 00011101 1D:
    |_, _| Z80::unsupported_operation(),
    // 00011110 1E:
    |_, _| Z80::unsupported_operation(),
    // 00011111 1F:
    |_, _| Z80::unsupported_operation(),
    // 00100000 20:
    |_, _| Z80::unsupported_operation(),
    // 00100001 21:
    |_, _| Z80::unsupported_operation(),
    // 00100010 22:
    |_, _| Z80::unsupported_operation(),
    // 00100011 23:
    |_, _| Z80::unsupported_operation(),
    // 00100100 24:
    |_, _| Z80::unsupported_operation(),
    // 00100101 25:
    |_, _| Z80::unsupported_operation(),
    // 00100110 26:
    |_, _| Z80::unsupported_operation(),
    // 00100111 27:
    |_, _| Z80::unsupported_operation(),
    // 00101000 28:
    |_, _| Z80::unsupported_operation(),
    // 00101001 29:
    |_, _| Z80::unsupported_operation(),
    // 00101010 2A:
    |_, _| Z80::unsupported_operation(),
    // 00101011 2B:
    |_, _| Z80::unsupported_operation(),
    // 00101100 2C:
    |_, _| Z80::unsupported_operation(),
    // 00101101 2D:
    |_, _| Z80::unsupported_operation(),
    // 00101110 2E:
    |_, _| Z80::unsupported_operation(),
    // 00101111 2F:
    |_, _| Z80::unsupported_operation(),
    // 00110000 30:
    |_, _| Z80::unsupported_operation(),
    // 00110001 31:
    |_, _| Z80::unsupported_operation(),
    // 00110010 32:
    |_, _| Z80::unsupported_operation(),
    // 00110011 33:
    |_, _| Z80::unsupported_operation(),
    // 00110100 34:
    |_, _| Z80::unsupported_operation(),
    // 00110101 35:
    |_, _| Z80::unsupported_operation(),
    // 00110110 36:
    |_, _| Z80::unsupported_operation(),
    // 00110111 37:
    |_, _| Z80::unsupported_operation(),
    // 00111000 38:
    |_, _| Z80::unsupported_operation(),
    // 00111001 39:
    |_, _| Z80::unsupported_operation(),
    // 00111010 3A:
    |_, _| Z80::unsupported_operation(),
    // 00111011 3B:
    |_, _| Z80::unsupported_operation(),
    // 00111100 3C:
    |_, _| Z80::unsupported_operation(),
    // 00111101 3D:
    |_, _| Z80::unsupported_operation(),
    // 00111110 3E:
    |_, _| Z80::unsupported_operation(),
    // 00111111 3F:
    |_, _| Z80::unsupported_operation(),
    // 01000000 40:
    |_, _| Z80::unsupported_operation(),
    // 01000001 41:
    |_, _| Z80::unsupported_operation(),
    // 01000010 42:
    |_, _| Z80::unsupported_operation(),
    // 01000011 43:
    |_, _| Z80::unsupported_operation(),
    // 01000100 44:
    |_, _| Z80::unsupported_operation(),
    // 01000101 45:
    |_, _| Z80::unsupported_operation(),
    // 01000110 46:
    |_, _| Z80::unsupported_operation(),
    // 01000111 47:
    |_, _| Z80::unsupported_operation(),
    // 01001000 48:
    |_, _| Z80::unsupported_operation(),
    // 01001001 49:
    |_, _| Z80::unsupported_operation(),
    // 01001010 4A:
    |_, _| Z80::unsupported_operation(),
    // 01001011 4B:
    |_, _| Z80::unsupported_operation(),
    // 01001100 4C:
    |_, _| Z80::unsupported_operation(),
    // 01001101 4D:
    |_, _| Z80::unsupported_operation(),
    // 01001110 4E:
    |_, _| Z80::unsupported_operation(),
    // 01001111 4F:
    |_, _| Z80::unsupported_operation(),
    // 01010000 50:
    |_, _| Z80::unsupported_operation(),
    // 01010001 51:
    |_, _| Z80::unsupported_operation(),
    // 01010010 52:
    |_, _| Z80::unsupported_operation(),
    // 01010011 53:
    |_, _| Z80::unsupported_operation(),
    // 01010100 54:
    |_, _| Z80::unsupported_operation(),
    // 01010101 55:
    |_, _| Z80::unsupported_operation(),
    // 01010110 56:
    |_, _| Z80::unsupported_operation(),
    // 01010111 57:
    |_, _| Z80::unsupported_operation(),
    // 01011000 58:
    |_, _| Z80::unsupported_operation(),
    // 01011001 59:
    |_, _| Z80::unsupported_operation(),
    // 01011010 5A:
    |_, _| Z80::unsupported_operation(),
    // 01011011 5B:
    |_, _| Z80::unsupported_operation(),
    // 01011100 5C:
    |_, _| Z80::unsupported_operation(),
    // 01011101 5D:
    |_, _| Z80::unsupported_operation(),
    // 01011110 5E:
    |_, _| Z80::unsupported_operation(),
    // 01011111 5F:
    |_, _| Z80::unsupported_operation(),
    // 01100000 60:
    |_, _| Z80::unsupported_operation(),
    // 01100001 61:
    |_, _| Z80::unsupported_operation(),
    // 01100010 62:
    |_, _| Z80::unsupported_operation(),
    // 01100011 63:
    |_, _| Z80::unsupported_operation(),
    // 01100100 64:
    |_, _| Z80::unsupported_operation(),
    // 01100101 65:
    |_, _| Z80::unsupported_operation(),
    // 01100110 66:
    |_, _| Z80::unsupported_operation(),
    // 01100111 67:
    |_, _| Z80::unsupported_operation(),
    // 01101000 68:
    |_, _| Z80::unsupported_operation(),
    // 01101001 69:
    |_, _| Z80::unsupported_operation(),
    // 01101010 6A:
    |_, _| Z80::unsupported_operation(),
    // 01101011 6B:
    |_, _| Z80::unsupported_operation(),
    // 01101100 6C:
    |_, _| Z80::unsupported_operation(),
    // 01101101 6D:
    |_, _| Z80::unsupported_operation(),
    // 01101110 6E:
    |_, _| Z80::unsupported_operation(),
    // 01101111 6F:
    |_, _| Z80::unsupported_operation(),
    // 01110000 70:
    |_, _| Z80::unsupported_operation(),
    // 01110001 71:
    |_, _| Z80::unsupported_operation(),
    // 01110010 72:
    |_, _| Z80::unsupported_operation(),
    // 01110011 73:
    |_, _| Z80::unsupported_operation(),
    // 01110100 74:
    |_, _| Z80::unsupported_operation(),
    // 01110101 75:
    |_, _| Z80::unsupported_operation(),
    // 01110110 76:
    |_, _| Z80::unsupported_operation(),
    // 01110111 77:
    |_, _| Z80::unsupported_operation(),
    // 01111000 78:
    |_, _| Z80::unsupported_operation(),
    // 01111001 79:
    |_, _| Z80::unsupported_operation(),
    // 01111010 7A:
    |_, _| Z80::unsupported_operation(),
    // 01111011 7B:
    |_, _| Z80::unsupported_operation(),
    // 01111100 7C:
    |_, _| Z80::unsupported_operation(),
    // 01111101 7D:
    |_, _| Z80::unsupported_operation(),
    // 01111110 7E:
    |_, _| Z80::unsupported_operation(),
    // 01111111 7F:
    |_, _| Z80::unsupported_operation(),
    // 10000000 80:
    |_, _| Z80::unsupported_operation(),
    // 10000001 81:
    |_, _| Z80::unsupported_operation(),
    // 10000010 82:
    |_, _| Z80::unsupported_operation(),
    // 10000011 83:
    |_, _| Z80::unsupported_operation(),
    // 10000100 84:
    |_, _| Z80::unsupported_operation(),
    // 10000101 85:
    |_, _| Z80::unsupported_operation(),
    // 10000110 86:
    |_, _| Z80::unsupported_operation(),
    // 10000111 87:
    |_, _| Z80::unsupported_operation(),
    // 10001000 88:
    |_, _| Z80::unsupported_operation(),
    // 10001001 89:
    |_, _| Z80::unsupported_operation(),
    // 10001010 8A:
    |_, _| Z80::unsupported_operation(),
    // 10001011 8B:
    |_, _| Z80::unsupported_operation(),
    // 10001100 8C:
    |_, _| Z80::unsupported_operation(),
    // 10001101 8D:
    |_, _| Z80::unsupported_operation(),
    // 10001110 8E:
    |_, _| Z80::unsupported_operation(),
    // 10001111 8F:
    |_, _| Z80::unsupported_operation(),
    // 10010000 90:
    |_, _| Z80::unsupported_operation(),
    // 10010001 91:
    |_, _| Z80::unsupported_operation(),
    // 10010010 92:
    |_, _| Z80::unsupported_operation(),
    // 10010011 93:
    |_, _| Z80::unsupported_operation(),
    // 10010100 94:
    |_, _| Z80::unsupported_operation(),
    // 10010101 95:
    |_, _| Z80::unsupported_operation(),
    // 10010110 96:
    |_, _| Z80::unsupported_operation(),
    // 10010111 97:
    |_, _| Z80::unsupported_operation(),
    // 10011000 98:
    |_, _| Z80::unsupported_operation(),
    // 10011001 99:
    |_, _| Z80::unsupported_operation(),
    // 10011010 9A:
    |_, _| Z80::unsupported_operation(),
    // 10011011 9B:
    |_, _| Z80::unsupported_operation(),
    // 10011100 9C:
    |_, _| Z80::unsupported_operation(),
    // 10011101 9D:
    |_, _| Z80::unsupported_operation(),
    // 10011110 9E:
    |_, _| Z80::unsupported_operation(),
    // 10011111 9F:
    |_, _| Z80::unsupported_operation(),
    // 10100000 A0:
    |_, _| Z80::unsupported_operation(),
    // 10100001 A1:
    |_, _| Z80::unsupported_operation(),
    // 10100010 A2:
    |_, _| Z80::unsupported_operation(),
    // 10100011 A3:
    |_, _| Z80::unsupported_operation(),
    // 10100100 A4:
    |_, _| Z80::unsupported_operation(),
    // 10100101 A5:
    |_, _| Z80::unsupported_operation(),
    // 10100110 A6:
    |_, _| Z80::unsupported_operation(),
    // 10100111 A7:
    |_, _| Z80::unsupported_operation(),
    // 10101000 A8:
    |_, _| Z80::unsupported_operation(),
    // 10101001 A9:
    |_, _| Z80::unsupported_operation(),
    // 10101010 AA:
    |_, _| Z80::unsupported_operation(),
    // 10101011 AB:
    |_, _| Z80::unsupported_operation(),
    // 10101100 AC:
    |_, _| Z80::unsupported_operation(),
    // 10101101 AD:
    |_, _| Z80::unsupported_operation(),
    // 10101110 AE:
    |_, _| Z80::unsupported_operation(),
    // 10101111 AF:
    |_, _| Z80::unsupported_operation(),
    // 10110000 B0:
    |_, _| Z80::unsupported_operation(),
    // 10110001 B1:
    |_, _| Z80::unsupported_operation(),
    // 10110010 B2:
    |_, _| Z80::unsupported_operation(),
    // 10110011 B3:
    |_, _| Z80::unsupported_operation(),
    // 10110100 B4:
    |_, _| Z80::unsupported_operation(),
    // 10110101 B5:
    |_, _| Z80::unsupported_operation(),
    // 10110110 B6:
    |_, _| Z80::unsupported_operation(),
    // 10110111 B7:
    |_, _| Z80::unsupported_operation(),
    // 10111000 B8:
    |_, _| Z80::unsupported_operation(),
    // 10111001 B9:
    |_, _| Z80::unsupported_operation(),
    // 10111010 BA:
    |_, _| Z80::unsupported_operation(),
    // 10111011 BB:
    |_, _| Z80::unsupported_operation(),
    // 10111100 BC:
    |_, _| Z80::unsupported_operation(),
    // 10111101 BD:
    |_, _| Z80::unsupported_operation(),
    // 10111110 BE:
    |_, _| Z80::unsupported_operation(),
    // 10111111 BF:
    |_, _| Z80::unsupported_operation(),
    // 11000000 C0:
    |_, _| Z80::unsupported_operation(),
    // 11000001 C1:
    |_, _| Z80::unsupported_operation(),
    // 11000010 C2:
    |_, _| Z80::unsupported_operation(),
    // 11000011 C3:
    |_, _| Z80::unsupported_operation(),
    // 11000100 C4:
    |_, _| Z80::unsupported_operation(),
    // 11000101 C5:
    |_, _| Z80::unsupported_operation(),
    // 11000110 C6:
    |_, _| Z80::unsupported_operation(),
    // 11000111 C7:
    |_, _| Z80::unsupported_operation(),
    // 11001000 C8:
    |_, _| Z80::unsupported_operation(),
    // 11001001 C9:
    |_, _| Z80::unsupported_operation(),
    // 11001010 CA:
    |_, _| Z80::unsupported_operation(),
    // 11001011 CB:
    |_, _| Z80::unsupported_operation(),
    // 11001100 CC:
    |_, _| Z80::unsupported_operation(),
    // 11001101 CD:
    |_, _| Z80::unsupported_operation(),
    // 11001110 CE:
    |_, _| Z80::unsupported_operation(),
    // 11001111 CF:
    |_, _| Z80::unsupported_operation(),
    // 11010000 D0:
    |_, _| Z80::unsupported_operation(),
    // 11010001 D1:
    |_, _| Z80::unsupported_operation(),
    // 11010010 D2:
    |_, _| Z80::unsupported_operation(),
    // 11010011 D3:
    |_, _| Z80::unsupported_operation(),
    // 11010100 D4:
    |_, _| Z80::unsupported_operation(),
    // 11010101 D5:
    |_, _| Z80::unsupported_operation(),
    // 11010110 D6:
    |_, _| Z80::unsupported_operation(),
    // 11010111 D7:
    |_, _| Z80::unsupported_operation(),
    // 11011000 D8:
    |_, _| Z80::unsupported_operation(),
    // 11011001 D9:
    |_, _| Z80::unsupported_operation(),
    // 11011010 DA:
    |_, _| Z80::unsupported_operation(),
    // 11011011 DB:
    |_, _| Z80::unsupported_operation(),
    // 11011100 DC:
    |_, _| Z80::unsupported_operation(),
    // 11011101 DD:
    |_, _| Z80::unsupported_operation(),
    // 11011110 DE:
    |_, _| Z80::unsupported_operation(),
    // 11011111 DF:
    |_, _| Z80::unsupported_operation(),
    // 11100000 E0:
    |_, _| Z80::unsupported_operation(),
    // 11100001 E1:
    |_, _| Z80::unsupported_operation(),
    // 11100010 E2:
    |_, _| Z80::unsupported_operation(),
    // 11100011 E3:
    |_, _| Z80::unsupported_operation(),
    // 11100100 E4:
    |_, _| Z80::unsupported_operation(),
    // 11100101 E5:
    |_, _| Z80::unsupported_operation(),
    // 11100110 E6:
    |_, _| Z80::unsupported_operation(),
    // 11100111 E7:
    |_, _| Z80::unsupported_operation(),
    // 11101000 E8:
    |_, _| Z80::unsupported_operation(),
    // 11101001 E9:
    |_, _| Z80::unsupported_operation(),
    // 11101010 EA:
    |_, _| Z80::unsupported_operation(),
    // 11101011 EB:
    |_, _| Z80::unsupported_operation(),
    // 11101100 EC:
    |_, _| Z80::unsupported_operation(),
    // 11101101 ED:
    |_, _| Z80::unsupported_operation(),
    // 11101110 EE:
    |_, _| Z80::unsupported_operation(),
    // 11101111 EF:
    |_, _| Z80::unsupported_operation(),
    // 11110000 F0:
    |_, _| Z80::unsupported_operation(),
    // 11110001 F1:
    |_, _| Z80::unsupported_operation(),
    // 11110010 F2:
    |_, _| Z80::unsupported_operation(),
    // 11110011 F3:
    |_, _| Z80::unsupported_operation(),
    // 11110100 F4:
    |_, _| Z80::unsupported_operation(),
    // 11110101 F5:
    |_, _| Z80::unsupported_operation(),
    // 11110110 F6:
    |_, _| Z80::unsupported_operation(),
    // 11110111 F7:
    |_, _| Z80::unsupported_operation(),
    // 11111000 F8:
    |_, _| Z80::unsupported_operation(),
    // 11111001 F9:
    |_, _| Z80::unsupported_operation(),
    // 11111010 FA:
    |_, _| Z80::unsupported_operation(),
    // 11111011 FB:
    |_, _| Z80::unsupported_operation(),
    // 11111100 FC:
    |_, _| Z80::unsupported_operation(),
    // 11111101 FD:
    |_, _| Z80::unsupported_operation(),
    // 11111110 FE:
    |_, _| Z80::unsupported_operation(),
    // 11111111 FF:
    |_, _| Z80::unsupported_operation()
];

// DD prefix
const IX_FUNCTIONS: [fn(&mut Z80, &mut dyn Z80Memory) -> u8; 256] = [
    // 00000000 00:
    |_, _| Z80::unsupported_operation(),
    // 00000001 01:
    |_, _| Z80::unsupported_operation(),
    // 00000010 02:
    |_, _| Z80::unsupported_operation(),
    // 00000011 03:
    |_, _| Z80::unsupported_operation(),
    // 00000100 04:
    |_, _| Z80::unsupported_operation(),
    // 00000101 05:
    |_, _| Z80::unsupported_operation(),
    // 00000110 06:
    |_, _| Z80::unsupported_operation(),
    // 00000111 07:
    |_, _| Z80::unsupported_operation(),
    // 00001000 08:
    |_, _| Z80::unsupported_operation(),
    // 00001001 09:
    |_, _| Z80::unsupported_operation(),
    // 00001010 0A:
    |_, _| Z80::unsupported_operation(),
    // 00001011 0B:
    |_, _| Z80::unsupported_operation(),
    // 00001100 0C:
    |_, _| Z80::unsupported_operation(),
    // 00001101 0D:
    |_, _| Z80::unsupported_operation(),
    // 00001110 0E:
    |_, _| Z80::unsupported_operation(),
    // 00001111 0F:
    |_, _| Z80::unsupported_operation(),
    // 00010000 10:
    |_, _| Z80::unsupported_operation(),
    // 00010001 11:
    |_, _| Z80::unsupported_operation(),
    // 00010010 12:
    |_, _| Z80::unsupported_operation(),
    // 00010011 13:
    |_, _| Z80::unsupported_operation(),
    // 00010100 14:
    |_, _| Z80::unsupported_operation(),
    // 00010101 15:
    |_, _| Z80::unsupported_operation(),
    // 00010110 16:
    |_, _| Z80::unsupported_operation(),
    // 00010111 17:
    |_, _| Z80::unsupported_operation(),
    // 00011000 18:
    |_, _| Z80::unsupported_operation(),
    // 00011001 19:
    |_, _| Z80::unsupported_operation(),
    // 00011010 1A:
    |_, _| Z80::unsupported_operation(),
    // 00011011 1B:
    |_, _| Z80::unsupported_operation(),
    // 00011100 1C:
    |_, _| Z80::unsupported_operation(),
    // 00011101 1D:
    |_, _| Z80::unsupported_operation(),
    // 00011110 1E:
    |_, _| Z80::unsupported_operation(),
    // 00011111 1F:
    |_, _| Z80::unsupported_operation(),
    // 00100000 20:
    |_, _| Z80::unsupported_operation(),
    // 00100001 21: LD IX, nn
    |z80, mem| z80.ld_ix_nn(mem),
    // 00100010 22: LD (nn), IX
    Z80::ld_mem_nn_ix,
    // 00100011 23:
    |_, _| Z80::unsupported_operation(),
    // 00100100 24:
    |_, _| Z80::unsupported_operation(),
    // 00100101 25:
    |_, _| Z80::unsupported_operation(),
    // 00100110 26:
    |_, _| Z80::unsupported_operation(),
    // 00100111 27:
    |_, _| Z80::unsupported_operation(),
    // 00101000 28:
    |_, _| Z80::unsupported_operation(),
    // 00101001 29:
    |_, _| Z80::unsupported_operation(),
    // 00101010 2A: LD IX, (nn)
    |z80, mem| z80.ld_ix_mem_nn(mem),
    // 00101011 2B:
    |_, _| Z80::unsupported_operation(),
    // 00101100 2C:
    |_, _| Z80::unsupported_operation(),
    // 00101101 2D:
    |_, _| Z80::unsupported_operation(),
    // 00101110 2E:
    |_, _| Z80::unsupported_operation(),
    // 00101111 2F:
    |_, _| Z80::unsupported_operation(),
    // 00110000 30:
    |_, _| Z80::unsupported_operation(),
    // 00110001 31:
    |_, _| Z80::unsupported_operation(),
    // 00110010 32:
    |_, _| Z80::unsupported_operation(),
    // 00110011 33:
    |_, _| Z80::unsupported_operation(),
    // 00110100 34:
    |_, _| Z80::unsupported_operation(),
    // 00110101 35:
    |_, _| Z80::unsupported_operation(),
    // 00110110 36: LD (IX+d), n
    |z80, mem| z80.ld_mem_ixd_n(mem),
    // 00110111 37:
    |_, _| Z80::unsupported_operation(),
    // 00111000 38:
    |_, _| Z80::unsupported_operation(),
    // 00111001 39:
    |_, _| Z80::unsupported_operation(),
    // 00111010 3A:
    |_, _| Z80::unsupported_operation(),
    // 00111011 3B:
    |_, _| Z80::unsupported_operation(),
    // 00111100 3C:
    |_, _| Z80::unsupported_operation(),
    // 00111101 3D:
    |_, _| Z80::unsupported_operation(),
    // 00111110 3E:
    |_, _| Z80::unsupported_operation(),
    // 00111111 3F:
    |_, _| Z80::unsupported_operation(),
    // 01000000 40:
    |_, _| Z80::unsupported_operation(),
    // 01000001 41:
    |_, _| Z80::unsupported_operation(),
    // 01000010 42:
    |_, _| Z80::unsupported_operation(),
    // 01000011 43:
    |_, _| Z80::unsupported_operation(),
    // 01000100 44:
    |_, _| Z80::unsupported_operation(),
    // 01000101 45:
    |_, _| Z80::unsupported_operation(),
    // 01000110 46: LD B, (IX+d)
    |z80, mem| z80.ld_b_mem_ixd(mem),
    // 01000111 47:
    |_, _| Z80::unsupported_operation(),
    // 01001000 48:
    |_, _| Z80::unsupported_operation(),
    // 01001001 49:
    |_, _| Z80::unsupported_operation(),
    // 01001010 4A:
    |_, _| Z80::unsupported_operation(),
    // 01001011 4B:
    |_, _| Z80::unsupported_operation(),
    // 01001100 4C:
    |_, _| Z80::unsupported_operation(),
    // 01001101 4D:
    |_, _| Z80::unsupported_operation(),
    // 01001110 4E: LD C, (IX+d)
    |z80, mem| z80.ld_c_ixd(mem),
    // 01001111 4F:
    |_, _| Z80::unsupported_operation(),
    // 01010000 50:
    |_, _| Z80::unsupported_operation(),
    // 01010001 51:
    |_, _| Z80::unsupported_operation(),
    // 01010010 52:
    |_, _| Z80::unsupported_operation(),
    // 01010011 53:
    |_, _| Z80::unsupported_operation(),
    // 01010100 54:
    |_, _| Z80::unsupported_operation(),
    // 01010101 55:
    |_, _| Z80::unsupported_operation(),
    // 01010110 56: LD D, (IX+d)
    |z80, mem| z80.ld_d_mem_ixd(mem),
    // 01010111 57:
    |_, _| Z80::unsupported_operation(),
    // 01011000 58:
    |_, _| Z80::unsupported_operation(),
    // 01011001 59:
    |_, _| Z80::unsupported_operation(),
    // 01011010 5A:
    |_, _| Z80::unsupported_operation(),
    // 01011011 5B:
    |_, _| Z80::unsupported_operation(),
    // 01011100 5C:
    |_, _| Z80::unsupported_operation(),
    // 01011101 5D:
    |_, _| Z80::unsupported_operation(),
    // 01011110 5E: LD E, (IX+d)
    |z80, mem| z80.ld_e_mem_ixd(mem),
    // 01011111 5F:
    |_, _| Z80::unsupported_operation(),
    // 01100000 60:
    |_, _| Z80::unsupported_operation(),
    // 01100001 61:
    |_, _| Z80::unsupported_operation(),
    // 01100010 62:
    |_, _| Z80::unsupported_operation(),
    // 01100011 63:
    |_, _| Z80::unsupported_operation(),
    // 01100100 64:
    |_, _| Z80::unsupported_operation(),
    // 01100101 65:
    |_, _| Z80::unsupported_operation(),
    // 01100110 66: LD H, (IX+d)
    |z80, mem| z80.ld_h_mem_ixd(mem),
    // 01100111 67:
    |_, _| Z80::unsupported_operation(),
    // 01101000 68:
    |_, _| Z80::unsupported_operation(),
    // 01101001 69:
    |_, _| Z80::unsupported_operation(),
    // 01101010 6A:
    |_, _| Z80::unsupported_operation(),
    // 01101011 6B:
    |_, _| Z80::unsupported_operation(),
    // 01101100 6C:
    |_, _| Z80::unsupported_operation(),
    // 01101101 6D:
    |_, _| Z80::unsupported_operation(),
    // 01101110 6E: LD L, (IX+d)
    |z80, mem| z80.ld_l_mem_ixd(mem),
    // 01101111 6F:
    |_, _| Z80::unsupported_operation(),
    // 01110000 70: LD (IX+d), B
    |z80, mem| z80.ld_mem_ixd_b(mem),
    // 01110001 71: LD (IX+d), C
    |z80, mem| z80.ld_mem_ixd_c(mem),
    // 01110010 72: LD (IX+d), D
    |z80, mem| z80.ld_mem_ixd_d(mem),
    // 01110011 73: LD (IX+d), E
    |z80, mem| z80.ld_mem_ixd_e(mem),
    // 01110100 74: LD (IX+d), H
    |z80, mem| z80.ld_mem_ixd_h(mem),
    // 01110101 75: LD (IX+d), L
    |z80, mem| z80.ld_mem_ixd_l(mem),
    // 01110110 76:
    |_, _| Z80::unsupported_operation(),
    // 01110111 77: LD (IX+d), A
    |z80, mem| z80.ld_mem_ixd_a(mem),
    // 01111000 78:
    |_, _| Z80::unsupported_operation(),
    // 01111001 79:
    |_, _| Z80::unsupported_operation(),
    // 01111010 7A:
    |_, _| Z80::unsupported_operation(),
    // 01111011 7B:
    |_, _| Z80::unsupported_operation(),
    // 01111100 7C:
    |_, _| Z80::unsupported_operation(),
    // 01111101 7D:
    |_, _| Z80::unsupported_operation(),
    // 01111110 7E: LD A, (IX+d)
    |z80, mem| z80.ld_a_mem_ixd(mem),
    // 01111111 7F:
    |_, _| Z80::unsupported_operation(),
    // 10000000 80:
    |_, _| Z80::unsupported_operation(),
    // 10000001 81:
    |_, _| Z80::unsupported_operation(),
    // 10000010 82:
    |_, _| Z80::unsupported_operation(),
    // 10000011 83:
    |_, _| Z80::unsupported_operation(),
    // 10000100 84:
    |_, _| Z80::unsupported_operation(),
    // 10000101 85:
    |_, _| Z80::unsupported_operation(),
    // 10000110 86:
    |_, _| Z80::unsupported_operation(),
    // 10000111 87:
    |_, _| Z80::unsupported_operation(),
    // 10001000 88:
    |_, _| Z80::unsupported_operation(),
    // 10001001 89:
    |_, _| Z80::unsupported_operation(),
    // 10001010 8A:
    |_, _| Z80::unsupported_operation(),
    // 10001011 8B:
    |_, _| Z80::unsupported_operation(),
    // 10001100 8C:
    |_, _| Z80::unsupported_operation(),
    // 10001101 8D:
    |_, _| Z80::unsupported_operation(),
    // 10001110 8E:
    |_, _| Z80::unsupported_operation(),
    // 10001111 8F:
    |_, _| Z80::unsupported_operation(),
    // 10010000 90:
    |_, _| Z80::unsupported_operation(),
    // 10010001 91:
    |_, _| Z80::unsupported_operation(),
    // 10010010 92:
    |_, _| Z80::unsupported_operation(),
    // 10010011 93:
    |_, _| Z80::unsupported_operation(),
    // 10010100 94:
    |_, _| Z80::unsupported_operation(),
    // 10010101 95:
    |_, _| Z80::unsupported_operation(),
    // 10010110 96:
    |_, _| Z80::unsupported_operation(),
    // 10010111 97:
    |_, _| Z80::unsupported_operation(),
    // 10011000 98:
    |_, _| Z80::unsupported_operation(),
    // 10011001 99:
    |_, _| Z80::unsupported_operation(),
    // 10011010 9A:
    |_, _| Z80::unsupported_operation(),
    // 10011011 9B:
    |_, _| Z80::unsupported_operation(),
    // 10011100 9C:
    |_, _| Z80::unsupported_operation(),
    // 10011101 9D:
    |_, _| Z80::unsupported_operation(),
    // 10011110 9E:
    |_, _| Z80::unsupported_operation(),
    // 10011111 9F:
    |_, _| Z80::unsupported_operation(),
    // 10100000 A0:
    |_, _| Z80::unsupported_operation(),
    // 10100001 A1:
    |_, _| Z80::unsupported_operation(),
    // 10100010 A2:
    |_, _| Z80::unsupported_operation(),
    // 10100011 A3:
    |_, _| Z80::unsupported_operation(),
    // 10100100 A4:
    |_, _| Z80::unsupported_operation(),
    // 10100101 A5:
    |_, _| Z80::unsupported_operation(),
    // 10100110 A6:
    |_, _| Z80::unsupported_operation(),
    // 10100111 A7:
    |_, _| Z80::unsupported_operation(),
    // 10101000 A8:
    |_, _| Z80::unsupported_operation(),
    // 10101001 A9:
    |_, _| Z80::unsupported_operation(),
    // 10101010 AA:
    |_, _| Z80::unsupported_operation(),
    // 10101011 AB:
    |_, _| Z80::unsupported_operation(),
    // 10101100 AC:
    |_, _| Z80::unsupported_operation(),
    // 10101101 AD:
    |_, _| Z80::unsupported_operation(),
    // 10101110 AE:
    |_, _| Z80::unsupported_operation(),
    // 10101111 AF:
    |_, _| Z80::unsupported_operation(),
    // 10110000 B0:
    |_, _| Z80::unsupported_operation(),
    // 10110001 B1:
    |_, _| Z80::unsupported_operation(),
    // 10110010 B2:
    |_, _| Z80::unsupported_operation(),
    // 10110011 B3:
    |_, _| Z80::unsupported_operation(),
    // 10110100 B4:
    |_, _| Z80::unsupported_operation(),
    // 10110101 B5:
    |_, _| Z80::unsupported_operation(),
    // 10110110 B6:
    |_, _| Z80::unsupported_operation(),
    // 10110111 B7:
    |_, _| Z80::unsupported_operation(),
    // 10111000 B8:
    |_, _| Z80::unsupported_operation(),
    // 10111001 B9:
    |_, _| Z80::unsupported_operation(),
    // 10111010 BA:
    |_, _| Z80::unsupported_operation(),
    // 10111011 BB:
    |_, _| Z80::unsupported_operation(),
    // 10111100 BC:
    |_, _| Z80::unsupported_operation(),
    // 10111101 BD:
    |_, _| Z80::unsupported_operation(),
    // 10111110 BE:
    |_, _| Z80::unsupported_operation(),
    // 10111111 BF:
    |_, _| Z80::unsupported_operation(),
    // 11000000 C0:
    |_, _| Z80::unsupported_operation(),
    // 11000001 C1:
    |_, _| Z80::unsupported_operation(),
    // 11000010 C2:
    |_, _| Z80::unsupported_operation(),
    // 11000011 C3:
    |_, _| Z80::unsupported_operation(),
    // 11000100 C4:
    |_, _| Z80::unsupported_operation(),
    // 11000101 C5:
    |_, _| Z80::unsupported_operation(),
    // 11000110 C6:
    |_, _| Z80::unsupported_operation(),
    // 11000111 C7:
    |_, _| Z80::unsupported_operation(),
    // 11001000 C8:
    |_, _| Z80::unsupported_operation(),
    // 11001001 C9:
    |_, _| Z80::unsupported_operation(),
    // 11001010 CA:
    |_, _| Z80::unsupported_operation(),
    // 11001011 CB:
    |_, _| Z80::unsupported_operation(),
    // 11001100 CC:
    |_, _| Z80::unsupported_operation(),
    // 11001101 CD:
    |_, _| Z80::unsupported_operation(),
    // 11001110 CE:
    |_, _| Z80::unsupported_operation(),
    // 11001111 CF:
    |_, _| Z80::unsupported_operation(),
    // 11010000 D0:
    |_, _| Z80::unsupported_operation(),
    // 11010001 D1:
    |_, _| Z80::unsupported_operation(),
    // 11010010 D2:
    |_, _| Z80::unsupported_operation(),
    // 11010011 D3:
    |_, _| Z80::unsupported_operation(),
    // 11010100 D4:
    |_, _| Z80::unsupported_operation(),
    // 11010101 D5:
    |_, _| Z80::unsupported_operation(),
    // 11010110 D6:
    |_, _| Z80::unsupported_operation(),
    // 11010111 D7:
    |_, _| Z80::unsupported_operation(),
    // 11011000 D8:
    |_, _| Z80::unsupported_operation(),
    // 11011001 D9:
    |_, _| Z80::unsupported_operation(),
    // 11011010 DA:
    |_, _| Z80::unsupported_operation(),
    // 11011011 DB:
    |_, _| Z80::unsupported_operation(),
    // 11011100 DC:
    |_, _| Z80::unsupported_operation(),
    // 11011101 DD:
    |_, _| Z80::unsupported_operation(),
    // 11011110 DE:
    |_, _| Z80::unsupported_operation(),
    // 11011111 DF:
    |_, _| Z80::unsupported_operation(),
    // 11100000 E0:
    |_, _| Z80::unsupported_operation(),
    // 11100001 E1: POP IX
    |z80, mem| z80.pop_ix(mem),
    // 11100010 E2:
    |_, _| Z80::unsupported_operation(),
    // 11100011 E3: EX (SP), IX
    Z80::ex_mem_sp_ix,
    // 11100100 E4:
    |_, _| Z80::unsupported_operation(),
    // 11100101 E5: PUSH IX
    Z80::push_ix,
    // 11100110 E6:
    |_, _| Z80::unsupported_operation(),
    // 11100111 E7:
    |_, _| Z80::unsupported_operation(),
    // 11101000 E8:
    |_, _| Z80::unsupported_operation(),
    // 11101001 E9:
    |_, _| Z80::unsupported_operation(),
    // 11101010 EA:
    |_, _| Z80::unsupported_operation(),
    // 11101011 EB:
    |_, _| Z80::unsupported_operation(),
    // 11101100 EC:
    |_, _| Z80::unsupported_operation(),
    // 11101101 ED:
    |_, _| Z80::unsupported_operation(),
    // 11101110 EE:
    |_, _| Z80::unsupported_operation(),
    // 11101111 EF:
    |_, _| Z80::unsupported_operation(),
    // 11110000 F0:
    |_, _| Z80::unsupported_operation(),
    // 11110001 F1:
    |_, _| Z80::unsupported_operation(),
    // 11110010 F2:
    |_, _| Z80::unsupported_operation(),
    // 11110011 F3:
    |_, _| Z80::unsupported_operation(),
    // 11110100 F4:
    |_, _| Z80::unsupported_operation(),
    // 11110101 F5:
    |_, _| Z80::unsupported_operation(),
    // 11110110 F6:
    |_, _| Z80::unsupported_operation(),
    // 11110111 F7:
    |_, _| Z80::unsupported_operation(),
    // 11111000 F8:
    |_, _| Z80::unsupported_operation(),
    // 11111001 F9: LD SP, IX
    |z80, _| z80.ld_sp_ix(),
    // 11111010 FA:
    |_, _| Z80::unsupported_operation(),
    // 11111011 FB:
    |_, _| Z80::unsupported_operation(),
    // 11111100 FC:
    |_, _| Z80::unsupported_operation(),
    // 11111101 FD:
    |_, _| Z80::unsupported_operation(),
    // 11111110 FE:
    |_, _| Z80::unsupported_operation(),
    // 11111111 FF:
    |_, _| Z80::unsupported_operation()
];

// DDCB prefix
const IX_BIT_INSTRUCTIONS: [fn(&mut Z80, &mut dyn Z80Memory) -> u8; 256] = [
    // 00000000 00:
    |_, _| Z80::unsupported_operation(),
    // 00000001 01:
    |_, _| Z80::unsupported_operation(),
    // 00000010 02:
    |_, _| Z80::unsupported_operation(),
    // 00000011 03:
    |_, _| Z80::unsupported_operation(),
    // 00000100 04:
    |_, _| Z80::unsupported_operation(),
    // 00000101 05:
    |_, _| Z80::unsupported_operation(),
    // 00000110 06:
    |_, _| Z80::unsupported_operation(),
    // 00000111 07:
    |_, _| Z80::unsupported_operation(),
    // 00001000 08:
    |_, _| Z80::unsupported_operation(),
    // 00001001 09:
    |_, _| Z80::unsupported_operation(),
    // 00001010 0A:
    |_, _| Z80::unsupported_operation(),
    // 00001011 0B:
    |_, _| Z80::unsupported_operation(),
    // 00001100 0C:
    |_, _| Z80::unsupported_operation(),
    // 00001101 0D:
    |_, _| Z80::unsupported_operation(),
    // 00001110 0E:
    |_, _| Z80::unsupported_operation(),
    // 00001111 0F:
    |_, _| Z80::unsupported_operation(),
    // 00010000 10:
    |_, _| Z80::unsupported_operation(),
    // 00010001 11:
    |_, _| Z80::unsupported_operation(),
    // 00010010 12:
    |_, _| Z80::unsupported_operation(),
    // 00010011 13:
    |_, _| Z80::unsupported_operation(),
    // 00010100 14:
    |_, _| Z80::unsupported_operation(),
    // 00010101 15:
    |_, _| Z80::unsupported_operation(),
    // 00010110 16:
    |_, _| Z80::unsupported_operation(),
    // 00010111 17:
    |_, _| Z80::unsupported_operation(),
    // 00011000 18:
    |_, _| Z80::unsupported_operation(),
    // 00011001 19:
    |_, _| Z80::unsupported_operation(),
    // 00011010 1A:
    |_, _| Z80::unsupported_operation(),
    // 00011011 1B:
    |_, _| Z80::unsupported_operation(),
    // 00011100 1C:
    |_, _| Z80::unsupported_operation(),
    // 00011101 1D:
    |_, _| Z80::unsupported_operation(),
    // 00011110 1E:
    |_, _| Z80::unsupported_operation(),
    // 00011111 1F:
    |_, _| Z80::unsupported_operation(),
    // 00100000 20:
    |_, _| Z80::unsupported_operation(),
    // 00100001 21:
    |_, _| Z80::unsupported_operation(),
    // 00100010 22:
    |_, _| Z80::unsupported_operation(),
    // 00100011 23:
    |_, _| Z80::unsupported_operation(),
    // 00100100 24:
    |_, _| Z80::unsupported_operation(),
    // 00100101 25:
    |_, _| Z80::unsupported_operation(),
    // 00100110 26:
    |_, _| Z80::unsupported_operation(),
    // 00100111 27:
    |_, _| Z80::unsupported_operation(),
    // 00101000 28:
    |_, _| Z80::unsupported_operation(),
    // 00101001 29:
    |_, _| Z80::unsupported_operation(),
    // 00101010 2A:
    |_, _| Z80::unsupported_operation(),
    // 00101011 2B:
    |_, _| Z80::unsupported_operation(),
    // 00101100 2C:
    |_, _| Z80::unsupported_operation(),
    // 00101101 2D:
    |_, _| Z80::unsupported_operation(),
    // 00101110 2E:
    |_, _| Z80::unsupported_operation(),
    // 00101111 2F:
    |_, _| Z80::unsupported_operation(),
    // 00110000 30:
    |_, _| Z80::unsupported_operation(),
    // 00110001 31:
    |_, _| Z80::unsupported_operation(),
    // 00110010 32:
    |_, _| Z80::unsupported_operation(),
    // 00110011 33:
    |_, _| Z80::unsupported_operation(),
    // 00110100 34:
    |_, _| Z80::unsupported_operation(),
    // 00110101 35:
    |_, _| Z80::unsupported_operation(),
    // 00110110 36:
    |_, _| Z80::unsupported_operation(),
    // 00110111 37:
    |_, _| Z80::unsupported_operation(),
    // 00111000 38:
    |_, _| Z80::unsupported_operation(),
    // 00111001 39:
    |_, _| Z80::unsupported_operation(),
    // 00111010 3A:
    |_, _| Z80::unsupported_operation(),
    // 00111011 3B:
    |_, _| Z80::unsupported_operation(),
    // 00111100 3C:
    |_, _| Z80::unsupported_operation(),
    // 00111101 3D:
    |_, _| Z80::unsupported_operation(),
    // 00111110 3E:
    |_, _| Z80::unsupported_operation(),
    // 00111111 3F:
    |_, _| Z80::unsupported_operation(),
    // 01000000 40:
    |_, _| Z80::unsupported_operation(),
    // 01000001 41:
    |_, _| Z80::unsupported_operation(),
    // 01000010 42:
    |_, _| Z80::unsupported_operation(),
    // 01000011 43:
    |_, _| Z80::unsupported_operation(),
    // 01000100 44:
    |_, _| Z80::unsupported_operation(),
    // 01000101 45:
    |_, _| Z80::unsupported_operation(),
    // 01000110 46:
    |_, _| Z80::unsupported_operation(),
    // 01000111 47:
    |_, _| Z80::unsupported_operation(),
    // 01001000 48:
    |_, _| Z80::unsupported_operation(),
    // 01001001 49:
    |_, _| Z80::unsupported_operation(),
    // 01001010 4A:
    |_, _| Z80::unsupported_operation(),
    // 01001011 4B:
    |_, _| Z80::unsupported_operation(),
    // 01001100 4C:
    |_, _| Z80::unsupported_operation(),
    // 01001101 4D:
    |_, _| Z80::unsupported_operation(),
    // 01001110 4E:
    |_, _| Z80::unsupported_operation(),
    // 01001111 4F:
    |_, _| Z80::unsupported_operation(),
    // 01010000 50:
    |_, _| Z80::unsupported_operation(),
    // 01010001 51:
    |_, _| Z80::unsupported_operation(),
    // 01010010 52:
    |_, _| Z80::unsupported_operation(),
    // 01010011 53:
    |_, _| Z80::unsupported_operation(),
    // 01010100 54:
    |_, _| Z80::unsupported_operation(),
    // 01010101 55:
    |_, _| Z80::unsupported_operation(),
    // 01010110 56:
    |_, _| Z80::unsupported_operation(),
    // 01010111 57:
    |_, _| Z80::unsupported_operation(),
    // 01011000 58:
    |_, _| Z80::unsupported_operation(),
    // 01011001 59:
    |_, _| Z80::unsupported_operation(),
    // 01011010 5A:
    |_, _| Z80::unsupported_operation(),
    // 01011011 5B:
    |_, _| Z80::unsupported_operation(),
    // 01011100 5C:
    |_, _| Z80::unsupported_operation(),
    // 01011101 5D:
    |_, _| Z80::unsupported_operation(),
    // 01011110 5E:
    |_, _| Z80::unsupported_operation(),
    // 01011111 5F:
    |_, _| Z80::unsupported_operation(),
    // 01100000 60:
    |_, _| Z80::unsupported_operation(),
    // 01100001 61:
    |_, _| Z80::unsupported_operation(),
    // 01100010 62:
    |_, _| Z80::unsupported_operation(),
    // 01100011 63:
    |_, _| Z80::unsupported_operation(),
    // 01100100 64:
    |_, _| Z80::unsupported_operation(),
    // 01100101 65:
    |_, _| Z80::unsupported_operation(),
    // 01100110 66:
    |_, _| Z80::unsupported_operation(),
    // 01100111 67:
    |_, _| Z80::unsupported_operation(),
    // 01101000 68:
    |_, _| Z80::unsupported_operation(),
    // 01101001 69:
    |_, _| Z80::unsupported_operation(),
    // 01101010 6A:
    |_, _| Z80::unsupported_operation(),
    // 01101011 6B:
    |_, _| Z80::unsupported_operation(),
    // 01101100 6C:
    |_, _| Z80::unsupported_operation(),
    // 01101101 6D:
    |_, _| Z80::unsupported_operation(),
    // 01101110 6E:
    |_, _| Z80::unsupported_operation(),
    // 01101111 6F:
    |_, _| Z80::unsupported_operation(),
    // 01110000 70:
    |_, _| Z80::unsupported_operation(),
    // 01110001 71:
    |_, _| Z80::unsupported_operation(),
    // 01110010 72:
    |_, _| Z80::unsupported_operation(),
    // 01110011 73:
    |_, _| Z80::unsupported_operation(),
    // 01110100 74:
    |_, _| Z80::unsupported_operation(),
    // 01110101 75:
    |_, _| Z80::unsupported_operation(),
    // 01110110 76:
    |_, _| Z80::unsupported_operation(),
    // 01110111 77:
    |_, _| Z80::unsupported_operation(),
    // 01111000 78:
    |_, _| Z80::unsupported_operation(),
    // 01111001 79:
    |_, _| Z80::unsupported_operation(),
    // 01111010 7A:
    |_, _| Z80::unsupported_operation(),
    // 01111011 7B:
    |_, _| Z80::unsupported_operation(),
    // 01111100 7C:
    |_, _| Z80::unsupported_operation(),
    // 01111101 7D:
    |_, _| Z80::unsupported_operation(),
    // 01111110 7E:
    |_, _| Z80::unsupported_operation(),
    // 01111111 7F:
    |_, _| Z80::unsupported_operation(),
    // 10000000 80:
    |_, _| Z80::unsupported_operation(),
    // 10000001 81:
    |_, _| Z80::unsupported_operation(),
    // 10000010 82:
    |_, _| Z80::unsupported_operation(),
    // 10000011 83:
    |_, _| Z80::unsupported_operation(),
    // 10000100 84:
    |_, _| Z80::unsupported_operation(),
    // 10000101 85:
    |_, _| Z80::unsupported_operation(),
    // 10000110 86:
    |_, _| Z80::unsupported_operation(),
    // 10000111 87:
    |_, _| Z80::unsupported_operation(),
    // 10001000 88:
    |_, _| Z80::unsupported_operation(),
    // 10001001 89:
    |_, _| Z80::unsupported_operation(),
    // 10001010 8A:
    |_, _| Z80::unsupported_operation(),
    // 10001011 8B:
    |_, _| Z80::unsupported_operation(),
    // 10001100 8C:
    |_, _| Z80::unsupported_operation(),
    // 10001101 8D:
    |_, _| Z80::unsupported_operation(),
    // 10001110 8E:
    |_, _| Z80::unsupported_operation(),
    // 10001111 8F:
    |_, _| Z80::unsupported_operation(),
    // 10010000 90:
    |_, _| Z80::unsupported_operation(),
    // 10010001 91:
    |_, _| Z80::unsupported_operation(),
    // 10010010 92:
    |_, _| Z80::unsupported_operation(),
    // 10010011 93:
    |_, _| Z80::unsupported_operation(),
    // 10010100 94:
    |_, _| Z80::unsupported_operation(),
    // 10010101 95:
    |_, _| Z80::unsupported_operation(),
    // 10010110 96:
    |_, _| Z80::unsupported_operation(),
    // 10010111 97:
    |_, _| Z80::unsupported_operation(),
    // 10011000 98:
    |_, _| Z80::unsupported_operation(),
    // 10011001 99:
    |_, _| Z80::unsupported_operation(),
    // 10011010 9A:
    |_, _| Z80::unsupported_operation(),
    // 10011011 9B:
    |_, _| Z80::unsupported_operation(),
    // 10011100 9C:
    |_, _| Z80::unsupported_operation(),
    // 10011101 9D:
    |_, _| Z80::unsupported_operation(),
    // 10011110 9E:
    |_, _| Z80::unsupported_operation(),
    // 10011111 9F:
    |_, _| Z80::unsupported_operation(),
    // 10100000 A0:
    |_, _| Z80::unsupported_operation(),
    // 10100001 A1:
    |_, _| Z80::unsupported_operation(),
    // 10100010 A2:
    |_, _| Z80::unsupported_operation(),
    // 10100011 A3:
    |_, _| Z80::unsupported_operation(),
    // 10100100 A4:
    |_, _| Z80::unsupported_operation(),
    // 10100101 A5:
    |_, _| Z80::unsupported_operation(),
    // 10100110 A6:
    |_, _| Z80::unsupported_operation(),
    // 10100111 A7:
    |_, _| Z80::unsupported_operation(),
    // 10101000 A8:
    |_, _| Z80::unsupported_operation(),
    // 10101001 A9:
    |_, _| Z80::unsupported_operation(),
    // 10101010 AA:
    |_, _| Z80::unsupported_operation(),
    // 10101011 AB:
    |_, _| Z80::unsupported_operation(),
    // 10101100 AC:
    |_, _| Z80::unsupported_operation(),
    // 10101101 AD:
    |_, _| Z80::unsupported_operation(),
    // 10101110 AE:
    |_, _| Z80::unsupported_operation(),
    // 10101111 AF:
    |_, _| Z80::unsupported_operation(),
    // 10110000 B0:
    |_, _| Z80::unsupported_operation(),
    // 10110001 B1:
    |_, _| Z80::unsupported_operation(),
    // 10110010 B2:
    |_, _| Z80::unsupported_operation(),
    // 10110011 B3:
    |_, _| Z80::unsupported_operation(),
    // 10110100 B4:
    |_, _| Z80::unsupported_operation(),
    // 10110101 B5:
    |_, _| Z80::unsupported_operation(),
    // 10110110 B6:
    |_, _| Z80::unsupported_operation(),
    // 10110111 B7:
    |_, _| Z80::unsupported_operation(),
    // 10111000 B8:
    |_, _| Z80::unsupported_operation(),
    // 10111001 B9:
    |_, _| Z80::unsupported_operation(),
    // 10111010 BA:
    |_, _| Z80::unsupported_operation(),
    // 10111011 BB:
    |_, _| Z80::unsupported_operation(),
    // 10111100 BC:
    |_, _| Z80::unsupported_operation(),
    // 10111101 BD:
    |_, _| Z80::unsupported_operation(),
    // 10111110 BE:
    |_, _| Z80::unsupported_operation(),
    // 10111111 BF:
    |_, _| Z80::unsupported_operation(),
    // 11000000 C0:
    |_, _| Z80::unsupported_operation(),
    // 11000001 C1:
    |_, _| Z80::unsupported_operation(),
    // 11000010 C2:
    |_, _| Z80::unsupported_operation(),
    // 11000011 C3:
    |_, _| Z80::unsupported_operation(),
    // 11000100 C4:
    |_, _| Z80::unsupported_operation(),
    // 11000101 C5:
    |_, _| Z80::unsupported_operation(),
    // 11000110 C6:
    |_, _| Z80::unsupported_operation(),
    // 11000111 C7:
    |_, _| Z80::unsupported_operation(),
    // 11001000 C8:
    |_, _| Z80::unsupported_operation(),
    // 11001001 C9:
    |_, _| Z80::unsupported_operation(),
    // 11001010 CA:
    |_, _| Z80::unsupported_operation(),
    // 11001011 CB:
    |_, _| Z80::unsupported_operation(),
    // 11001100 CC:
    |_, _| Z80::unsupported_operation(),
    // 11001101 CD:
    |_, _| Z80::unsupported_operation(),
    // 11001110 CE:
    |_, _| Z80::unsupported_operation(),
    // 11001111 CF:
    |_, _| Z80::unsupported_operation(),
    // 11010000 D0:
    |_, _| Z80::unsupported_operation(),
    // 11010001 D1:
    |_, _| Z80::unsupported_operation(),
    // 11010010 D2:
    |_, _| Z80::unsupported_operation(),
    // 11010011 D3:
    |_, _| Z80::unsupported_operation(),
    // 11010100 D4:
    |_, _| Z80::unsupported_operation(),
    // 11010101 D5:
    |_, _| Z80::unsupported_operation(),
    // 11010110 D6:
    |_, _| Z80::unsupported_operation(),
    // 11010111 D7:
    |_, _| Z80::unsupported_operation(),
    // 11011000 D8:
    |_, _| Z80::unsupported_operation(),
    // 11011001 D9:
    |_, _| Z80::unsupported_operation(),
    // 11011010 DA:
    |_, _| Z80::unsupported_operation(),
    // 11011011 DB:
    |_, _| Z80::unsupported_operation(),
    // 11011100 DC:
    |_, _| Z80::unsupported_operation(),
    // 11011101 DD:
    |_, _| Z80::unsupported_operation(),
    // 11011110 DE:
    |_, _| Z80::unsupported_operation(),
    // 11011111 DF:
    |_, _| Z80::unsupported_operation(),
    // 11100000 E0:
    |_, _| Z80::unsupported_operation(),
    // 11100001 E1:
    |_, _| Z80::unsupported_operation(),
    // 11100010 E2:
    |_, _| Z80::unsupported_operation(),
    // 11100011 E3:
    |_, _| Z80::unsupported_operation(),
    // 11100100 E4:
    |_, _| Z80::unsupported_operation(),
    // 11100101 E5:
    |_, _| Z80::unsupported_operation(),
    // 11100110 E6:
    |_, _| Z80::unsupported_operation(),
    // 11100111 E7:
    |_, _| Z80::unsupported_operation(),
    // 11101000 E8:
    |_, _| Z80::unsupported_operation(),
    // 11101001 E9:
    |_, _| Z80::unsupported_operation(),
    // 11101010 EA:
    |_, _| Z80::unsupported_operation(),
    // 11101011 EB:
    |_, _| Z80::unsupported_operation(),
    // 11101100 EC:
    |_, _| Z80::unsupported_operation(),
    // 11101101 ED:
    |_, _| Z80::unsupported_operation(),
    // 11101110 EE:
    |_, _| Z80::unsupported_operation(),
    // 11101111 EF:
    |_, _| Z80::unsupported_operation(),
    // 11110000 F0:
    |_, _| Z80::unsupported_operation(),
    // 11110001 F1:
    |_, _| Z80::unsupported_operation(),
    // 11110010 F2:
    |_, _| Z80::unsupported_operation(),
    // 11110011 F3:
    |_, _| Z80::unsupported_operation(),
    // 11110100 F4:
    |_, _| Z80::unsupported_operation(),
    // 11110101 F5:
    |_, _| Z80::unsupported_operation(),
    // 11110110 F6:
    |_, _| Z80::unsupported_operation(),
    // 11110111 F7:
    |_, _| Z80::unsupported_operation(),
    // 11111000 F8:
    |_, _| Z80::unsupported_operation(),
    // 11111001 F9:
    |_, _| Z80::unsupported_operation(),
    // 11111010 FA:
    |_, _| Z80::unsupported_operation(),
    // 11111011 FB:
    |_, _| Z80::unsupported_operation(),
    // 11111100 FC:
    |_, _| Z80::unsupported_operation(),
    // 11111101 FD:
    |_, _| Z80::unsupported_operation(),
    // 11111110 FE:
    |_, _| Z80::unsupported_operation(),
    // 11111111 FF:
    |_, _| Z80::unsupported_operation(),
    ];

// ED prefix
const EXTENDED_INSTRUCTIONS: [fn(&mut Z80, &mut dyn Z80Memory) -> u8; 256] = [
    // 00000000 00:
    |_, _| Z80::unsupported_operation(),
    // 00000001 01:
    |_, _| Z80::unsupported_operation(),
    // 00000010 02:
    |_, _| Z80::unsupported_operation(),
    // 00000011 03:
    |_, _| Z80::unsupported_operation(),
    // 00000100 04:
    |_, _| Z80::unsupported_operation(),
    // 00000101 05:
    |_, _| Z80::unsupported_operation(),
    // 00000110 06:
    |_, _| Z80::unsupported_operation(),
    // 00000111 07:
    |_, _| Z80::unsupported_operation(),
    // 00001000 08:
    |_, _| Z80::unsupported_operation(),
    // 00001001 09:
    |_, _| Z80::unsupported_operation(),
    // 00001010 0A:
    |_, _| Z80::unsupported_operation(),
    // 00001011 0B:
    |_, _| Z80::unsupported_operation(),
    // 00001100 0C:
    |_, _| Z80::unsupported_operation(),
    // 00001101 0D:
    |_, _| Z80::unsupported_operation(),
    // 00001110 0E:
    |_, _| Z80::unsupported_operation(),
    // 00001111 0F:
    |_, _| Z80::unsupported_operation(),
    // 00010000 10:
    |_, _| Z80::unsupported_operation(),
    // 00010001 11:
    |_, _| Z80::unsupported_operation(),
    // 00010010 12:
    |_, _| Z80::unsupported_operation(),
    // 00010011 13:
    |_, _| Z80::unsupported_operation(),
    // 00010100 14:
    |_, _| Z80::unsupported_operation(),
    // 00010101 15:
    |_, _| Z80::unsupported_operation(),
    // 00010110 16:
    |_, _| Z80::unsupported_operation(),
    // 00010111 17:
    |_, _| Z80::unsupported_operation(),
    // 00011000 18:
    |_, _| Z80::unsupported_operation(),
    // 00011001 19:
    |_, _| Z80::unsupported_operation(),
    // 00011010 1A:
    |_, _| Z80::unsupported_operation(),
    // 00011011 1B:
    |_, _| Z80::unsupported_operation(),
    // 00011100 1C:
    |_, _| Z80::unsupported_operation(),
    // 00011101 1D:
    |_, _| Z80::unsupported_operation(),
    // 00011110 1E:
    |_, _| Z80::unsupported_operation(),
    // 00011111 1F:
    |_, _| Z80::unsupported_operation(),
    // 00100000 20:
    |_, _| Z80::unsupported_operation(),
    // 00100001 21:
    |_, _| Z80::unsupported_operation(),
    // 00100010 22:
    |_, _| Z80::unsupported_operation(),
    // 00100011 23:
    |_, _| Z80::unsupported_operation(),
    // 00100100 24:
    |_, _| Z80::unsupported_operation(),
    // 00100101 25:
    |_, _| Z80::unsupported_operation(),
    // 00100110 26:
    |_, _| Z80::unsupported_operation(),
    // 00100111 27:
    |_, _| Z80::unsupported_operation(),
    // 00101000 28:
    |_, _| Z80::unsupported_operation(),
    // 00101001 29:
    |_, _| Z80::unsupported_operation(),
    // 00101010 2A:
    |_, _| Z80::unsupported_operation(),
    // 00101011 2B:
    |_, _| Z80::unsupported_operation(),
    // 00101100 2C:
    |_, _| Z80::unsupported_operation(),
    // 00101101 2D:
    |_, _| Z80::unsupported_operation(),
    // 00101110 2E:
    |_, _| Z80::unsupported_operation(),
    // 00101111 2F:
    |_, _| Z80::unsupported_operation(),
    // 00110000 30:
    |_, _| Z80::unsupported_operation(),
    // 00110001 31:
    |_, _| Z80::unsupported_operation(),
    // 00110010 32:
    |_, _| Z80::unsupported_operation(),
    // 00110011 33:
    |_, _| Z80::unsupported_operation(),
    // 00110100 34:
    |_, _| Z80::unsupported_operation(),
    // 00110101 35:
    |_, _| Z80::unsupported_operation(),
    // 00110110 36:
    |_, _| Z80::unsupported_operation(),
    // 00110111 37:
    |_, _| Z80::unsupported_operation(),
    // 00111000 38:
    |_, _| Z80::unsupported_operation(),
    // 00111001 39:
    |_, _| Z80::unsupported_operation(),
    // 00111010 3A:
    |_, _| Z80::unsupported_operation(),
    // 00111011 3B:
    |_, _| Z80::unsupported_operation(),
    // 00111100 3C:
    |_, _| Z80::unsupported_operation(),
    // 00111101 3D:
    |_, _| Z80::unsupported_operation(),
    // 00111110 3E:
    |_, _| Z80::unsupported_operation(),
    // 00111111 3F:
    |_, _| Z80::unsupported_operation(),
    // 01000000 40:
    |_, _| Z80::unsupported_operation(),
    // 01000001 41:
    |_, _| Z80::unsupported_operation(),
    // 01000010 42:
    |_, _| Z80::unsupported_operation(),
    // 01000011 43: LD (nn), BC
    Z80::ld_mem_nn_ddbc,
    // 01000100 44: NEG
    |z80, _| z80.neg(),
    // 01000101 45:
    |_, _| Z80::unsupported_operation(),
    // 01000110 46:
    |_, _| Z80::unsupported_operation(),
    // 01000111 47: LD I, A
    |z80, _| z80.ld_i_a(),
    // 01001000 48:
    |_, _| Z80::unsupported_operation(),
    // 01001001 49:
    |_, _| Z80::unsupported_operation(),
    // 01001010 4A:
    |_, _| Z80::unsupported_operation(),
    // 01001011 4B: LD BC, (nn)
    |z80, mem| z80.ld_ddbc_mem_nn(mem),
    // 01001100 4C:
    |_, _| Z80::unsupported_operation(),
    // 01001101 4D:
    |_, _| Z80::unsupported_operation(),
    // 01001110 4E:
    |_, _| Z80::unsupported_operation(),
    // 01001111 4F: LD R, A
    |z80, _| z80.ld_r_a(),
    // 01010000 50:
    |_, _| Z80::unsupported_operation(),
    // 01010001 51:
    |_, _| Z80::unsupported_operation(),
    // 01010010 52:
    |_, _| Z80::unsupported_operation(),
    // 01010011 53: LD (nn), DE
    Z80::ld_mem_nn_ddde,
    // 01010100 54:
    |_, _| Z80::unsupported_operation(),
    // 01010101 55:
    |_, _| Z80::unsupported_operation(),
    // 01010110 56:
    |_, _| Z80::unsupported_operation(),
    // 01010111 57: LD A, I
    |z80, _| z80.ld_a_i(),
    // 01011000 58:
    |_, _| Z80::unsupported_operation(),
    // 01011001 59:
    |_, _| Z80::unsupported_operation(),
    // 01011010 5A:
    |_, _| Z80::unsupported_operation(),
    // 01011011 5B: LD DE, (nn)
    |z80, mem| z80.ld_ddde_mem_nn(mem),
    // 01011100 5C:
    |_, _| Z80::unsupported_operation(),
    // 01011101 5D:
    |_, _| Z80::unsupported_operation(),
    // 01011110 5E:
    |_, _| Z80::unsupported_operation(),
    // 01011111 5F: LD A, R
    |z80, _| z80.ld_a_r(),
    // 01100000 60:
    |_, _| Z80::unsupported_operation(),
    // 01100001 61:
    |_, _| Z80::unsupported_operation(),
    // 01100010 62:
    |_, _| Z80::unsupported_operation(),
    // 01100011 63: LD (nn), HL
    Z80::ld_mem_nn_ddhl,
    // 01100100 64:
    |_, _| Z80::unsupported_operation(),
    // 01100101 65:
    |_, _| Z80::unsupported_operation(),
    // 01100110 66:
    |_, _| Z80::unsupported_operation(),
    // 01100111 67:
    |_, _| Z80::unsupported_operation(),
    // 01101000 68:
    |_, _| Z80::unsupported_operation(),
    // 01101001 69:
    |_, _| Z80::unsupported_operation(),
    // 01101010 6A:
    |_, _| Z80::unsupported_operation(),
    // 01101011 6B: LD HL, (nn)
    |z80, mem| z80.ld_ddhl_mem_nn(mem),
    // 01101100 6C:
    |_, _| Z80::unsupported_operation(),
    // 01101101 6D:
    |_, _| Z80::unsupported_operation(),
    // 01101110 6E:
    |_, _| Z80::unsupported_operation(),
    // 01101111 6F:
    |_, _| Z80::unsupported_operation(),
    // 01110000 70:
    |_, _| Z80::unsupported_operation(),
    // 01110001 71:
    |_, _| Z80::unsupported_operation(),
    // 01110010 72:
    |_, _| Z80::unsupported_operation(),
    // 01110011 73: LD (nn), SP
    Z80::ld_mem_nn_ddsp,
    // 01110100 74:
    |_, _| Z80::unsupported_operation(),
    // 01110101 75:
    |_, _| Z80::unsupported_operation(),
    // 01110110 76:
    |_, _| Z80::unsupported_operation(),
    // 01110111 77:
    |_, _| Z80::unsupported_operation(),
    // 01111000 78:
    |_, _| Z80::unsupported_operation(),
    // 01111001 79:
    |_, _| Z80::unsupported_operation(),
    // 01111010 7A:
    |_, _| Z80::unsupported_operation(),
    // 01111011 7B: LD SP, (nn)
    |z80, mem| z80.ld_ddsp_mem_nn(mem),
    // 01111100 7C:
    |_, _| Z80::unsupported_operation(),
    // 01111101 7D:
    |_, _| Z80::unsupported_operation(),
    // 01111110 7E:
    |_, _| Z80::unsupported_operation(),
    // 01111111 7F:
    |_, _| Z80::unsupported_operation(),
    // 10000000 80:
    |_, _| Z80::unsupported_operation(),
    // 10000001 81:
    |_, _| Z80::unsupported_operation(),
    // 10000010 82:
    |_, _| Z80::unsupported_operation(),
    // 10000011 83:
    |_, _| Z80::unsupported_operation(),
    // 10000100 84:
    |_, _| Z80::unsupported_operation(),
    // 10000101 85:
    |_, _| Z80::unsupported_operation(),
    // 10000110 86:
    |_, _| Z80::unsupported_operation(),
    // 10000111 87:
    |_, _| Z80::unsupported_operation(),
    // 10001000 88:
    |_, _| Z80::unsupported_operation(),
    // 10001001 89:
    |_, _| Z80::unsupported_operation(),
    // 10001010 8A:
    |_, _| Z80::unsupported_operation(),
    // 10001011 8B:
    |_, _| Z80::unsupported_operation(),
    // 10001100 8C:
    |_, _| Z80::unsupported_operation(),
    // 10001101 8D:
    |_, _| Z80::unsupported_operation(),
    // 10001110 8E:
    |_, _| Z80::unsupported_operation(),
    // 10001111 8F:
    |_, _| Z80::unsupported_operation(),
    // 10010000 90:
    |_, _| Z80::unsupported_operation(),
    // 10010001 91:
    |_, _| Z80::unsupported_operation(),
    // 10010010 92:
    |_, _| Z80::unsupported_operation(),
    // 10010011 93:
    |_, _| Z80::unsupported_operation(),
    // 10010100 94:
    |_, _| Z80::unsupported_operation(),
    // 10010101 95:
    |_, _| Z80::unsupported_operation(),
    // 10010110 96:
    |_, _| Z80::unsupported_operation(),
    // 10010111 97:
    |_, _| Z80::unsupported_operation(),
    // 10011000 98:
    |_, _| Z80::unsupported_operation(),
    // 10011001 99:
    |_, _| Z80::unsupported_operation(),
    // 10011010 9A:
    |_, _| Z80::unsupported_operation(),
    // 10011011 9B:
    |_, _| Z80::unsupported_operation(),
    // 10011100 9C:
    |_, _| Z80::unsupported_operation(),
    // 10011101 9D:
    |_, _| Z80::unsupported_operation(),
    // 10011110 9E:
    |_, _| Z80::unsupported_operation(),
    // 10011111 9F:
    |_, _| Z80::unsupported_operation(),
    // 10100000 A0: LDI
    Z80::ldi,
    // 10100001 A1: CPI
    |z80, mem| z80.cpi(mem),
    // 10100010 A2:
    |_, _| Z80::unsupported_operation(),
    // 10100011 A3:
    |_, _| Z80::unsupported_operation(),
    // 10100100 A4:
    |_, _| Z80::unsupported_operation(),
    // 10100101 A5:
    |_, _| Z80::unsupported_operation(),
    // 10100110 A6:
    |_, _| Z80::unsupported_operation(),
    // 10100111 A7:
    |_, _| Z80::unsupported_operation(),
    // 10101000 A8: LDD
    Z80::ldd,
    // 10101001 A9: CPD
    |z80, mem| z80.cpd(mem),
    // 10101010 AA:
    |_, _| Z80::unsupported_operation(),
    // 10101011 AB:
    |_, _| Z80::unsupported_operation(),
    // 10101100 AC:
    |_, _| Z80::unsupported_operation(),
    // 10101101 AD:
    |_, _| Z80::unsupported_operation(),
    // 10101110 AE:
    |_, _| Z80::unsupported_operation(),
    // 10101111 AF:
    |_, _| Z80::unsupported_operation(),
    // 10110000 B0: LDIR
    Z80::ldir,
    // 10110001 B1: CPIR
    |z80, mem| z80.cpir(mem),
    // 10110010 B2:
    |_, _| Z80::unsupported_operation(),
    // 10110011 B3:
    |_, _| Z80::unsupported_operation(),
    // 10110100 B4:
    |_, _| Z80::unsupported_operation(),
    // 10110101 B5:
    |_, _| Z80::unsupported_operation(),
    // 10110110 B6:
    |_, _| Z80::unsupported_operation(),
    // 10110111 B7:
    |_, _| Z80::unsupported_operation(),
    // 10111000 B8: LDDR
    Z80::lddr,
    // 10111001 B9: CPDR
    |z80, mem| z80.cpdr(mem),
    // 10111010 BA:
    |_, _| Z80::unsupported_operation(),
    // 10111011 BB:
    |_, _| Z80::unsupported_operation(),
    // 10111100 BC:
    |_, _| Z80::unsupported_operation(),
    // 10111101 BD:
    |_, _| Z80::unsupported_operation(),
    // 10111110 BE:
    |_, _| Z80::unsupported_operation(),
    // 10111111 BF:
    |_, _| Z80::unsupported_operation(),
    // 11000000 C0:
    |_, _| Z80::unsupported_operation(),
    // 11000001 C1:
    |_, _| Z80::unsupported_operation(),
    // 11000010 C2:
    |_, _| Z80::unsupported_operation(),
    // 11000011 C3:
    |_, _| Z80::unsupported_operation(),
    // 11000100 C4:
    |_, _| Z80::unsupported_operation(),
    // 11000101 C5:
    |_, _| Z80::unsupported_operation(),
    // 11000110 C6:
    |_, _| Z80::unsupported_operation(),
    // 11000111 C7:
    |_, _| Z80::unsupported_operation(),
    // 11001000 C8:
    |_, _| Z80::unsupported_operation(),
    // 11001001 C9:
    |_, _| Z80::unsupported_operation(),
    // 11001010 CA:
    |_, _| Z80::unsupported_operation(),
    // 11001011 CB:
    |_, _| Z80::unsupported_operation(),
    // 11001100 CC:
    |_, _| Z80::unsupported_operation(),
    // 11001101 CD:
    |_, _| Z80::unsupported_operation(),
    // 11001110 CE:
    |_, _| Z80::unsupported_operation(),
    // 11001111 CF:
    |_, _| Z80::unsupported_operation(),
    // 11010000 D0:
    |_, _| Z80::unsupported_operation(),
    // 11010001 D1:
    |_, _| Z80::unsupported_operation(),
    // 11010010 D2:
    |_, _| Z80::unsupported_operation(),
    // 11010011 D3:
    |_, _| Z80::unsupported_operation(),
    // 11010100 D4:
    |_, _| Z80::unsupported_operation(),
    // 11010101 D5:
    |_, _| Z80::unsupported_operation(),
    // 11010110 D6:
    |_, _| Z80::unsupported_operation(),
    // 11010111 D7:
    |_, _| Z80::unsupported_operation(),
    // 11011000 D8:
    |_, _| Z80::unsupported_operation(),
    // 11011001 D9:
    |_, _| Z80::unsupported_operation(),
    // 11011010 DA:
    |_, _| Z80::unsupported_operation(),
    // 11011011 DB:
    |_, _| Z80::unsupported_operation(),
    // 11011100 DC:
    |_, _| Z80::unsupported_operation(),
    // 11011101 DD:
    |_, _| Z80::unsupported_operation(),
    // 11011110 DE:
    |_, _| Z80::unsupported_operation(),
    // 11011111 DF:
    |_, _| Z80::unsupported_operation(),
    // 11100000 E0:
    |_, _| Z80::unsupported_operation(),
    // 11100001 E1:
    |_, _| Z80::unsupported_operation(),
    // 11100010 E2:
    |_, _| Z80::unsupported_operation(),
    // 11100011 E3:
    |_, _| Z80::unsupported_operation(),
    // 11100100 E4:
    |_, _| Z80::unsupported_operation(),
    // 11100101 E5:
    |_, _| Z80::unsupported_operation(),
    // 11100110 E6:
    |_, _| Z80::unsupported_operation(),
    // 11100111 E7:
    |_, _| Z80::unsupported_operation(),
    // 11101000 E8:
    |_, _| Z80::unsupported_operation(),
    // 11101001 E9:
    |_, _| Z80::unsupported_operation(),
    // 11101010 EA:
    |_, _| Z80::unsupported_operation(),
    // 11101011 EB:
    |_, _| Z80::unsupported_operation(),
    // 11101100 EC:
    |_, _| Z80::unsupported_operation(),
    // 11101101 ED:
    |_, _| Z80::unsupported_operation(),
    // 11101110 EE:
    |_, _| Z80::unsupported_operation(),
    // 11101111 EF:
    |_, _| Z80::unsupported_operation(),
    // 11110000 F0:
    |_, _| Z80::unsupported_operation(),
    // 11110001 F1:
    |_, _| Z80::unsupported_operation(),
    // 11110010 F2:
    |_, _| Z80::unsupported_operation(),
    // 11110011 F3:
    |_, _| Z80::unsupported_operation(),
    // 11110100 F4:
    |_, _| Z80::unsupported_operation(),
    // 11110101 F5:
    |_, _| Z80::unsupported_operation(),
    // 11110110 F6:
    |_, _| Z80::unsupported_operation(),
    // 11110111 F7:
    |_, _| Z80::unsupported_operation(),
    // 11111000 F8:
    |_, _| Z80::unsupported_operation(),
    // 11111001 F9:
    |_, _| Z80::unsupported_operation(),
    // 11111010 FA:
    |_, _| Z80::unsupported_operation(),
    // 11111011 FB:
    |_, _| Z80::unsupported_operation(),
    // 11111100 FC:
    |_, _| Z80::unsupported_operation(),
    // 11111101 FD:
    |_, _| Z80::unsupported_operation(),
    // 11111110 FE:
    |_, _| Z80::unsupported_operation(),
    // 11111111 FF:
    |_, _| Z80::unsupported_operation(),
];

// FD prefix
const IY_FUNCTIONS: [fn(&mut Z80, &mut dyn Z80Memory) -> u8; 256] = [
    // 00000000 00:
    |_, _| Z80::unsupported_operation(),
    // 00000001 01:
    |_, _| Z80::unsupported_operation(),
    // 00000010 02:
    |_, _| Z80::unsupported_operation(),
    // 00000011 03:
    |_, _| Z80::unsupported_operation(),
    // 00000100 04:
    |_, _| Z80::unsupported_operation(),
    // 00000101 05:
    |_, _| Z80::unsupported_operation(),
    // 00000110 06:
    |_, _| Z80::unsupported_operation(),
    // 00000111 07:
    |_, _| Z80::unsupported_operation(),
    // 00001000 08:
    |_, _| Z80::unsupported_operation(),
    // 00001001 09:
    |_, _| Z80::unsupported_operation(),
    // 00001010 0A:
    |_, _| Z80::unsupported_operation(),
    // 00001011 0B:
    |_, _| Z80::unsupported_operation(),
    // 00001100 0C:
    |_, _| Z80::unsupported_operation(),
    // 00001101 0D:
    |_, _| Z80::unsupported_operation(),
    // 00001110 0E:
    |_, _| Z80::unsupported_operation(),
    // 00001111 0F:
    |_, _| Z80::unsupported_operation(),
    // 00010000 10:
    |_, _| Z80::unsupported_operation(),
    // 00010001 11:
    |_, _| Z80::unsupported_operation(),
    // 00010010 12:
    |_, _| Z80::unsupported_operation(),
    // 00010011 13:
    |_, _| Z80::unsupported_operation(),
    // 00010100 14:
    |_, _| Z80::unsupported_operation(),
    // 00010101 15:
    |_, _| Z80::unsupported_operation(),
    // 00010110 16:
    |_, _| Z80::unsupported_operation(),
    // 00010111 17:
    |_, _| Z80::unsupported_operation(),
    // 00011000 18:
    |_, _| Z80::unsupported_operation(),
    // 00011001 19:
    |_, _| Z80::unsupported_operation(),
    // 00011010 1A:
    |_, _| Z80::unsupported_operation(),
    // 00011011 1B:
    |_, _| Z80::unsupported_operation(),
    // 00011100 1C:
    |_, _| Z80::unsupported_operation(),
    // 00011101 1D:
    |_, _| Z80::unsupported_operation(),
    // 00011110 1E:
    |_, _| Z80::unsupported_operation(),
    // 00011111 1F:
    |_, _| Z80::unsupported_operation(),
    // 00100000 20:
    |_, _| Z80::unsupported_operation(),
    // 00100001 21: LD IY, nn
    |z80, mem| z80.ld_iy_nn(mem),
    // 00100010 22: LD (nn), IY
    Z80::ld_mem_nn_iy,
    // 00100011 23:
    |_, _| Z80::unsupported_operation(),
    // 00100100 24:
    |_, _| Z80::unsupported_operation(),
    // 00100101 25:
    |_, _| Z80::unsupported_operation(),
    // 00100110 26:
    |_, _| Z80::unsupported_operation(),
    // 00100111 27:
    |_, _| Z80::unsupported_operation(),
    // 00101000 28:
    |_, _| Z80::unsupported_operation(),
    // 00101001 29:
    |_, _| Z80::unsupported_operation(),
    // 00101010 2A: LD IY, (nn)
    |z80, mem| z80.ld_iy_mem_nn(mem),
    // 00101011 2B:
    |_, _| Z80::unsupported_operation(),
    // 00101100 2C:
    |_, _| Z80::unsupported_operation(),
    // 00101101 2D:
    |_, _| Z80::unsupported_operation(),
    // 00101110 2E:
    |_, _| Z80::unsupported_operation(),
    // 00101111 2F:
    |_, _| Z80::unsupported_operation(),
    // 00110000 30:
    |_, _| Z80::unsupported_operation(),
    // 00110001 31:
    |_, _| Z80::unsupported_operation(),
    // 00110010 32:
    |_, _| Z80::unsupported_operation(),
    // 00110011 33:
    |_, _| Z80::unsupported_operation(),
    // 00110100 34:
    |_, _| Z80::unsupported_operation(),
    // 00110101 35:
    |_, _| Z80::unsupported_operation(),
    // 00110110 36: LD (IY+d), n
    |z80, mem| z80.ld_mem_iyd_n(mem),
    // 00110111 37:
    |_, _| Z80::unsupported_operation(),
    // 00111000 38:
    |_, _| Z80::unsupported_operation(),
    // 00111001 39:
    |_, _| Z80::unsupported_operation(),
    // 00111010 3A:
    |_, _| Z80::unsupported_operation(),
    // 00111011 3B:
    |_, _| Z80::unsupported_operation(),
    // 00111100 3C:
    |_, _| Z80::unsupported_operation(),
    // 00111101 3D:
    |_, _| Z80::unsupported_operation(),
    // 00111110 3E:
    |_, _| Z80::unsupported_operation(),
    // 00111111 3F:
    |_, _| Z80::unsupported_operation(),
    // 01000000 40:
    |_, _| Z80::unsupported_operation(),
    // 01000001 41:
    |_, _| Z80::unsupported_operation(),
    // 01000010 42:
    |_, _| Z80::unsupported_operation(),
    // 01000011 43:
    |_, _| Z80::unsupported_operation(),
    // 01000100 44:
    |_, _| Z80::unsupported_operation(),
    // 01000101 45:
    |_, _| Z80::unsupported_operation(),
    // 01000110 46: LD B, (IY+d)
    |z80, mem| z80.ld_b_mem_iyd(mem),
    // 01000111 47:
    |_, _| Z80::unsupported_operation(),
    // 01001000 48:
    |_, _| Z80::unsupported_operation(),
    // 01001001 49:
    |_, _| Z80::unsupported_operation(),
    // 01001010 4A:
    |_, _| Z80::unsupported_operation(),
    // 01001011 4B:
    |_, _| Z80::unsupported_operation(),
    // 01001100 4C:
    |_, _| Z80::unsupported_operation(),
    // 01001101 4D:
    |_, _| Z80::unsupported_operation(),
    // 01001110 4E: LD C, (IY+d)
    |z80, mem| z80.ld_c_mem_iyd(mem),
    // 01001111 4F:
    |_, _| Z80::unsupported_operation(),
    // 01010000 50:
    |_, _| Z80::unsupported_operation(),
    // 01010001 51:
    |_, _| Z80::unsupported_operation(),
    // 01010010 52:
    |_, _| Z80::unsupported_operation(),
    // 01010011 53:
    |_, _| Z80::unsupported_operation(),
    // 01010100 54:
    |_, _| Z80::unsupported_operation(),
    // 01010101 55:
    |_, _| Z80::unsupported_operation(),
    // 01010110 56: LD D, (IY+d)
    |z80, mem| z80.ld_d_mem_iyd(mem),
    // 01010111 57:
    |_, _| Z80::unsupported_operation(),
    // 01011000 58:
    |_, _| Z80::unsupported_operation(),
    // 01011001 59:
    |_, _| Z80::unsupported_operation(),
    // 01011010 5A:
    |_, _| Z80::unsupported_operation(),
    // 01011011 5B:
    |_, _| Z80::unsupported_operation(),
    // 01011100 5C:
    |_, _| Z80::unsupported_operation(),
    // 01011101 5D:
    |_, _| Z80::unsupported_operation(),
    // 01011110 5E: LD E, (IY+d)
    |z80, mem| z80.ld_e_mem_iyd(mem),
    // 01011111 5F:
    |_, _| Z80::unsupported_operation(),
    // 01100000 60:
    |_, _| Z80::unsupported_operation(),
    // 01100001 61:
    |_, _| Z80::unsupported_operation(),
    // 01100010 62:
    |_, _| Z80::unsupported_operation(),
    // 01100011 63:
    |_, _| Z80::unsupported_operation(),
    // 01100100 64:
    |_, _| Z80::unsupported_operation(),
    // 01100101 65:
    |_, _| Z80::unsupported_operation(),
    // 01100110 66: LD H, (IY+d)
    |z80, mem| z80.ld_h_mem_iyd(mem),
    // 01100111 67:
    |_, _| Z80::unsupported_operation(),
    // 01101000 68:
    |_, _| Z80::unsupported_operation(),
    // 01101001 69:
    |_, _| Z80::unsupported_operation(),
    // 01101010 6A:
    |_, _| Z80::unsupported_operation(),
    // 01101011 6B:
    |_, _| Z80::unsupported_operation(),
    // 01101100 6C:
    |_, _| Z80::unsupported_operation(),
    // 01101101 6D:
    |_, _| Z80::unsupported_operation(),
    // 01101110 6E: LD L, (IY+d)
    |z80, mem| z80.ld_l_mem_iyd(mem),
    // 01101111 6F:
    |_, _| Z80::unsupported_operation(),
    // 01110000 70: LD (IY+d), B
    |z80, mem| z80.ld_mem_iyd_b(mem),
    // 01110001 71: LD (IY+d), C
    |z80, mem| z80.ld_mem_iyd_c(mem),
    // 01110010 72: LD (IY+d), D
    |z80, mem| z80.ld_mem_iyd_d(mem),
    // 01110011 73: LD (IY+d), E
    |z80, mem| z80.ld_mem_iyd_e(mem),
    // 01110100 74: LD (IY+d), H
    |z80, mem| z80.ld_mem_iyd_h(mem),
    // 01110101 75: LD (IY+d), L
    |z80, mem| z80.ld_mem_iyd_l(mem),
    // 01110110 76:
    |_, _| Z80::unsupported_operation(),
    // 01110111 77: LD (IY+d), A
    |z80, mem| z80.ld_mem_iyd_a(mem),
    // 01111000 78:
    |_, _| Z80::unsupported_operation(),
    // 01111001 79:
    |_, _| Z80::unsupported_operation(),
    // 01111010 7A:
    |_, _| Z80::unsupported_operation(),
    // 01111011 7B:
    |_, _| Z80::unsupported_operation(),
    // 01111100 7C:
    |_, _| Z80::unsupported_operation(),
    // 01111101 7D:
    |_, _| Z80::unsupported_operation(),
    // 01111110 7E: LD A, (IY+d)
    |z80, mem| z80.ld_a_mem_iyd(mem),
    // 01111111 7F:
    |_, _| Z80::unsupported_operation(),
    // 10000000 80:
    |_, _| Z80::unsupported_operation(),
    // 10000001 81:
    |_, _| Z80::unsupported_operation(),
    // 10000010 82:
    |_, _| Z80::unsupported_operation(),
    // 10000011 83:
    |_, _| Z80::unsupported_operation(),
    // 10000100 84:
    |_, _| Z80::unsupported_operation(),
    // 10000101 85:
    |_, _| Z80::unsupported_operation(),
    // 10000110 86:
    |_, _| Z80::unsupported_operation(),
    // 10000111 87:
    |_, _| Z80::unsupported_operation(),
    // 10001000 88:
    |_, _| Z80::unsupported_operation(),
    // 10001001 89:
    |_, _| Z80::unsupported_operation(),
    // 10001010 8A:
    |_, _| Z80::unsupported_operation(),
    // 10001011 8B:
    |_, _| Z80::unsupported_operation(),
    // 10001100 8C:
    |_, _| Z80::unsupported_operation(),
    // 10001101 8D:
    |_, _| Z80::unsupported_operation(),
    // 10001110 8E:
    |_, _| Z80::unsupported_operation(),
    // 10001111 8F:
    |_, _| Z80::unsupported_operation(),
    // 10010000 90:
    |_, _| Z80::unsupported_operation(),
    // 10010001 91:
    |_, _| Z80::unsupported_operation(),
    // 10010010 92:
    |_, _| Z80::unsupported_operation(),
    // 10010011 93:
    |_, _| Z80::unsupported_operation(),
    // 10010100 94:
    |_, _| Z80::unsupported_operation(),
    // 10010101 95:
    |_, _| Z80::unsupported_operation(),
    // 10010110 96:
    |_, _| Z80::unsupported_operation(),
    // 10010111 97:
    |_, _| Z80::unsupported_operation(),
    // 10011000 98:
    |_, _| Z80::unsupported_operation(),
    // 10011001 99:
    |_, _| Z80::unsupported_operation(),
    // 10011010 9A:
    |_, _| Z80::unsupported_operation(),
    // 10011011 9B:
    |_, _| Z80::unsupported_operation(),
    // 10011100 9C:
    |_, _| Z80::unsupported_operation(),
    // 10011101 9D:
    |_, _| Z80::unsupported_operation(),
    // 10011110 9E:
    |_, _| Z80::unsupported_operation(),
    // 10011111 9F:
    |_, _| Z80::unsupported_operation(),
    // 10100000 A0:
    |_, _| Z80::unsupported_operation(),
    // 10100001 A1:
    |_, _| Z80::unsupported_operation(),
    // 10100010 A2:
    |_, _| Z80::unsupported_operation(),
    // 10100011 A3:
    |_, _| Z80::unsupported_operation(),
    // 10100100 A4:
    |_, _| Z80::unsupported_operation(),
    // 10100101 A5:
    |_, _| Z80::unsupported_operation(),
    // 10100110 A6:
    |_, _| Z80::unsupported_operation(),
    // 10100111 A7:
    |_, _| Z80::unsupported_operation(),
    // 10101000 A8:
    |_, _| Z80::unsupported_operation(),
    // 10101001 A9:
    |_, _| Z80::unsupported_operation(),
    // 10101010 AA:
    |_, _| Z80::unsupported_operation(),
    // 10101011 AB:
    |_, _| Z80::unsupported_operation(),
    // 10101100 AC:
    |_, _| Z80::unsupported_operation(),
    // 10101101 AD:
    |_, _| Z80::unsupported_operation(),
    // 10101110 AE:
    |_, _| Z80::unsupported_operation(),
    // 10101111 AF:
    |_, _| Z80::unsupported_operation(),
    // 10110000 B0:
    |_, _| Z80::unsupported_operation(),
    // 10110001 B1:
    |_, _| Z80::unsupported_operation(),
    // 10110010 B2:
    |_, _| Z80::unsupported_operation(),
    // 10110011 B3:
    |_, _| Z80::unsupported_operation(),
    // 10110100 B4:
    |_, _| Z80::unsupported_operation(),
    // 10110101 B5:
    |_, _| Z80::unsupported_operation(),
    // 10110110 B6:
    |_, _| Z80::unsupported_operation(),
    // 10110111 B7:
    |_, _| Z80::unsupported_operation(),
    // 10111000 B8:
    |_, _| Z80::unsupported_operation(),
    // 10111001 B9:
    |_, _| Z80::unsupported_operation(),
    // 10111010 BA:
    |_, _| Z80::unsupported_operation(),
    // 10111011 BB:
    |_, _| Z80::unsupported_operation(),
    // 10111100 BC:
    |_, _| Z80::unsupported_operation(),
    // 10111101 BD:
    |_, _| Z80::unsupported_operation(),
    // 10111110 BE:
    |_, _| Z80::unsupported_operation(),
    // 10111111 BF:
    |_, _| Z80::unsupported_operation(),
    // 11000000 C0:
    |_, _| Z80::unsupported_operation(),
    // 11000001 C1:
    |_, _| Z80::unsupported_operation(),
    // 11000010 C2:
    |_, _| Z80::unsupported_operation(),
    // 11000011 C3:
    |_, _| Z80::unsupported_operation(),
    // 11000100 C4:
    |_, _| Z80::unsupported_operation(),
    // 11000101 C5:
    |_, _| Z80::unsupported_operation(),
    // 11000110 C6:
    |_, _| Z80::unsupported_operation(),
    // 11000111 C7:
    |_, _| Z80::unsupported_operation(),
    // 11001000 C8:
    |_, _| Z80::unsupported_operation(),
    // 11001001 C9:
    |_, _| Z80::unsupported_operation(),
    // 11001010 CA:
    |_, _| Z80::unsupported_operation(),
    // 11001011 CB:
    |_, _| Z80::unsupported_operation(),
    // 11001100 CC:
    |_, _| Z80::unsupported_operation(),
    // 11001101 CD:
    |_, _| Z80::unsupported_operation(),
    // 11001110 CE:
    |_, _| Z80::unsupported_operation(),
    // 11001111 CF:
    |_, _| Z80::unsupported_operation(),
    // 11010000 D0:
    |_, _| Z80::unsupported_operation(),
    // 11010001 D1:
    |_, _| Z80::unsupported_operation(),
    // 11010010 D2:
    |_, _| Z80::unsupported_operation(),
    // 11010011 D3:
    |_, _| Z80::unsupported_operation(),
    // 11010100 D4:
    |_, _| Z80::unsupported_operation(),
    // 11010101 D5:
    |_, _| Z80::unsupported_operation(),
    // 11010110 D6:
    |_, _| Z80::unsupported_operation(),
    // 11010111 D7:
    |_, _| Z80::unsupported_operation(),
    // 11011000 D8:
    |_, _| Z80::unsupported_operation(),
    // 11011001 D9:
    |_, _| Z80::unsupported_operation(),
    // 11011010 DA:
    |_, _| Z80::unsupported_operation(),
    // 11011011 DB:
    |_, _| Z80::unsupported_operation(),
    // 11011100 DC:
    |_, _| Z80::unsupported_operation(),
    // 11011101 DD:
    |_, _| Z80::unsupported_operation(),
    // 11011110 DE:
    |_, _| Z80::unsupported_operation(),
    // 11011111 DF:
    |_, _| Z80::unsupported_operation(),
    // 11100000 E0:
    |_, _| Z80::unsupported_operation(),
    // 11100001 E1: POP IY
    |z80, mem| z80.pop_iy(mem),
    // 11100010 E2:
    |_, _| Z80::unsupported_operation(),
    // 11100011 E3: EX (SP), IY
    Z80::ex_mem_sp_iy,
    // 11100100 E4:
    |_, _| Z80::unsupported_operation(),
    // 11100101 E5: PUSH IY
    Z80::push_iy,
    // 11100110 E6:
    |_, _| Z80::unsupported_operation(),
    // 11100111 E7:
    |_, _| Z80::unsupported_operation(),
    // 11101000 E8:
    |_, _| Z80::unsupported_operation(),
    // 11101001 E9:
    |_, _| Z80::unsupported_operation(),
    // 11101010 EA:
    |_, _| Z80::unsupported_operation(),
    // 11101011 EB:
    |_, _| Z80::unsupported_operation(),
    // 11101100 EC:
    |_, _| Z80::unsupported_operation(),
    // 11101101 ED:
    |_, _| Z80::unsupported_operation(),
    // 11101110 EE:
    |_, _| Z80::unsupported_operation(),
    // 11101111 EF:
    |_, _| Z80::unsupported_operation(),
    // 11110000 F0:
    |_, _| Z80::unsupported_operation(),
    // 11110001 F1:
    |_, _| Z80::unsupported_operation(),
    // 11110010 F2:
    |_, _| Z80::unsupported_operation(),
    // 11110011 F3:
    |_, _| Z80::unsupported_operation(),
    // 11110100 F4:
    |_, _| Z80::unsupported_operation(),
    // 11110101 F5:
    |_, _| Z80::unsupported_operation(),
    // 11110110 F6:
    |_, _| Z80::unsupported_operation(),
    // 11110111 F7:
    |_, _| Z80::unsupported_operation(),
    // 11111000 F8:
    |_, _| Z80::unsupported_operation(),
    // 11111001 F9: LD SP, IY
    |z80, _| z80.ld_sp_iy(),
    // 11111010 FA:
    |_, _| Z80::unsupported_operation(),
    // 11111011 FB:
    |_, _| Z80::unsupported_operation(),
    // 11111100 FC:
    |_, _| Z80::unsupported_operation(),
    // 11111101 FD:
    |_, _| Z80::unsupported_operation(),
    // 11111110 FE:
    |_, _| Z80::unsupported_operation(),
    // 11111111 FF:
    |_, _| Z80::unsupported_operation(),
];

// FDCB prefix
const IY_BIT_INSTRUCTIONS: [fn(&mut Z80, &mut dyn Z80Memory) -> u8; 256] = [
    // 00000000 00:
    |_, _| Z80::unsupported_operation(),
    // 00000001 01:
    |_, _| Z80::unsupported_operation(),
    // 00000010 02:
    |_, _| Z80::unsupported_operation(),
    // 00000011 03:
    |_, _| Z80::unsupported_operation(),
    // 00000100 04:
    |_, _| Z80::unsupported_operation(),
    // 00000101 05:
    |_, _| Z80::unsupported_operation(),
    // 00000110 06:
    |_, _| Z80::unsupported_operation(),
    // 00000111 07:
    |_, _| Z80::unsupported_operation(),
    // 00001000 08:
    |_, _| Z80::unsupported_operation(),
    // 00001001 09:
    |_, _| Z80::unsupported_operation(),
    // 00001010 0A:
    |_, _| Z80::unsupported_operation(),
    // 00001011 0B:
    |_, _| Z80::unsupported_operation(),
    // 00001100 0C:
    |_, _| Z80::unsupported_operation(),
    // 00001101 0D:
    |_, _| Z80::unsupported_operation(),
    // 00001110 0E:
    |_, _| Z80::unsupported_operation(),
    // 00001111 0F:
    |_, _| Z80::unsupported_operation(),
    // 00010000 10:
    |_, _| Z80::unsupported_operation(),
    // 00010001 11:
    |_, _| Z80::unsupported_operation(),
    // 00010010 12:
    |_, _| Z80::unsupported_operation(),
    // 00010011 13:
    |_, _| Z80::unsupported_operation(),
    // 00010100 14:
    |_, _| Z80::unsupported_operation(),
    // 00010101 15:
    |_, _| Z80::unsupported_operation(),
    // 00010110 16:
    |_, _| Z80::unsupported_operation(),
    // 00010111 17:
    |_, _| Z80::unsupported_operation(),
    // 00011000 18:
    |_, _| Z80::unsupported_operation(),
    // 00011001 19:
    |_, _| Z80::unsupported_operation(),
    // 00011010 1A:
    |_, _| Z80::unsupported_operation(),
    // 00011011 1B:
    |_, _| Z80::unsupported_operation(),
    // 00011100 1C:
    |_, _| Z80::unsupported_operation(),
    // 00011101 1D:
    |_, _| Z80::unsupported_operation(),
    // 00011110 1E:
    |_, _| Z80::unsupported_operation(),
    // 00011111 1F:
    |_, _| Z80::unsupported_operation(),
    // 00100000 20:
    |_, _| Z80::unsupported_operation(),
    // 00100001 21:
    |_, _| Z80::unsupported_operation(),
    // 00100010 22:
    |_, _| Z80::unsupported_operation(),
    // 00100011 23:
    |_, _| Z80::unsupported_operation(),
    // 00100100 24:
    |_, _| Z80::unsupported_operation(),
    // 00100101 25:
    |_, _| Z80::unsupported_operation(),
    // 00100110 26:
    |_, _| Z80::unsupported_operation(),
    // 00100111 27:
    |_, _| Z80::unsupported_operation(),
    // 00101000 28:
    |_, _| Z80::unsupported_operation(),
    // 00101001 29:
    |_, _| Z80::unsupported_operation(),
    // 00101010 2A:
    |_, _| Z80::unsupported_operation(),
    // 00101011 2B:
    |_, _| Z80::unsupported_operation(),
    // 00101100 2C:
    |_, _| Z80::unsupported_operation(),
    // 00101101 2D:
    |_, _| Z80::unsupported_operation(),
    // 00101110 2E:
    |_, _| Z80::unsupported_operation(),
    // 00101111 2F:
    |_, _| Z80::unsupported_operation(),
    // 00110000 30:
    |_, _| Z80::unsupported_operation(),
    // 00110001 31:
    |_, _| Z80::unsupported_operation(),
    // 00110010 32:
    |_, _| Z80::unsupported_operation(),
    // 00110011 33:
    |_, _| Z80::unsupported_operation(),
    // 00110100 34:
    |_, _| Z80::unsupported_operation(),
    // 00110101 35:
    |_, _| Z80::unsupported_operation(),
    // 00110110 36:
    |_, _| Z80::unsupported_operation(),
    // 00110111 37:
    |_, _| Z80::unsupported_operation(),
    // 00111000 38:
    |_, _| Z80::unsupported_operation(),
    // 00111001 39:
    |_, _| Z80::unsupported_operation(),
    // 00111010 3A:
    |_, _| Z80::unsupported_operation(),
    // 00111011 3B:
    |_, _| Z80::unsupported_operation(),
    // 00111100 3C:
    |_, _| Z80::unsupported_operation(),
    // 00111101 3D:
    |_, _| Z80::unsupported_operation(),
    // 00111110 3E:
    |_, _| Z80::unsupported_operation(),
    // 00111111 3F:
    |_, _| Z80::unsupported_operation(),
    // 01000000 40:
    |_, _| Z80::unsupported_operation(),
    // 01000001 41:
    |_, _| Z80::unsupported_operation(),
    // 01000010 42:
    |_, _| Z80::unsupported_operation(),
    // 01000011 43:
    |_, _| Z80::unsupported_operation(),
    // 01000100 44:
    |_, _| Z80::unsupported_operation(),
    // 01000101 45:
    |_, _| Z80::unsupported_operation(),
    // 01000110 46:
    |_, _| Z80::unsupported_operation(),
    // 01000111 47:
    |_, _| Z80::unsupported_operation(),
    // 01001000 48:
    |_, _| Z80::unsupported_operation(),
    // 01001001 49:
    |_, _| Z80::unsupported_operation(),
    // 01001010 4A:
    |_, _| Z80::unsupported_operation(),
    // 01001011 4B:
    |_, _| Z80::unsupported_operation(),
    // 01001100 4C:
    |_, _| Z80::unsupported_operation(),
    // 01001101 4D:
    |_, _| Z80::unsupported_operation(),
    // 01001110 4E:
    |_, _| Z80::unsupported_operation(),
    // 01001111 4F:
    |_, _| Z80::unsupported_operation(),
    // 01010000 50:
    |_, _| Z80::unsupported_operation(),
    // 01010001 51:
    |_, _| Z80::unsupported_operation(),
    // 01010010 52:
    |_, _| Z80::unsupported_operation(),
    // 01010011 53:
    |_, _| Z80::unsupported_operation(),
    // 01010100 54:
    |_, _| Z80::unsupported_operation(),
    // 01010101 55:
    |_, _| Z80::unsupported_operation(),
    // 01010110 56:
    |_, _| Z80::unsupported_operation(),
    // 01010111 57:
    |_, _| Z80::unsupported_operation(),
    // 01011000 58:
    |_, _| Z80::unsupported_operation(),
    // 01011001 59:
    |_, _| Z80::unsupported_operation(),
    // 01011010 5A:
    |_, _| Z80::unsupported_operation(),
    // 01011011 5B:
    |_, _| Z80::unsupported_operation(),
    // 01011100 5C:
    |_, _| Z80::unsupported_operation(),
    // 01011101 5D:
    |_, _| Z80::unsupported_operation(),
    // 01011110 5E:
    |_, _| Z80::unsupported_operation(),
    // 01011111 5F:
    |_, _| Z80::unsupported_operation(),
    // 01100000 60:
    |_, _| Z80::unsupported_operation(),
    // 01100001 61:
    |_, _| Z80::unsupported_operation(),
    // 01100010 62:
    |_, _| Z80::unsupported_operation(),
    // 01100011 63:
    |_, _| Z80::unsupported_operation(),
    // 01100100 64:
    |_, _| Z80::unsupported_operation(),
    // 01100101 65:
    |_, _| Z80::unsupported_operation(),
    // 01100110 66:
    |_, _| Z80::unsupported_operation(),
    // 01100111 67:
    |_, _| Z80::unsupported_operation(),
    // 01101000 68:
    |_, _| Z80::unsupported_operation(),
    // 01101001 69:
    |_, _| Z80::unsupported_operation(),
    // 01101010 6A:
    |_, _| Z80::unsupported_operation(),
    // 01101011 6B:
    |_, _| Z80::unsupported_operation(),
    // 01101100 6C:
    |_, _| Z80::unsupported_operation(),
    // 01101101 6D:
    |_, _| Z80::unsupported_operation(),
    // 01101110 6E:
    |_, _| Z80::unsupported_operation(),
    // 01101111 6F:
    |_, _| Z80::unsupported_operation(),
    // 01110000 70:
    |_, _| Z80::unsupported_operation(),
    // 01110001 71:
    |_, _| Z80::unsupported_operation(),
    // 01110010 72:
    |_, _| Z80::unsupported_operation(),
    // 01110011 73:
    |_, _| Z80::unsupported_operation(),
    // 01110100 74:
    |_, _| Z80::unsupported_operation(),
    // 01110101 75:
    |_, _| Z80::unsupported_operation(),
    // 01110110 76:
    |_, _| Z80::unsupported_operation(),
    // 01110111 77:
    |_, _| Z80::unsupported_operation(),
    // 01111000 78:
    |_, _| Z80::unsupported_operation(),
    // 01111001 79:
    |_, _| Z80::unsupported_operation(),
    // 01111010 7A:
    |_, _| Z80::unsupported_operation(),
    // 01111011 7B:
    |_, _| Z80::unsupported_operation(),
    // 01111100 7C:
    |_, _| Z80::unsupported_operation(),
    // 01111101 7D:
    |_, _| Z80::unsupported_operation(),
    // 01111110 7E:
    |_, _| Z80::unsupported_operation(),
    // 01111111 7F:
    |_, _| Z80::unsupported_operation(),
    // 10000000 80:
    |_, _| Z80::unsupported_operation(),
    // 10000001 81:
    |_, _| Z80::unsupported_operation(),
    // 10000010 82:
    |_, _| Z80::unsupported_operation(),
    // 10000011 83:
    |_, _| Z80::unsupported_operation(),
    // 10000100 84:
    |_, _| Z80::unsupported_operation(),
    // 10000101 85:
    |_, _| Z80::unsupported_operation(),
    // 10000110 86:
    |_, _| Z80::unsupported_operation(),
    // 10000111 87:
    |_, _| Z80::unsupported_operation(),
    // 10001000 88:
    |_, _| Z80::unsupported_operation(),
    // 10001001 89:
    |_, _| Z80::unsupported_operation(),
    // 10001010 8A:
    |_, _| Z80::unsupported_operation(),
    // 10001011 8B:
    |_, _| Z80::unsupported_operation(),
    // 10001100 8C:
    |_, _| Z80::unsupported_operation(),
    // 10001101 8D:
    |_, _| Z80::unsupported_operation(),
    // 10001110 8E:
    |_, _| Z80::unsupported_operation(),
    // 10001111 8F:
    |_, _| Z80::unsupported_operation(),
    // 10010000 90:
    |_, _| Z80::unsupported_operation(),
    // 10010001 91:
    |_, _| Z80::unsupported_operation(),
    // 10010010 92:
    |_, _| Z80::unsupported_operation(),
    // 10010011 93:
    |_, _| Z80::unsupported_operation(),
    // 10010100 94:
    |_, _| Z80::unsupported_operation(),
    // 10010101 95:
    |_, _| Z80::unsupported_operation(),
    // 10010110 96:
    |_, _| Z80::unsupported_operation(),
    // 10010111 97:
    |_, _| Z80::unsupported_operation(),
    // 10011000 98:
    |_, _| Z80::unsupported_operation(),
    // 10011001 99:
    |_, _| Z80::unsupported_operation(),
    // 10011010 9A:
    |_, _| Z80::unsupported_operation(),
    // 10011011 9B:
    |_, _| Z80::unsupported_operation(),
    // 10011100 9C:
    |_, _| Z80::unsupported_operation(),
    // 10011101 9D:
    |_, _| Z80::unsupported_operation(),
    // 10011110 9E:
    |_, _| Z80::unsupported_operation(),
    // 10011111 9F:
    |_, _| Z80::unsupported_operation(),
    // 10100000 A0:
    |_, _| Z80::unsupported_operation(),
    // 10100001 A1:
    |_, _| Z80::unsupported_operation(),
    // 10100010 A2:
    |_, _| Z80::unsupported_operation(),
    // 10100011 A3:
    |_, _| Z80::unsupported_operation(),
    // 10100100 A4:
    |_, _| Z80::unsupported_operation(),
    // 10100101 A5:
    |_, _| Z80::unsupported_operation(),
    // 10100110 A6:
    |_, _| Z80::unsupported_operation(),
    // 10100111 A7:
    |_, _| Z80::unsupported_operation(),
    // 10101000 A8:
    |_, _| Z80::unsupported_operation(),
    // 10101001 A9:
    |_, _| Z80::unsupported_operation(),
    // 10101010 AA:
    |_, _| Z80::unsupported_operation(),
    // 10101011 AB:
    |_, _| Z80::unsupported_operation(),
    // 10101100 AC:
    |_, _| Z80::unsupported_operation(),
    // 10101101 AD:
    |_, _| Z80::unsupported_operation(),
    // 10101110 AE:
    |_, _| Z80::unsupported_operation(),
    // 10101111 AF:
    |_, _| Z80::unsupported_operation(),
    // 10110000 B0:
    |_, _| Z80::unsupported_operation(),
    // 10110001 B1:
    |_, _| Z80::unsupported_operation(),
    // 10110010 B2:
    |_, _| Z80::unsupported_operation(),
    // 10110011 B3:
    |_, _| Z80::unsupported_operation(),
    // 10110100 B4:
    |_, _| Z80::unsupported_operation(),
    // 10110101 B5:
    |_, _| Z80::unsupported_operation(),
    // 10110110 B6:
    |_, _| Z80::unsupported_operation(),
    // 10110111 B7:
    |_, _| Z80::unsupported_operation(),
    // 10111000 B8:
    |_, _| Z80::unsupported_operation(),
    // 10111001 B9:
    |_, _| Z80::unsupported_operation(),
    // 10111010 BA:
    |_, _| Z80::unsupported_operation(),
    // 10111011 BB:
    |_, _| Z80::unsupported_operation(),
    // 10111100 BC:
    |_, _| Z80::unsupported_operation(),
    // 10111101 BD:
    |_, _| Z80::unsupported_operation(),
    // 10111110 BE:
    |_, _| Z80::unsupported_operation(),
    // 10111111 BF:
    |_, _| Z80::unsupported_operation(),
    // 11000000 C0:
    |_, _| Z80::unsupported_operation(),
    // 11000001 C1:
    |_, _| Z80::unsupported_operation(),
    // 11000010 C2:
    |_, _| Z80::unsupported_operation(),
    // 11000011 C3:
    |_, _| Z80::unsupported_operation(),
    // 11000100 C4:
    |_, _| Z80::unsupported_operation(),
    // 11000101 C5:
    |_, _| Z80::unsupported_operation(),
    // 11000110 C6:
    |_, _| Z80::unsupported_operation(),
    // 11000111 C7:
    |_, _| Z80::unsupported_operation(),
    // 11001000 C8:
    |_, _| Z80::unsupported_operation(),
    // 11001001 C9:
    |_, _| Z80::unsupported_operation(),
    // 11001010 CA:
    |_, _| Z80::unsupported_operation(),
    // 11001011 CB:
    |_, _| Z80::unsupported_operation(),
    // 11001100 CC:
    |_, _| Z80::unsupported_operation(),
    // 11001101 CD:
    |_, _| Z80::unsupported_operation(),
    // 11001110 CE:
    |_, _| Z80::unsupported_operation(),
    // 11001111 CF:
    |_, _| Z80::unsupported_operation(),
    // 11010000 D0:
    |_, _| Z80::unsupported_operation(),
    // 11010001 D1:
    |_, _| Z80::unsupported_operation(),
    // 11010010 D2:
    |_, _| Z80::unsupported_operation(),
    // 11010011 D3:
    |_, _| Z80::unsupported_operation(),
    // 11010100 D4:
    |_, _| Z80::unsupported_operation(),
    // 11010101 D5:
    |_, _| Z80::unsupported_operation(),
    // 11010110 D6:
    |_, _| Z80::unsupported_operation(),
    // 11010111 D7:
    |_, _| Z80::unsupported_operation(),
    // 11011000 D8:
    |_, _| Z80::unsupported_operation(),
    // 11011001 D9:
    |_, _| Z80::unsupported_operation(),
    // 11011010 DA:
    |_, _| Z80::unsupported_operation(),
    // 11011011 DB:
    |_, _| Z80::unsupported_operation(),
    // 11011100 DC:
    |_, _| Z80::unsupported_operation(),
    // 11011101 DD:
    |_, _| Z80::unsupported_operation(),
    // 11011110 DE:
    |_, _| Z80::unsupported_operation(),
    // 11011111 DF:
    |_, _| Z80::unsupported_operation(),
    // 11100000 E0:
    |_, _| Z80::unsupported_operation(),
    // 11100001 E1:
    |_, _| Z80::unsupported_operation(),
    // 11100010 E2:
    |_, _| Z80::unsupported_operation(),
    // 11100011 E3:
    |_, _| Z80::unsupported_operation(),
    // 11100100 E4:
    |_, _| Z80::unsupported_operation(),
    // 11100101 E5:
    |_, _| Z80::unsupported_operation(),
    // 11100110 E6:
    |_, _| Z80::unsupported_operation(),
    // 11100111 E7:
    |_, _| Z80::unsupported_operation(),
    // 11101000 E8:
    |_, _| Z80::unsupported_operation(),
    // 11101001 E9:
    |_, _| Z80::unsupported_operation(),
    // 11101010 EA:
    |_, _| Z80::unsupported_operation(),
    // 11101011 EB:
    |_, _| Z80::unsupported_operation(),
    // 11101100 EC:
    |_, _| Z80::unsupported_operation(),
    // 11101101 ED:
    |_, _| Z80::unsupported_operation(),
    // 11101110 EE:
    |_, _| Z80::unsupported_operation(),
    // 11101111 EF:
    |_, _| Z80::unsupported_operation(),
    // 11110000 F0:
    |_, _| Z80::unsupported_operation(),
    // 11110001 F1:
    |_, _| Z80::unsupported_operation(),
    // 11110010 F2:
    |_, _| Z80::unsupported_operation(),
    // 11110011 F3:
    |_, _| Z80::unsupported_operation(),
    // 11110100 F4:
    |_, _| Z80::unsupported_operation(),
    // 11110101 F5:
    |_, _| Z80::unsupported_operation(),
    // 11110110 F6:
    |_, _| Z80::unsupported_operation(),
    // 11110111 F7:
    |_, _| Z80::unsupported_operation(),
    // 11111000 F8:
    |_, _| Z80::unsupported_operation(),
    // 11111001 F9:
    |_, _| Z80::unsupported_operation(),
    // 11111010 FA:
    |_, _| Z80::unsupported_operation(),
    // 11111011 FB:
    |_, _| Z80::unsupported_operation(),
    // 11111100 FC:
    |_, _| Z80::unsupported_operation(),
    // 11111101 FD:
    |_, _| Z80::unsupported_operation(),
    // 11111110 FE:
    |_, _| Z80::unsupported_operation(),
    // 11111111 FF:
    |_, _| Z80::unsupported_operation(),
    ];

pub trait Z80Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Register {
    value: u8,
}

impl Register {
    pub fn new() -> Register {
        Register { value: 0 }
    }
    pub fn from(value: u8) -> Register {
        Register { value }
    }
    pub fn inc(&mut self) {
        self.value = self.value.wrapping_add(1);
    }
    pub fn dec(&mut self) {
        self.value = self.value.wrapping_sub(1);
    }
    pub fn add(&mut self, rhs: u8) {
        self.value = self.value.wrapping_add(rhs);
    }
    pub fn sub(&mut self, rhs: u8) {
        self.value = self.value.wrapping_sub(rhs);
    }
    pub fn value(&self) -> u8 {
        self.value
    }
    pub fn set_value(&mut self, rhs: u8) {
        self.value = rhs;
    }
}

#[derive(Clone, Debug)]
pub struct Z80 {
    /// Program Counter (PC).
    /// The program counter holds the 16-bit address of the current instruction being fetched from memory.
    /// The Program Counter is automatically incremented after its contents are transferred to the address lines.
    /// When a program jump occurs, the new value is automatically placed in the Program Counter,
    /// overriding the incrementer.
    pc: u16,

    /// Stack Pointer (SP).
    /// The stack pointer holds the 16-bit address of the current top of a stack located anywhere
    /// in external system RAM memory. The external stack memory is organized as a last-in
    /// first-out (LIFO) file. Data can be pushed onto the stack from specific CPU registers or popped
    /// off of the stack to specific CPU registers through the execution of PUSH and POP instructions.
    /// The data popped from the stack is always the most recent data pushed onto it. The stack allows
    /// simple implementation of multiple level interrupts, unlimited subroutine nesting and
    /// simplification of many types of data manipulation.
    sp: u16,

    /// Index Register (IX).
    /// The two independent index registers hold a 16-bit base address that is used in indexed addressing modes.
    /// In this mode, an index register is used as a base to point to a region in memory from which data is to
    /// be stored or retrieved. An additional byte is included in indexed instructions to specify a displacement
    /// from this base. This displacement is specified as a two’s complement signed integer. This mode of
    /// addressing greatly simplifies many types of programs, especially when tables of data are used.
    ix: u16,

    /// Index Register (IY).
    /// The two independent index registers hold a 16-bit base address that is used in indexed addressing modes.
    /// In this mode, an index register is used as a base to point to a region in memory from which data is to
    /// be stored or retrieved. An additional byte is included in indexed instructions to specify a displacement
    /// from this base. This displacement is specified as a two’s complement signed integer. This mode of
    /// addressing greatly simplifies many types of programs, especially when tables of data are used.
    iy: u16,

    /// Interrupt Page Address (I) Register.
    /// The Z80 CPU can be operated in a mode in which an indirect call to any memory location can be achieved
    /// in response to an interrupt. The I register is used for this purpose and stores the high-order eight
    /// bits of the indirect address while the interrupting device provides the lower eight bits of the address.
    /// This feature allows interrupt routines to be dynamically located anywhere in memory with minimal access
    /// time to the routine.
    i: Register,

    /// Memory Refresh (R) Register.
    /// The Z80 CPU contains a memory refresh counter, enabling dynamic memories to be used with the same ease
    /// as static memories. Seven bits of this 8-bit register are automatically incremented after each
    /// instruction fetch. The eighth bit remains as programmed, resulting from an LD R, A instruction.
    /// The data in the refresh counter is sent out on the lower portion of the address bus along with a refresh
    /// control signal while the CPU is decoding and executing the fetched instruction. This mode of refresh is
    /// transparent to the programmer and does not slow the CPU operation. The programmer can load the R register
    /// for testing purposes, but this register is normally not used by the programmer. During refresh, the
    /// contents of the I Register are placed on the upper eight bits of the address bus.
    r: Register,

    /// Accumulator (A) and Flag (F) Registers.
    /// The CPU includes two independent 8-bit Accumulators and associated 8-bit Flag registers.
    /// The Accumulator (A) holds the results of 8-bit arithmetic or logical operations while the Flag Register (F)
    /// indicates specific conditions for 8-bit or 16-bit operations, such as indicating whether or not the
    /// result of an operation is equal to 0. The programmer selects the Accumulator and flag pair with a
    /// single exchange instruction so that it is possible to work with either pair.
    a: Register,

    /// Accumulator and Flag Registers.
    /// The CPU includes two independent 8-bit Accumulators and associated 8-bit Flag registers.
    /// The Accumulator holds the results of 8-bit arithmetic or logical operations while the Flag Register
    /// indicates specific conditions for 8-bit or 16-bit operations, such as indicating whether or not the
    /// result of an operation is equal to 0. The programmer selects the Accumulator and flag pair with a
    /// single exchange instruction so that it is possible to work with either pair.
    f: Register,

    /// Alternate accumulator register
    a_prime: Register,
    /// Alternate flag register
    f_prime: Register,

    // General purpose registers
    b: Register,
    c: Register,
    d: Register,
    e: Register,
    h: Register,
    l: Register,

    // Alternate general purpose registers
    b_prime: Register,
    c_prime: Register,
    d_prime: Register,
    e_prime: Register,
    h_prime: Register,
    l_prime: Register,

    /// Interrupt enable flip flop 1
    iff1: bool,

    /// Interrupt enable flip flop 2
    iff2: bool,
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            pc: 0,
            sp: 0,
            ix: 0,
            iy: 0,
            i: Register::new(),
            r: Register::new(),
            a: Register::new(),
            a_prime: Register::new(),
            f: Register::new(),
            f_prime: Register::new(),
            b: Register::new(),
            c: Register::new(),
            d: Register::new(),
            e: Register::new(),
            h: Register::new(),
            l: Register::new(),
            b_prime: Register::new(),
            c_prime: Register::new(),
            d_prime: Register::new(),
            e_prime: Register::new(),
            h_prime: Register::new(),
            l_prime: Register::new(),
            iff1: true,
            iff2: true,
        }
    }

    fn load_register_pair(high: &mut Register, low: &mut Register, value: u16) {
        high.set_value((value >> 8) as u8);
        low.set_value(value as u8);
    }

    pub fn a(&self) -> u8 {
        self.a.value()
    }

    pub fn set_a(&mut self, value: u8) {
        self.a.set_value(value);
    }

    pub fn f(&self) -> u8 {
        self.f.value()
    }

    pub fn set_f(&mut self, value: u8) {
        self.f.set_value(value);
    }

    pub fn b(&self) -> u8 {
        self.b.value()
    }

    pub fn set_b(&mut self, value: u8) {
        self.b.set_value(value);
    }

    pub fn c(&self) -> u8 {
        self.c.value()
    }

    pub fn set_c(&mut self, value: u8) {
        self.c.set_value(value);
    }

    pub fn d(&self) -> u8 {
        self.d.value()
    }

    pub fn set_d(&mut self, value: u8) {
        self.d.set_value(value);
    }

    pub fn e(&self) -> u8 {
        self.e.value()
    }

    pub fn set_e(&mut self, value: u8) {
        self.e.set_value(value);
    }

    pub fn h(&self) -> u8 {
        self.h.value()
    }

    pub fn set_h(&mut self, value: u8) {
        self.h.set_value(value);
    }

    pub fn l(&self) -> u8 {
        self.l.value()
    }

    pub fn set_l(&mut self, value: u8) {
        self.l.set_value(value);
    }

    pub fn set_bc(&mut self, value: u16) {
        Z80::load_register_pair(&mut self.b, &mut self.c, value);
    }

    pub fn set_de(&mut self, value: u16) {
        Z80::load_register_pair(&mut self.d, &mut self.e, value);
    }

    pub fn set_hl(&mut self, value: u16) {
        Z80::load_register_pair(&mut self.h, &mut self.l, value);
    }

    fn read_register_pair(high: &Register, low: &Register) -> u16 {
        (high.value() as u16) << 8 | low.value() as u16
    }

    pub fn bc(&self) -> u16 {
        Z80::read_register_pair(&self.b, &self.c)
    }

    pub fn de(&self) -> u16 {
        Z80::read_register_pair(&self.d, &self.e)
    }

    pub fn hl(&self) -> u16 {
        Z80::read_register_pair(&self.h, &self.l)
    }

    pub fn fetch_next_opcode(&mut self, mem: &dyn Z80Memory) -> u8 {
        let opcode = mem.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        opcode
    }

    pub fn process_next_instruction(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let mut t_states: u8 = 0;

        // Spend 4 ticks fetching the next instruction
        let opcode = self.fetch_next_opcode(mem);
        t_states += 4;

        let opcode_function = MAIN_FUNCTIONS[opcode as usize];
        t_states += opcode_function(self, mem);

        return t_states;
    }

    fn process_extended_instruction(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let mut t_states: u8 = 0;

        // Spend 4 ticks fetching the next instruction
        let opcode = self.fetch_next_opcode(mem);
        t_states += 4;

        let opcode_function = EXTENDED_INSTRUCTIONS[opcode as usize];
        t_states += opcode_function(self, mem);

        return t_states;
    }

    fn unsupported_operation() -> u8 {
        panic!("Operation is not implemented!");
    }
}

#[cfg(test)]
mod tests {
    use crate::z80::{Z80Memory, Z80};

    pub struct Ram<'a> {
        bytes: &'a mut [u8],
    }

    impl<'a> Ram<'a> {
        pub fn new(bytes: &'a mut [u8]) -> Ram<'a> {
            Ram { bytes }
        }
    }

    impl<'a> Z80Memory for Ram<'a> {
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

        assert_eq!(0, z80.pc);

        assert_eq!(ram.bytes[0], z80.fetch_next_opcode(&ram));

        assert_eq!(1, z80.pc);

        assert_eq!(ram.bytes[1], z80.fetch_next_opcode(&ram));

        assert_eq!(2, z80.pc);

        assert_eq!(ram.bytes[2], z80.fetch_next_opcode(&ram));

        assert_eq!(3, z80.pc);
    }

    #[test]
    fn test_process_next_instruction() {
        let mut bytes = [
            0x26, // LD H, n
            0xDD,
        ];

        let ram = &mut Ram::new(&mut bytes);

        let mut z80 = Z80::new();

        let t_states = z80.process_next_instruction(ram);

        assert_eq!(11, t_states);
        assert_eq!(bytes[1], z80.h.value());
    }
}
