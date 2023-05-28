use super::{
    register_flags::{
        set_h_flag, set_h_flag_with, set_n_flag, set_p_flag_with, set_s_flag_with, set_x_flag,
        set_x_flag_with, set_y_flag_with, set_z_flag_with, unset_h_flag, unset_n_flag,
        S_FLAG_BITMASK,
    },
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
        let temp_d = self.d.value();
        let temp_e = self.e.value();
        let temp_h = self.h.value();
        let temp_l = self.l.value();

        self.d.set_value(temp_h);
        self.e.set_value(temp_l);
        self.h.set_value(temp_d);
        self.l.set_value(temp_e);

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
        let temp_a = self.a.value();
        let temp_f = self.f.value();
        let temp_a_prime = self.a_prime.value();
        let temp_f_prime = self.f_prime.value();

        self.a.set_value(temp_a_prime);
        self.f.set_value(temp_f_prime);
        self.a_prime.set_value(temp_a);
        self.f_prime.set_value(temp_f);

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
        let temp_b = self.b.value();
        let temp_c = self.c.value();
        let temp_d = self.d.value();
        let temp_e = self.e.value();
        let temp_h = self.h.value();
        let temp_l = self.l.value();
        let temp_b_prime = self.b_prime.value();
        let temp_c_prime = self.c_prime.value();
        let temp_d_prime = self.d_prime.value();
        let temp_e_prime = self.e_prime.value();
        let temp_h_prime = self.h_prime.value();
        let temp_l_prime = self.l_prime.value();

        self.b_prime.set_value(temp_b);
        self.c_prime.set_value(temp_c);
        self.d_prime.set_value(temp_d);
        self.e_prime.set_value(temp_e);
        self.h_prime.set_value(temp_h);
        self.l_prime.set_value(temp_l);
        self.b.set_value(temp_b_prime);
        self.c.set_value(temp_c_prime);
        self.d.set_value(temp_d_prime);
        self.e.set_value(temp_e_prime);
        self.h.set_value(temp_h_prime);
        self.l.set_value(temp_l_prime);

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
        mem.write(self.stack_pointer, self.l.value());
        self.l.set_value(mem_sp);

        let mem_sp = mem.read(self.stack_pointer + 1);
        mem.write(self.stack_pointer + 1, self.h.value());
        self.h.set_value(mem_sp);

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
        let mem_sph = mem.read(self.stack_pointer.wrapping_add(1));

        mem.write(self.stack_pointer, ixl);
        mem.write(self.stack_pointer.wrapping_add(1), ixh);

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
        let mem_sph = mem.read(self.stack_pointer.wrapping_add(1));

        mem.write(self.stack_pointer, iyl);
        mem.write(self.stack_pointer.wrapping_add(1), iyh);

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
        self.set_hl(hl.wrapping_add(1));
        self.set_de(de.wrapping_add(1));

        // Read and decrement BC
        let bc = self.bc().wrapping_sub(1);
        self.set_bc(bc);

        unset_h_flag(&mut self.f);
        set_p_flag_with(&mut self.f, bc != 0);
        unset_n_flag(&mut self.f);

        // Extra behaviour from http://www.z80.info/zip/z80-documented.pdf p.16
        let n = self.a.value().wrapping_add(data);
        set_y_flag_with(&mut self.f, n & 0b00000010 != 0);
        set_x_flag_with(&mut self.f, n & 0b00001000 != 0);

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
        self.set_hl(hl.wrapping_add(1));
        self.set_de(de.wrapping_add(1));

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
        set_p_flag_with(&mut self.f, bc != 0);
        unset_n_flag(&mut self.f);

        // Extra behaviour from http://www.z80.info/zip/z80-documented.pdf p.16
        let n = self.a.value().wrapping_add(data);
        set_y_flag_with(&mut self.f, n & 0b00000010 != 0);
        set_x_flag_with(&mut self.f, n & 0b00001000 != 0);

        t_states
    }

    /// ## LDD
    /// ### Operation
    /// (DE) ← (HL), DE ← DE - 1, HL ← HL - 1, BC ← BC – 1
    /// ### Op Code
    /// LDD
    /// ### Operands
    /// None
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `1 0 1 0 1 0 0 0` (A8)
    /// ### Description
    /// This 2-byte instruction transfers a byte of data from the memory
    /// location addressed by the contents of the HL register pair to the memory
    /// location addressed by the contents of the DE register pair. Then both of
    /// these register pairs including the Byte Counter (BC) Register pair are
    /// decremented.
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
    /// instruction LDD results in the following contents in register pairs and
    /// memory addresses:
    ///
    /// |         |          |       |
    /// | ------- | -------- | ----- |
    /// | HL      | contains | 1110h |
    /// | (1111h) | contains | 88h   |
    /// | DE      | contains | 2221h |
    /// | (2222h) | contains | 88h   |
    /// | BC      | contains | 6h    |
    pub fn ldd(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        // Transfer data from HL address to DE address
        let hl = self.hl();
        let data = mem.read(hl);
        let de = self.de();
        mem.write(de, data);

        // Increment HL and DE
        self.set_hl(hl.wrapping_sub(1));
        self.set_de(de.wrapping_sub(1));

        // Read and decrement BC
        let bc = self.bc().wrapping_sub(1);
        self.set_bc(bc);

        unset_h_flag(&mut self.f);
        set_p_flag_with(&mut self.f, bc != 0);
        unset_n_flag(&mut self.f);

        // Extra behaviour from http://www.z80.info/zip/z80-documented.pdf p.16
        let n = self.a.value().wrapping_add(data);
        set_y_flag_with(&mut self.f, n & 0b00000010 != 0);
        set_x_flag_with(&mut self.f, n & 0b00001000 != 0);

        // T states
        16
    }

    /// ## LDDR
    /// ### Operation
    /// repeat {(DE) ← (HL), DE ← DE - 1, HL ← HL - 1, BC ← BC – 1} while (BC ≠ 0)
    /// ### Op Code
    /// LDDR
    /// ### Operand
    /// None
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `1 0 1 1 1 0 0 0` (B0)
    /// ### Description
    /// This 2-byte instruction transfers a byte of data from the memory
    /// location addressed by the contents of the HL register pair to the memory
    /// location addressed by the contents of the DE register pair. Then both of
    /// these registers, and the BC (Byte Counter), are decremented. If
    /// decrementing causes BC to go to 0, the instruction is terminated. If BC
    /// is not 0, the program counter is decremented by two and the instruction
    /// is repeated. Interrupts are recognized and two refresh cycles execute
    /// after each data transfer.
    ///
    /// When the BC is set to 0 prior to instruction execution, the instruction
    /// loops through 64 KB.
    ///
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
    /// The HL register pair contains 1114h, the DE register pair contains
    /// 2225h, the BC register pair contains 0003h, and memory locations contain
    /// the following data.
    ///
    /// |         |          |     |         |          |     |
    /// | ------- | -------- | --- | ------- | -------- | --- |
    /// | (1114h) | contains | A5h | (2225h) | contains | C5h |
    /// | (1113h) | contains | 36h | (2224h) | contains | 59h |
    /// | (1112h) | contains | 88h | (2223h) | contains | 66h |
    ///
    /// Upon the execution of an LDDR instruction, the contents of register
    /// pairs and memory locations now contain:
    ///
    /// |         |          |       |         |          |     |
    /// | ------- | -------- | ----- | ------- | -------- | --- |
    /// | HL      | contains | 1111h |         |          |     |
    /// | DE      | contains | 2222h |         |          |     |
    /// | BC      | contains | 0000h |         |          |     |
    /// | (1114h) | contains | A5h   | (2225h) | contains | A5h |
    /// | (1113h) | contains | 36h   | (2224h) | contains | 36h |
    /// | (1112h) | contains | 88h   | (2223h) | contains | 88h |
    pub fn lddr(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        // Transfer data from HL address to DE address
        let hl = self.hl();
        let data = mem.read(hl);
        let de = self.de();
        mem.write(de, data);

        // Increment HL and DE
        self.set_hl(hl.wrapping_sub(1));
        self.set_de(de.wrapping_sub(1));

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
        set_p_flag_with(&mut self.f, bc != 0);
        unset_n_flag(&mut self.f);

        // Extra behaviour from http://www.z80.info/zip/z80-documented.pdf p.16
        let n = self.a.value().wrapping_add(data);
        set_y_flag_with(&mut self.f, n & 0b00000010 != 0);
        set_x_flag_with(&mut self.f, n & 0b00001000 != 0);

        t_states
    }

    /// ## CPI
    /// ### Operation
    /// A – (HL), HL ← HL +1, BC ← BC – 1
    /// ### Op Code
    /// CPI
    /// ### Operands
    /// None.
    /// ### Description
    /// The contents of the memory location addressed by the HL register is
    /// compared with the contents of the Accumulator. With a true compare, a
    /// condition bit is set. Then HL is incremented and the Byte Counter
    /// (register pair BC) is decremented.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 16 (4, 4, 3, 5) | 4.00       |
    ///
    /// ### Condition Bits Affected
    /// * S is set if result is negative; otherwise, it is reset.
    /// * Z is set if A is (HL); otherwise, it is reset.
    /// * H is set if borrow from bit 4; otherwise, it is reset.
    /// * P/V is set if BC – 1 is not 0; otherwise, it is reset.
    /// * N is set.
    /// * C is not affected.
    /// ### Example
    /// If the HL register pair contains 1111h, memory location 1111h contains
    /// 3Bh, the Accumulator contains 3Bh, and the Byte Counter contains 0001h.
    /// Upon the execution of a CPI instruction, the Byte Counter contains
    /// 0000h, the HL register pair contains 1112h, the Z flag in the F register
    /// is set, and the P/V flag in the F Register is reset. There is no effect
    /// on the contents of the Accumulator or to address 1111h.
    pub fn cpi(&mut self, mem: &dyn Z80Memory) -> u8 {
        let address = self.hl();
        let data = mem.read(address);

        let a = self.a.value();
        let n = a.wrapping_sub(data);

        let hl = address.wrapping_add(1);
        self.set_hl(hl);

        let bc = self.bc().wrapping_sub(1);
        self.set_bc(bc);

        let sign_flag = n & S_FLAG_BITMASK == S_FLAG_BITMASK;
        set_s_flag_with(&mut self.f, sign_flag);
        set_z_flag_with(&mut self.f, n == 0);
        let half_carry_borrow = (a & 0x10) != (n & 0x10);
        set_h_flag_with(&mut self.f, half_carry_borrow);
        set_p_flag_with(&mut self.f, bc != 0);
        set_n_flag(&mut self.f);

        // Extra behaviour from http://www.z80.info/zip/z80-documented.pdf p.16
        let y_flag = n & 0b00000010 == 0b00000010;
        set_y_flag_with(&mut self.f, y_flag);
        let x_flag = n & 0b00001000 == 0b00001000;
        set_x_flag_with(&mut self.f, x_flag);

        // T states
        16
    }
}

