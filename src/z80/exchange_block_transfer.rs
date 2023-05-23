use super::{
    flag_register::{set_p_flag_with, unset_h_flag, unset_n_flag},
    Z80Memory, Z80,
};

impl Z80 {
    // Exchange, Block Trabsfer, and Search Group

    /// ## EX DE, HL
    /// ### Operation
    /// DE ↔ HL
    /// ### Op Code
    /// EX
    /// ### Operands
    /// DE, HL
    /// `1 1 1 0 1 0 1 1` (EB)
    /// ### Description
    /// The 2-byte contents of register pairs DE and HL are exchanged.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 1        | 4        | 1.00       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If register pair DE contains 2822h and register pair HL contains 499Ah,
    /// then upon the execution of an EX DE, HL instruction, register pair DE
    /// contains 499Ah and register pair HL contains 2822h.
    pub fn ex_de_hl(&mut self) -> u8 {
        (self.d, self.e, self.h, self.l) = (self.h, self.l, self.d, self.e);

        // T states
        4
    }

    /// ## EX AF, AF′
    /// ### Operation
    /// AF ↔ AF'
    /// ### Op Code
    /// EX
    /// ### Operands
    /// AF, AF′
    /// `0 0 0 0 1 0 0 0` (08)
    /// ### Description
    /// The 2-byte contents of the register pairs AF and AF' are exchanged.
    /// Register pair AF consists of registers A′ and F′.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 1        | 4        | 1.00       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If register pair AF contains 9900h and register pair AF′ contains 5944h,
    /// the contents of AF are 5944h and the contents of AF′ are 9900h upon
    /// execution of the EX AF, AF′ instruction.
    pub fn ex_af_afp(&mut self) -> u8 {
        (self.a, self.f, self.a_prime, self.f_prime) = (self.a_prime, self.f_prime, self.a, self.f);

        // T states
        4
    }

    /// ## EXX
    /// ### Operation
    /// (BC) ↔ (BC′), (DE) ↔ (DE'), (HL) ↔ (HL′)
    /// ### Op Code
    /// EXX
    /// ### Operands
    /// None.
    /// `1 1 0 1 1 0 0 1` (D9)
    /// ### Description
    /// Each 2-byte value in register pairs BC, DE, and HL is exchanged with the
    /// 2-byte value in BC', DE', and HL', respectively.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 1        | 4        | 1.00       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If register pairs BC, DE, and HL contain 445Ah, 3DA2h, and 8859h,
    /// respectively, and register pairs BC’, DE’, and HL’ contain 0988h, 9300h,
    /// and 00E7h, respectively, then upon the execution of an EXX instruction,
    /// BC contains 0988h; DE contains 9300h; HL contains 00E7h; BC’ contains
    /// 445Ah; DE’ contains 3DA2h; and HL’ contains 8859h.
    pub fn exx(&mut self) -> u8 {
        (
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.b_prime,
            self.c_prime,
            self.d_prime,
            self.e_prime,
            self.h_prime,
            self.l_prime,
        ) = (
            self.b_prime,
            self.c_prime,
            self.d_prime,
            self.e_prime,
            self.h_prime,
            self.l_prime,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
        );

        // T states
        4
    }

    /// ## EX (SP), HL
    /// ### Operation
    /// H ↔ (SP+1), L ↔ (SP)
    /// ### Op Code
    /// EX
    /// ### Operands
    /// (SP), HL
    /// `1 1 1 0 0 0 1 1` (E3)
    /// ### Description
    /// The low-order byte contained in register pair HL is exchanged with the
    /// contents of the memory address specified by the contents of register
    /// pair SP (Stack Pointer), and the high-order byte of HL is exchanged with
    /// the next highest memory address (SP+1).
    ///
    /// | M Cycles | T States           | 4 MHz E.T. |
    /// | -------- | ------------------ | ---------- |
    /// | 5        | 19 (4, 3, 4, 3, 5) | 4.75       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If the HL register pair contains 7012h, the SP register pair contains
    /// 8856h, the memory location 8856h contains byte 11h, and memory location
    /// 8857h contains byte 22h, then the instruction EX (SP), HL results in the
    /// HL register pair containing number 2211h, memory location 8856h
    /// containing byte 12h, memory location 8857h containing byte 70h and Stack
    /// Pointer containing 8856h.
    pub fn ex_mem_sp_hl(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let mem_sp = mem.read(self.stack_pointer);
        mem.write(self.stack_pointer, self.l);
        self.l = mem_sp;

        let mem_sp = mem.read(self.stack_pointer + 1);
        mem.write(self.stack_pointer + 1, self.h);
        self.h = mem_sp;

        // T states
        19
    }