mod tests {
    use crate::z80::{
        register_flags::{h_flag, n_flag, p_flag, s_flag, z_flag},
        tests::Ram,
    };

    use super::*;

    #[test]
    fn test_ex_de_hl() {
        let mut z80 = Z80::new();
        z80.set_de(0x2822);
        z80.set_hl(0x499A);

        let t_states = z80.ex_de_hl();

        assert_eq!(4, t_states);
        assert_eq!(0x499A, z80.de());
        assert_eq!(0x2822, z80.hl());
    }

    #[test]
    fn test_ex_af_afp() {
        let mut z80 = Z80::new();
        z80.a.set_value(0x99);
        z80.f.set_value(0x00);
        z80.a_prime.set_value(0x59);
        z80.f_prime.set_value(0x44);

        let t_states = z80.ex_af_afp();

        assert_eq!(4, t_states);

        assert_eq!(0x59, z80.a.value());
        assert_eq!(0x44, z80.f.value());
        assert_eq!(0x99, z80.a_prime.value());
        assert_eq!(0x00, z80.f_prime.value())
    }

    #[test]
    fn test_exx() {
        let mut z80 = Z80::new();
        z80.b.set_value(0x44);
        z80.c.set_value(0x5A);
        z80.d.set_value(0x3D);
        z80.e.set_value(0xA2);
        z80.h.set_value(0x88);
        z80.l.set_value(0x59);

        z80.b_prime.set_value(0x09);
        z80.c_prime.set_value(0x88);
        z80.d_prime.set_value(0x93);
        z80.e_prime.set_value(0x00);
        z80.h_prime.set_value(0x00);
        z80.l_prime.set_value(0xE7);

        let t_states = z80.exx();

        assert_eq!(4, t_states);

        assert_eq!(0x09, z80.b.value());
        assert_eq!(0x88, z80.c.value());
        assert_eq!(0x93, z80.d.value());
        assert_eq!(0x00, z80.e.value());
        assert_eq!(0x00, z80.h.value());
        assert_eq!(0xE7, z80.l.value());

        assert_eq!(0x44, z80.b_prime.value());
        assert_eq!(0x5A, z80.c_prime.value());
        assert_eq!(0x3D, z80.d_prime.value());
        assert_eq!(0xA2, z80.e_prime.value());
        assert_eq!(0x88, z80.h_prime.value());
        assert_eq!(0x59, z80.l_prime.value());
    }

    #[test]
    fn test_ex_mem_sp_hl() {
        let bytes = &mut [0xE3, 0x11, 0x22];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.stack_pointer = 1;
        z80.h.set_value(0x70);
        z80.l.set_value(0x12);

        let t_states = z80.ex_mem_sp_hl(&mut ram);
        assert_eq!(19, t_states);

        assert_eq!(0x22, z80.h.value());
        assert_eq!(0x11, z80.l.value());
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
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0x00FF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }

    #[test]
    fn test_ldi_bc_result_0() {
        let bytes = &mut [0xED, 0xA0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0001);

        let t_states = z80.ldi(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0x0000, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(!p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
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
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0xFFFF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
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
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0x00FF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
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
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0x0000, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(!p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
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
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0xFFFF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }

    #[test]
    fn test_ldd() {
        let bytes = &mut [0xED, 0xA0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0100);

        let t_states = z80.ldd(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0001, z80.hl());
        assert_eq!(0x0002, z80.de());
        assert_eq!(0x00FF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }

    #[test]
    fn test_ldd_bc_result_0() {
        let bytes = &mut [0xED, 0xA0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0001);

        let t_states = z80.ldd(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0001, z80.hl());
        assert_eq!(0x0002, z80.de());
        assert_eq!(0x0000, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(!p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }

    #[test]
    fn test_ldd_bc_result_ffff() {
        let bytes = &mut [0xED, 0xA0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0000);

        let t_states = z80.ldd(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0001, z80.hl());
        assert_eq!(0x0002, z80.de());
        assert_eq!(0xFFFF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }

    #[test]
    fn test_lddr() {
        let bytes = &mut [0xED, 0xB0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0100);

        let t_states = z80.lddr(&mut ram);
        assert_eq!(21, t_states);

        assert_eq!(0, z80.program_counter);
        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0001, z80.hl());
        assert_eq!(0x0002, z80.de());
        assert_eq!(0x00FF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }
    #[test]
    fn test_lddr_bc_result_0() {
        let bytes = &mut [0xED, 0xB0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0001);

        let t_states = z80.lddr(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(2, z80.program_counter);
        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0001, z80.hl());
        assert_eq!(0x0002, z80.de());
        assert_eq!(0x0000, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(!p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }

    #[test]
    fn test_lddr_bc_result_ffff() {
        let bytes = &mut [0xED, 0xB0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0000);

        let t_states = z80.lddr(&mut ram);
        assert_eq!(21, t_states);

        assert_eq!(0, z80.program_counter);
        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0001, z80.hl());
        assert_eq!(0x0002, z80.de());
        assert_eq!(0xFFFF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }

    #[test]
    fn test_cpi_true_compare_with_bc_zero() {
        let mut bytes = [0xED, 0xA1, 0x80];
        let mem = Ram::new(&mut bytes);
        let mut z80 = Z80::new();
        z80.set_hl(0x0002);
        z80.set_bc(0x0001);
        z80.set_a(0x80);

        let t_states = z80.cpi(&mem);

        assert_eq!(16, t_states);

        assert_eq!(false, s_flag(&z80.f));
        assert_eq!(true, z_flag(&z80.f));
        assert_eq!(false, h_flag(&z80.f));
        assert_eq!(false, p_flag(&z80.f));
        assert_eq!(true, n_flag(&z80.f));
    }

    #[test]
    fn test_cpi_positive_compare_with_half_carry_borrow_and_bc_not_zero() {
        let mut bytes = [0xED, 0xA1, 0x01];
        let mem = Ram::new(&mut bytes);
        let mut z80 = Z80::new();
        z80.set_hl(0x0002);
        z80.set_bc(0x0002);
        z80.set_a(0x10);

        let t_states = z80.cpi(&mem);

        assert_eq!(16, t_states);

        assert_eq!(false, s_flag(&z80.f));
        assert_eq!(false, z_flag(&z80.f));
        assert_eq!(true, h_flag(&z80.f));
        assert_eq!(true, p_flag(&z80.f));
        assert_eq!(true, n_flag(&z80.f));
    }

    #[test]
    fn test_cpi_negative_compare() {
        let mut bytes = [0xED, 0xA1, 0x10];
        let mem = Ram::new(&mut bytes);
        let mut z80 = Z80::new();
        z80.set_hl(0x0002);
        z80.set_bc(0x0002);
        z80.set_a(0x08);

        let t_states = z80.cpi(&mem);

        assert_eq!(16, t_states);

        assert_eq!(true, s_flag(&z80.f));
        assert_eq!(false, z_flag(&z80.f));
        assert_eq!(true, h_flag(&z80.f));
        assert_eq!(true, p_flag(&z80.f));
        assert_eq!(true, n_flag(&z80.f));
    }
}