    /// ## EX (SP), IX
    /// ### Operation
    /// IXH ↔ (SP+1), IXL ↔ (SP)
    /// ### Op Code
    /// EX
    /// ### Operands
    /// (SP), IX
    /// `1 1 0 1 1 1 0 1` (DD)
    /// `1 1 1 0 0 0 1 1` (E3)
    /// ### Description
    /// The low-order byte contained in register IX is exchanged with the
    /// contents of the memory address specified by the contents of register
    /// pair SP (Stack Pointer), and the high-order byte of IX is exchanged with
    /// the next highest memory address (SP+1).
    ///
    /// | M Cycles | T States              | 4 MHz E.T. |
    /// | -------- | --------------------- | ---------- |
    /// | 6        | 23 (4, 4, 3, 4, 3, 5) | 5.75       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If Index Register IX contains 3988h, the SP register pair Contains
    /// 0100h, memory location 0100h contains byte 90h, and memory location
    /// 0101h contains byte 48h, then the instruction EX (SP), IX results in the
    /// IX register pair containing number 4890h, memory location 0100h
    /// containing 88h, memory location 0101h containing 39h, and the Stack
    /// Pointer containing 0100h.
    pub fn ex_mem_sp_ix(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let ixl = self.ix as u8;
        let ixh = (self.ix >> 8) as u8;

        let mem_spl = mem.read(self.stack_pointer);
        let mem_sph = mem.read(self.stack_pointer + 1);

        mem.write(self.stack_pointer, ixl);
        mem.write(self.stack_pointer + 1, ixh);

        self.ix = (mem_sph as u16) << 8 | mem_spl as u16;

        // T states
        23
    }

    /// ## EX (SP), IY
    /// ### Operation
    /// IYH ↔ (SP+1), IYL ↔ (SP)
    /// ### Op Code
    /// EX
    /// ### Operands
    /// (SP), IY
    /// `1 1 1 1 1 1 0 1` (FD)
    /// `1 1 1 0 0 0 1 1` (E3)
    /// ### Description
    /// The low-order byte contained in register IY is exchanged with the
    /// contents of the memory address specified by the contents of register
    /// pair SP (Stack Pointer), and the high-order byte of IY is exchanged with
    /// the next highest memory address (SP+1).
    ///
    /// | M Cycles | T States              | 4 MHz E.T. |
    /// | -------- | --------------------- | ---------- |
    /// | 6        | 23 (4, 4, 3, 4, 3, 5) | 5.75       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If Index Register IY contains 3988h, the SP register pair Contains
    /// 0100h, memory location 0100h contains byte 90h, and memory location
    /// 0101h contains byte 48h, then the instruction EX (SP), IY results in the
    /// IY register pair containing number 4890h, memory location 0100h
    /// containing 88h, memory location 0101h containing 39h, and the Stack
    /// Pointer containing 0100h.
    pub fn ex_mem_sp_iy(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let iyl = self.iy as u8;
        let iyh = (self.iy >> 8) as u8;

        let mem_spl = mem.read(self.stack_pointer);
        let mem_sph = mem.read(self.stack_pointer + 1);

        mem.write(self.stack_pointer, iyl);
        mem.write(self.stack_pointer + 1, iyh);

        self.iy = (mem_sph as u16) << 8 | mem_spl as u16;

        // T states
        23
    }

    /// ## LDI
    /// ### Operation
    /// (DE) ← (HL), DE ← DE + 1, HL ← HL + 1, BC ← BC – 1
    /// ### Op Code
    /// LDI
    /// ### Operands
    /// None
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `1 0 1 0 0 0 0 0` (A0)
    /// ### Description
    /// A byte of data is transferred from the memory location addressed, by the
    /// contents of the HL register pair to the memory location addressed by the
    /// contents of the DE register pair. Then both these register pairs are
    /// incremented and the Byte Counter (BC) Register pair is decremented.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 16 (4, 4, 3, 5) | 4.00       |
    ///
    /// ### Condition Bits Affected
    /// * S is not affected.
    /// * Z is not affected.
    /// * H is reset.
    /// * P/V is set if BC – 1 ≠ 0; otherwise, it is reset.
    /// * N is reset.
    /// * C is not affected.
    /// ### Example
    /// If the HL register pair contains 1111h, memory location 1111h contains
    /// byte 88h, the DE register pair contains 2222h, the memory location 2222h
    /// contains byte 66h, and the BC register pair contains 7h, then the
    /// instruction LDI results in the following contents in register pairs and
    /// memory addresses:
    ///
    /// |         |          |       |
    /// | ------- | -------- | ----- |
    /// | HL      | contains | 1112h |
    /// | (1111h) | contains | 88h   |
    /// | DE      | contains | 2223h |
    /// | (2222h) | contains | 88h   |
    /// | BC      | contains | 6H    |
    pub fn ldi(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        // Transfer data from HL address to DE address
        let hl = self.hl();
        let data = mem.read(hl);
        let de = self.de();
        mem.write(de, data);

        // Increment HL and DE
        self.set_hl(hl + 1);
        self.set_de(de + 1);

        // Read and decrement BC
        let bc = self.bc().wrapping_sub(1);
        self.set_bc(bc);

        unset_h_flag(&mut self.f);
        set_p_flag_with(&mut self.f, bc.wrapping_sub(1) != 0);
        unset_n_flag(&mut self.f);

        // T states
        16
    }

    /// ## LDIR
    /// ### Operation
    /// repeat {(DE) ← (HL), DE ← DE + 1, HL ← HL + 1, BC ← BC – 1} while (BC ≠ 0)
    /// ### Op Code
    /// LDIR
    /// ### Operand
    /// None
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `1 0 1 1 0 0 0 0` (B0)
    /// ### Description
    /// This 2-byte instruction transfers a byte of data from the memory
    /// location addressed by the contents of the HL register pair to the memory
    /// location addressed by the DE register pair. Both these register pairs
    /// are incremented and the Byte Counter (BC) Register pair is decremented.
    /// If decrementing allows the BC to go to 0, the instruction is terminated.
    /// If BC is not 0, the program counter is decremented by two and the
    /// instruction is repeated. Interrupts are recognized and two refresh
    /// cycles are executed after each data transfer. When the BC is set to 0
    /// prior to instruction execution, the instruction loops through 64 KB.
    /// For BC ≠ 0:
    ///
    /// | M Cycles | T States           | 4 MHz E.T. |
    /// | -------- | ------------------ | ---------- |
    /// | 5        | 21 (4, 4, 3, 5, 5) | 5.25       |
    ///
    /// For BC = 0:
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 16 (4, 4, 3, 5) | 4.00       |
    ///
    /// ### Condition Bits Affected
    /// * S is not affected.
    /// * Z is not affected.
    /// * H is reset.
    /// * P/V is set if BC – 1 ≠ 0; otherwise, it is reset.
    /// * N is reset.
    /// * C is not affected.
    /// ### Example
    /// The HL register pair contains 11111h, the DE register pair contains
    /// 2222h, the BC register pair contains 0003h, and memory locations contain
    /// the following data.
    ///
    /// |         |          |     |         |          |     |
    /// | ------- | -------- | --- | ------- | -------- | --- |
    /// | (1111h) | contains | 88h | (2222h) | contains | 66h |
    /// | (1112h) | contains | 36h | (2223h) | contains | 59h |
    /// | (1113h) | contains | A5h | (2224h) | contains | C5h |
    ///
    /// Upon the execution of an LDIR instruction, the contents of register
    /// pairs and memory locations now contain:
    ///
    /// |         |          |       |         |          |     |
    /// | ------- | -------- | ----- | ------- | -------- | --- |
    /// | HL      | contains | 1114h |         |          |     |
    /// | DE      | contains | 2225h |         |          |     |
    /// | BC      | contains | 0000h |         |          |     |
    /// | (1111h) | contains | 88h   | (2222h) | contains | 88h |
    /// | (1112h) | contains | 36h   | (2223h) | contains | 36h |
    /// | (1113h) | contains | A5h   | (2224h) | contains | A5h |
    pub fn ldir(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        // Transfer data from HL address to DE address
        let hl = self.hl();
        let data = mem.read(hl);
        let de = self.de();
        mem.write(de, data);

        // Increment HL and DE
        self.set_hl(hl + 1);
        self.set_de(de + 1);

        // Read and decrement BC
        let bc = self.bc().wrapping_sub(1);
        self.set_bc(bc);

        let t_states;

        if bc != 0 {
            self.program_counter -= 2;
            t_states = 21;
        } else {
            t_states = 16;
        }

        unset_h_flag(&mut self.f);
        set_p_flag_with(&mut self.f, bc.wrapping_sub(1) != 0);
        unset_n_flag(&mut self.f);

        t_states
    }
}

mod tests {
    use crate::z80::{
        flag_register::{h_flag_set, n_flag_set, p_flag_set},
        tests::Ram,
    };

    use super::*;

    #[test]
    fn test_ex_de_hl() {
        let mut z80 = Z80::new();
        z80.d = 0x28;
        z80.e = 0x22;
        z80.h = 0x49;
        z80.l = 0x9A;

        let t_states = z80.ex_de_hl();

        assert_eq!(4, t_states);
        assert_eq!(0x49, z80.d);
        assert_eq!(0x9A, z80.e);
        assert_eq!(0x28, z80.h);
        assert_eq!(0x22, z80.l)
    }

    #[test]
    fn test_ex_af_afp() {
        let mut z80 = Z80::new();
        z80.a = 0x99;
        z80.f = 0x00;
        z80.a_prime = 0x59;
        z80.f_prime = 0x44;

        let t_states = z80.ex_af_afp();

        assert_eq!(4, t_states);

        assert_eq!(0x59, z80.a);
        assert_eq!(0x44, z80.f);
        assert_eq!(0x99, z80.a_prime);
        assert_eq!(0x00, z80.f_prime)
    }

    #[test]
    fn test_exx() {
        let mut z80 = Z80::new();
        z80.b = 0x44;
        z80.c = 0x5A;
        z80.d = 0x3D;
        z80.e = 0xA2;
        z80.h = 0x88;
        z80.l = 0x59;

        z80.b_prime = 0x09;
        z80.c_prime = 0x88;
        z80.d_prime = 0x93;
        z80.e_prime = 0x00;
        z80.h_prime = 0x00;
        z80.l_prime = 0xE7;

        let t_states = z80.exx();

        assert_eq!(4, t_states);

        assert_eq!(0x09, z80.b);
        assert_eq!(0x88, z80.c);
        assert_eq!(0x93, z80.d);
        assert_eq!(0x00, z80.e);
        assert_eq!(0x00, z80.h);
        assert_eq!(0xE7, z80.l);

        assert_eq!(0x44, z80.b_prime);
        assert_eq!(0x5A, z80.c_prime);
        assert_eq!(0x3D, z80.d_prime);
        assert_eq!(0xA2, z80.e_prime);
        assert_eq!(0x88, z80.h_prime);
        assert_eq!(0x59, z80.l_prime);
    }

    #[test]
    fn test_ex_mem_sp_hl() {
        let bytes = &mut [0xE3, 0x11, 0x22];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.stack_pointer = 1;
        z80.h = 0x70;
        z80.l = 0x12;

        let t_states = z80.ex_mem_sp_hl(&mut ram);
        assert_eq!(19, t_states);

        assert_eq!(0x22, z80.h);
        assert_eq!(0x11, z80.l);
        assert_eq!(0x12, ram.read(1));
        assert_eq!(0x70, ram.read(2));
    }

    #[test]
    fn test_ex_mem_sp_ix() {
        let bytes = &mut [0xDD, 0xE3, 0x90, 0x48];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.stack_pointer = 2;
        z80.ix = 0x3988;

        let t_states = z80.ex_mem_sp_ix(&mut ram);
        assert_eq!(23, t_states);

        assert_eq!(0x4890, z80.ix);
        assert_eq!(0x88, ram.read(2));
        assert_eq!(0x39, ram.read(3));
    }

    #[test]
    fn test_ex_mem_sp_iy() {
        let bytes = &mut [0xDD, 0xE3, 0x90, 0x48];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.stack_pointer = 2;
        z80.iy = 0x3988;

        let t_states = z80.ex_mem_sp_iy(&mut ram);
        assert_eq!(23, t_states);

        assert_eq!(0x4890, z80.iy);
        assert_eq!(0x88, ram.read(2));
        assert_eq!(0x39, ram.read(3));
    }

    #[test]
    fn test_ldi() {
        let bytes = &mut [0xED, 0xA0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0100);

        let t_states = z80.ldi(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x00FF, z80.bc());

        assert!(!h_flag_set(&z80.f));
        assert!(p_flag_set(&z80.f));
        assert!(!n_flag_set(&z80.f));
    }

    #[test]
    fn test_ldi_bc_result_1() {
        let bytes = &mut [0xED, 0xA0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0002);

        let t_states = z80.ldi(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0001, z80.bc());

        assert!(!h_flag_set(&z80.f));
        assert!(!p_flag_set(&z80.f));
        assert!(!n_flag_set(&z80.f));
    }

    #[test]
    fn test_ldi_bc_result_ffff() {
        let bytes = &mut [0xED, 0xA0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0000);

        let t_states = z80.ldi(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0xFFFF, z80.bc());

        assert!(!h_flag_set(&z80.f));
        assert!(p_flag_set(&z80.f));
        assert!(!n_flag_set(&z80.f));
    }

    #[test]
    fn test_ldir() {
        let bytes = &mut [0xED, 0xB0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0100);

        let t_states = z80.ldir(&mut ram);
        assert_eq!(21, t_states);

        assert_eq!(0, z80.program_counter);
        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x00FF, z80.bc());

        assert!(!h_flag_set(&z80.f));
        assert!(p_flag_set(&z80.f));
        assert!(!n_flag_set(&z80.f));
    }

    #[test]
    fn test_ldir_bc_result_1() {
        let bytes = &mut [0xED, 0xB0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0002);

        let t_states = z80.ldir(&mut ram);
        assert_eq!(21, t_states);

        assert_eq!(0, z80.program_counter);
        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0001, z80.bc());

        assert!(!h_flag_set(&z80.f));
        assert!(!p_flag_set(&z80.f));
        assert!(!n_flag_set(&z80.f));
    }

    #[test]
    fn test_ldir_bc_result_0() {
        let bytes = &mut [0xED, 0xB0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0001);

        let t_states = z80.ldir(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(2, z80.program_counter);
        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0000, z80.bc());

        assert!(!h_flag_set(&z80.f));
        assert!(p_flag_set(&z80.f));
        assert!(!n_flag_set(&z80.f));
    }

    #[test]
    fn test_ldir_bc_result_ffff() {
        let bytes = &mut [0xED, 0xB0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0000);

        let t_states = z80.ldir(&mut ram);
        assert_eq!(21, t_states);

        assert_eq!(0, z80.program_counter);
        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0xFFFF, z80.bc());

        assert!(!h_flag_set(&z80.f));
        assert!(p_flag_set(&z80.f));
        assert!(!n_flag_set(&z80.f));
    }
}
