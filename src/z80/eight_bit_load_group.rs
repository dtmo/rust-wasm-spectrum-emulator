use super::{
    register_flags::{
        s_flag_set, set_p_flag_with, set_s_flag_with, set_z_flag_with, unset_h_flag, unset_n_flag,
    },
    Register, Z80Memory, Z80,
};

impl Z80 {
    // 8-Bit Load Group

    /// ## LD r, r'
    ///
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
    fn ld_r_rp(r: &mut Register, r_prime: u8) -> u8 {
        r.set_value(r_prime);

        // T states
        4
    }

    pub fn ld_b_b(&mut self) -> u8 {
        let b = self.b.value();
        Z80::ld_r_rp(&mut self.b, b)
    }

    pub fn ld_b_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.c.value())
    }

    pub fn ld_b_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.d.value())
    }

    pub fn ld_b_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.e.value())
    }

    pub fn ld_b_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.h.value())
    }

    pub fn ld_b_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.l.value())
    }

    pub fn ld_b_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.a.value())
    }

    pub fn ld_c_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.b.value())
    }

    pub fn ld_c_c(&mut self) -> u8 {
        let c = self.c.value();
        Z80::ld_r_rp(&mut self.c, c)
    }

    pub fn ld_c_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.d.value())
    }

    pub fn ld_c_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.e.value())
    }

    pub fn ld_c_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.h.value())
    }

    pub fn ld_c_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.l.value())
    }

    pub fn ld_c_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.a.value())
    }

    pub fn ld_d_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.b.value())
    }

    pub fn ld_d_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.c.value())
    }

    pub fn ld_d_d(&mut self) -> u8 {
        let d = self.d.value();
        Z80::ld_r_rp(&mut self.d, d)
    }

    pub fn ld_d_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.e.value())
    }

    pub fn ld_d_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.h.value())
    }

    pub fn ld_d_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.l.value())
    }

    pub fn ld_d_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.a.value())
    }

    pub fn ld_e_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.b.value())
    }

    pub fn ld_e_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.c.value())
    }

    pub fn ld_e_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.d.value())
    }

    pub fn ld_e_e(&mut self) -> u8 {
        let e = self.e.value();
        Z80::ld_r_rp(&mut self.e, e)
    }

    pub fn ld_e_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.h.value())
    }

    pub fn ld_e_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.l.value())
    }

    pub fn ld_e_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.a.value())
    }

    pub fn ld_h_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.b.value())
    }

    pub fn ld_h_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.c.value())
    }

    pub fn ld_h_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.d.value())
    }

    pub fn ld_h_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.e.value())
    }

    pub fn ld_h_h(&mut self) -> u8 {
        let h = self.h.value();
        Z80::ld_r_rp(&mut self.h, h)
    }

    pub fn ld_h_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.l.value())
    }

    pub fn ld_h_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.a.value())
    }

    pub fn ld_l_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.b.value())
    }

    pub fn ld_l_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.c.value())
    }

    pub fn ld_l_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.d.value())
    }

    pub fn ld_l_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.e.value())
    }

    pub fn ld_l_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.h.value())
    }

    pub fn ld_l_l(&mut self) -> u8 {
        let l = self.l.value();
        Z80::ld_r_rp(&mut self.l, l)
    }

    pub fn ld_l_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.a.value())
    }

    pub fn ld_a_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.b.value())
    }

    pub fn ld_a_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.c.value())
    }

    pub fn ld_a_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.d.value())
    }

    pub fn ld_a_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.e.value())
    }

    pub fn ld_a_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.h.value())
    }

    pub fn ld_a_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.l.value())
    }

    pub fn ld_a_a(&mut self) -> u8 {
        let a = self.a.value();
        Z80::ld_r_rp(&mut self.a, a)
    }

    /// ## LD r,n
    ///
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

    fn ld_r_n(r: &mut Register, n: u8) -> u8 {
        r.set_value(n);

        // T states
        7
    }

    pub fn ld_a_n(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let n = self.fetch_next_opcode(mem);
        Z80::ld_r_n(&mut self.a, n)
    }

    pub fn ld_b_n(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let n = self.fetch_next_opcode(mem);
        Z80::ld_r_n(&mut self.b, n)
    }

    pub fn ld_c_n(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let n = self.fetch_next_opcode(mem);
        Z80::ld_r_n(&mut self.c, n)
    }

    pub fn ld_d_n(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let n = self.fetch_next_opcode(mem);
        Z80::ld_r_n(&mut self.d, n)
    }

    pub fn ld_e_n(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let n = self.fetch_next_opcode(mem);
        Z80::ld_r_n(&mut self.e, n)
    }

    pub fn ld_h_n(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let n = self.fetch_next_opcode(mem);
        Z80::ld_r_n(&mut self.h, n)
    }

    pub fn ld_l_n(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let n = self.fetch_next_opcode(mem);
        Z80::ld_r_n(&mut self.l, n)
    }

    /// ## LD r, (HL)
    ///
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
    fn ld_r_mem_hl(r: &mut Register, hl: u16, mem: &dyn Z80Memory) -> u8 {
        let data = mem.read(hl);
        r.set_value(data);

        // T states
        7
    }

    pub fn ld_a_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.a, hl, mem)
    }

    pub fn ld_b_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.b, hl, mem)
    }

    pub fn ld_c_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.c, hl, mem)
    }

    pub fn ld_d_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.d, hl, mem)
    }

    pub fn ld_e_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.e, hl, mem)
    }

    pub fn ld_h_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.h, hl, mem)
    }

    pub fn ld_l_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.l, hl, mem)
    }

    /// ## LD r, (IX+d)
    ///
    /// ### Operation
    ///
    /// r ← (IX+d)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// r, (IX+d)
    /// `1 1 0 1 1 1 0 1` DD
    /// `0 1 r r r 1 1 0`
    /// `d d d d d d d d`
    ///
    /// ### Description
    ///
    /// The (IX+d) operand (i.e., the contents of Index Register IX summed with
    /// two’s-complement displacement integer d) is loaded to register r, in
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
    /// | M Cycles | T States           | 4 M Hz E.T. |
    /// | -------- | ------------------ | ----------- |
    /// | 5        | 19 (4, 4, 3, 5, 3) | 2.50        |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If Index Register IX contains the number 25AFh, the instruction LD B,
    /// (IX+19h) allows the calculation of the sum 25AFh + 19h, which points to
    /// memory location 25C8h. If this address contains byte 39h, the
    /// instruction results in Register B also containing 39h.
    fn ld_r_mem_ixd(r: &mut Register, ix: &u16, d: u8, mem: &dyn Z80Memory) -> u8 {
        let displacement = i8::from_ne_bytes(d.to_ne_bytes());
        let address = ix.wrapping_add_signed(displacement as i16);
        let data = mem.read(address);
        r.set_value(data);

        // T states
        19
    }

    pub fn ld_a_mem_ixd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_ixd(&mut self.a, &self.ix, d, mem)
    }

    pub fn ld_b_mem_ixd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_ixd(&mut self.b, &self.ix, d, mem)
    }

    pub fn ld_c_ixd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_ixd(&mut self.c, &self.ix, d, mem)
    }

    pub fn ld_d_mem_ixd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_ixd(&mut self.d, &self.ix, d, mem)
    }

    pub fn ld_e_mem_ixd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_ixd(&mut self.e, &self.ix, d, mem)
    }

    pub fn ld_h_mem_ixd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_ixd(&mut self.h, &self.ix, d, mem)
    }

    pub fn ld_l_mem_ixd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_ixd(&mut self.l, &self.ix, d, mem)
    }

    /// ## LD r, (IY+d)
    ///
    /// ### Operation
    ///
    /// r ← (IY+d)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// r, (IY+d)
    /// `1 1 0 1 1 1 0 1` FD
    /// `0 1 r r r 1 1 0`
    /// `d d d d d d d d`
    ///
    /// ### Description
    ///
    /// The (IY+d) operand (i.e., the contents of Index Register IY summed with
    /// two’s-complement displacement integer d) is loaded to register r, in
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
    /// | M Cycles | T States           | 4 M Hz E.T. |
    /// | -------- | ------------------ | ----------- |
    /// | 5        | 19 (4, 4, 3, 5, 3) | 2.50        |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If Index Register IY contains the number 25AFh, the instruction LD B,
    /// (IY+19h) allows the calculation of the sum 25AFh + 19h, which points to
    /// memory location 25C8h. If this address contains byte 39h, the
    /// instruction results in Register B also containing 39h.
    fn ld_r_mem_iyd(r: &mut Register, iy: &u16, d: u8, mem: &dyn Z80Memory) -> u8 {
        let displacement = i8::from_ne_bytes(d.to_ne_bytes());
        let address = iy.wrapping_add_signed(displacement as i16);
        let data = mem.read(address);
        r.set_value(data);

        // T states
        19
    }

    pub fn ld_a_mem_iyd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_iyd(&mut self.a, &self.iy, d, mem)
    }

    pub fn ld_b_mem_iyd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_iyd(&mut self.b, &self.iy, d, mem)
    }

    pub fn ld_c_mem_iyd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_iyd(&mut self.c, &self.iy, d, mem)
    }

    pub fn ld_d_mem_iyd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_iyd(&mut self.d, &self.iy, d, mem)
    }

    pub fn ld_e_mem_iyd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_iyd(&mut self.e, &self.iy, d, mem)
    }

    pub fn ld_h_mem_iyd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_iyd(&mut self.h, &self.iy, d, mem)
    }

    pub fn ld_l_mem_iyd(&mut self, mem: &dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_r_mem_iyd(&mut self.l, &self.iy, d, mem)
    }

    /// ## LD (HL), r
    ///
    /// ### Operation
    ///
    /// (HL) ← r
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (HL), r
    /// `0 1 1 1 0 r r r`
    ///
    /// ### Description
    ///
    /// The contents of register r are loaded to the memory location specified
    /// by the contents of the HL register pair. The r symbol identifies
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
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 2        | 7 (4, 3) | 1.75       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the contents of register pair HL specify memory location 2146h and
    /// Register B contains byte 29h, then upon the execution of an LD (HL), B
    /// instruction, memory address 2146h also contains 29h.
    fn ld_mem_hl_r(hl: u16, r: &Register, mem: &mut dyn Z80Memory) -> u8 {
        mem.write(hl, r.value());

        // T states
        7
    }

    pub fn ld_mem_hl_a(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.a, mem)
    }

    pub fn ld_mem_hl_b(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.b, mem)
    }

    pub fn ld_mem_hl_c(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.c, mem)
    }

    pub fn ld_mem_hl_d(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.d, mem)
    }

    pub fn ld_mem_hl_e(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.e, mem)
    }

    pub fn ld_mem_hl_h(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.h, mem)
    }

    pub fn ld_mem_hl_l(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.l, mem)
    }

    /// ## LD (IX+d), r
    ///
    /// ### Operation
    ///
    /// (IX+d) ← r
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (IX+d), r
    /// `1 1 1 0 1 1 0 1` (DD)
    /// `0 1 1 1 0 r r r`
    /// `d d d d d d d d`
    ///
    /// ### Description
    ///
    /// The contents of register r are loaded to the memory address specified by
    ///  the contents of Index Register IX summed with d, a two’s-complement
    /// displacement integer. The r symbol identifies registers A, B, C, D, E,
    /// H, or L, assembled as follows in the object code:
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
    /// | M Cycles | T States           | 4 MHz E.T. |
    /// | -------- | ------------------ | ---------- |
    /// | 5        | 19 (4, 4, 3, 5, 3) | 4.75       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the C register contains byte 1Ch, and Index Register IX contains
    /// 3100h, then the instruction LID (IX + 6h), C performs the sum 3100h + 6h
    /// and loads 1Ch to memory location 3106h.
    fn ld_mem_ixd_r(ix: u16, d: u8, r: &Register, mem: &mut dyn Z80Memory) -> u8 {
        let displacement = i8::from_ne_bytes(d.to_ne_bytes());
        let address = ix.wrapping_add_signed(displacement as i16);
        mem.write(address, r.value());

        // T states
        19
    }

    pub fn ld_mem_ixd_a(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_ixd_r(self.ix, d, &self.a, mem)
    }

    pub fn ld_mem_ixd_b(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_ixd_r(self.ix, d, &self.b, mem)
    }

    pub fn ld_mem_ixd_c(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_ixd_r(self.ix, d, &self.c, mem)
    }

    pub fn ld_mem_ixd_d(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_ixd_r(self.ix, d, &self.d, mem)
    }

    pub fn ld_mem_ixd_e(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_ixd_r(self.ix, d, &self.e, mem)
    }

    pub fn ld_mem_ixd_h(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_ixd_r(self.ix, d, &self.h, mem)
    }

    pub fn ld_mem_ixd_l(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_ixd_r(self.ix, d, &self.l, mem)
    }

    /// ## LD (IY+d), r
    ///
    /// ### Operation
    ///
    /// (IY+d) ← r
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (IY+d), r
    /// `1 1 1 0 1 1 0 1` (DD)
    /// `0 1 1 1 0 r r r`
    /// `d d d d d d d d`
    ///
    /// ### Description
    ///
    /// The contents of register r are loaded to the memory address specified by
    ///  the contents of Index Register IY summed with d, a two’s-complement
    /// displacement integer. The r symbol identifies registers A, B, C, D, E,
    /// H, or L, assembled as follows in the object code:
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
    /// | M Cycles | T States           | 4 MHz E.T. |
    /// | -------- | ------------------ | ---------- |
    /// | 5        | 19 (4, 4, 3, 5, 3) | 4.75       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the C register contains byte 1Ch, and Index Register IY contains
    /// 3100h, then the instruction LID (IY + 6h), C performs the sum 3100h + 6h
    /// and loads 1Ch to memory location 3106h.
    fn ld_mem_iyd_r(iy: u16, d: u8, r: &Register, mem: &mut dyn Z80Memory) -> u8 {
        let displacement = i8::from_ne_bytes(d.to_ne_bytes());
        let address = iy.wrapping_add_signed(displacement as i16);
        mem.write(address, r.value());

        // T states
        19
    }

    pub fn ld_mem_iyd_a(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_iyd_r(self.iy, d, &self.a, mem)
    }

    pub fn ld_mem_iyd_b(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_iyd_r(self.iy, d, &self.b, mem)
    }

    pub fn ld_mem_iyd_c(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_iyd_r(self.iy, d, &self.c, mem)
    }

    pub fn ld_mem_iyd_d(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_iyd_r(self.iy, d, &self.d, mem)
    }

    pub fn ld_mem_iyd_e(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_iyd_r(self.iy, d, &self.e, mem)
    }

    pub fn ld_mem_iyd_h(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_iyd_r(self.iy, d, &self.h, mem)
    }

    pub fn ld_mem_iyd_l(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let d = self.fetch_next_opcode(mem);
        Z80::ld_mem_iyd_r(self.iy, d, &self.l, mem)
    }

    /// ## LD (HL), n
    ///
    /// ### Operation
    ///
    /// (HL) ← n
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (HL), n
    /// `0 0 1 1 0 1 1 0` (36)
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The n integer is loaded to the memory address specified by the contents
    /// of the HL register pair.
    ///
    /// | M Cycles | T States     | 4 MHz E.T. |
    /// | -------- | ------------ | ---------- |
    /// | 3        | 10 (4, 3, 3) | 2.50       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the HL register pair contains 4444h, the instruction LD (HL), 28h
    /// results in the memory location 4444h containing byte 28h.
    pub fn ld_mem_hl_n(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let n = self.fetch_next_opcode(mem);
        mem.write(self.hl(), n);

        // T states
        3
    }

    /// ## LD (IX+d), n
    ///
    /// ### Operation
    ///
    /// (IX+d) ← n
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (IX+d), n
    /// `1 1 0 1 1 1 0 1` (DD)
    /// `0 0 1 1 0 1 1 0` (36)
    /// `d d d d d d d d`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The n operand is loaded to the memory address specified by the sum of
    /// Index Register IX and the two’s complement displacement operand d.
    ///
    /// | M Cycles | T States         | 4 MHz E.T. |
    /// | -------- | ---------------- | ---------- |
    /// | 5        | 19 (4, 4, 3,5,3) | 4.75       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If Index Register IX contains the number 219Ah, then upon execution of
    /// an LD (IX+5h), 5Ah instruction, byte 5Ah is contained in memory address
    /// 219Fh.
    pub fn ld_mem_ixd_n(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let displacement = self.fetch_next_opcode(mem);
        let n = self.fetch_next_opcode(mem);
        let address = self.ix.wrapping_add_signed(displacement as i16);
        mem.write(address, n);

        // T states
        19
    }

    /// ## LD (IY+d), n
    ///
    /// ### Operation
    ///
    /// (IY+d) ← n
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (IY+d), n
    /// `1 1 1 1 1 1 0 1` (FD)
    /// `0 0 1 1 0 1 1 0` (36)
    /// `d d d d d d d d`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The n operand is loaded to the memory address specified by the sum of
    /// Index Register IX and the two’s complement displacement operand d.
    ///
    /// | M Cycles | T States         | 4 MHz E.T. |
    /// | -------- | ---------------- | ---------- |
    /// | 5        | 19 (4, 4, 3,5,3) | 4.75       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If Index Register IY contains the number 219Ah, then upon execution of
    /// an LD (IY+5h), 5Ah instruction, byte 5Ah is contained in memory address
    /// 219Fh.
    pub fn ld_mem_iyd_n(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let displacement = self.fetch_next_opcode(mem);
        let n = self.fetch_next_opcode(mem);
        let address = self.iy.wrapping_add_signed(displacement as i16);
        mem.write(address, n);

        // T states
        19
    }

    /// ## LD A, (BC)
    ///
    /// ### Operation
    ///
    /// A ← (BC)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// A, (BC)
    /// `0 0 0 0 1 0 1 0` (AA)
    ///
    /// ### Description
    ///
    /// The contents of the memory location specified by the contents of the BC
    /// register pair are loaded to the Accumulator.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 2        | 7 (4, 3) | 1.75       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the BC register pair contains the number 4747h, and memory address
    /// 4747h contains byte 12h, then the instruction LD A, (BC) results in byte
    /// 12h in Register A.
    pub fn ld_a_mem_bc(&mut self, mem: &dyn Z80Memory) -> u8 {
        self.a.set_value(mem.read(self.bc()));

        // T states
        7
    }

    /// ## LD A, (DE)
    ///
    /// ### Operation
    ///
    /// A ← (DE)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// A, (DE)
    /// `0 0 0 1 1 0 1 0` (AA)
    ///
    /// ### Description
    ///
    /// The contents of the memory location specified by the contents of the DE
    /// register pair are loaded to the Accumulator.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 2        | 7 (4, 3) | 1.75       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the DE register pair contains the number 30A2h, and memory address
    /// 30A2h contains byte 22h, then the instruction LD A, (DE) results in byte
    /// 22h in Register A.
    pub fn ld_a_mem_de(&mut self, mem: &dyn Z80Memory) -> u8 {
        self.a.set_value(mem.read(self.de()));

        // T states
        7
    }

    /// ## LD A, (nn)
    ///
    /// ### Operation
    ///
    /// A ← (nn)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// A, (nn)
    /// `0 0 1 1 1 0 1 0` (3A)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    /// The contents of the memory location specified by the operands nn are loaded
    /// to the Accumulator. The first n operand after the op code is the low-order
    /// byte of a 2-byte memoryaddress.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 13 (4, 3, 3, 3) | 3.25       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If nn contains 8832h and memory address 8832h contains byte 04h, then upon
    /// the execution of an LD A, (nn) instruction, the 04h byte is in the
    /// Accumulator.
    pub fn ld_a_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let nl = self.fetch_next_opcode(mem);
        let nh = self.fetch_next_opcode(mem);

        let address = ((nh as u16) << 8) | nl as u16;
        self.a.set_value(mem.read(address));

        // T states
        13
    }

    /// ## LD (BC), A
    ///
    /// ### Operation
    ///
    /// (BC) ← A
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (BC), A
    /// `0 0 0 0 0 0 1 0` (02)
    ///
    /// ### Description
    ///
    /// The contents of the Accumulator are loaded to the memory location
    /// specified by the contents of the register pair BC.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 2        | 7 (4, 3) | 1.75       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the Accumulator contains 7Ah and the BC register pair contains 1212h
    /// the instruction LD (BC), A results in 7Ah in memory location 1212h.
    pub fn ld_mem_bc_a(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        mem.write(self.bc(), self.a.value());

        // T states
        7
    }

    /// ## LD (DE), A
    ///
    /// ### Operation
    ///
    /// (DE) ← A
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (DE), A
    /// `0 0 0 1 0 0 1 0` (12)
    ///
    /// ### Description
    ///
    /// The contents of the Accumulator are loaded to the memory location
    /// specified by the contents of the register pair DE.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 2        | 7 (4, 3) | 1.75       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If register pair DE contains 1128h and the Accumulator contains byte
    /// A0h, then the execution of a LD (DE), A instruction results in A0h being
    /// stored in memory location 1128h.
    pub fn ld_mem_de_a(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        mem.write(self.de(), self.a.value());

        // T states
        7
    }

    /// ## LD (nn), A
    ///
    /// ### Operation
    ///
    /// (nn) ← A
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (nn), A
    /// `0 0 1 1 0 0 1 0` (32)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The contents of the Accumulator are loaded to the memory address
    /// specified by the operand nn. The first n operand after the op code is
    /// the low-order byte of nn.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 13 (4, 3, 3, 3) | 3.25       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the Accumulator contains byte D7h, then executing an LD (3141h), AD7h
    /// instruction results in memory location 3141h.
    pub fn ld_mem_nn_a(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let nl = self.fetch_next_opcode(mem);
        let nh = self.fetch_next_opcode(mem);
        let address = ((nh as u16) << 8) | nl as u16;
        mem.write(address, self.a.value());

        // T states
        13
    }

    /// ## LD A, I
    ///
    /// ### Operation
    ///
    /// A ← 1
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    /// A, I
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `0 1 0 1 0 1 1 1` (57)
    ///
    /// ### Description
    ///
    /// The contents of the Interrupt Vector Register I are loaded to the
    /// Accumulator.
    ///
    /// | M Cycles | T States | MHz E.T. |
    /// | -------- | -------- | -------- |
    /// | 2        | 9 (4, 5) | 2.25     |
    ///
    /// ### Condition Bits Affected
    ///
    /// S is set if the I Register is negative; otherwise, it is reset.
    /// Z is set if the I Register is 0; otherwise, it is reset.
    /// H is reset.
    /// P/V contains contents of IFF2.
    /// N is reset.
    /// C is not affected.
    /// If an interrupt occurs during execution of this instruction, the Parity flag contains a 0.
    pub fn ld_a_i(&mut self) -> u8 {
        self.a.set_value(self.i.value());

        // S is set if the I Register is negative; otherwise, it is reset.
        set_s_flag_with(&mut self.f, s_flag_set(&self.i));

        // Z is set if the I Register is 0; otherwise, it is reset.
        set_z_flag_with(&mut self.f, self.i.value() == 0);

        // H is reset.
        unset_h_flag(&mut self.f);

        // P/V contains contents of IFF2.
        set_p_flag_with(&mut self.f, self.iff2);

        // N is reset.
        unset_n_flag(&mut self.f);

        // TODO: If an interrupt occurs during execution of this instruction,
        // the Parity flag contains a 0.

        // T states
        9
    }

    /// ## LD A, R
    ///
    /// ### Operation
    ///
    /// A ← R
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// A, R
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `0 1 0 1 1 1 1 1` (5F)
    ///
    /// ### Description
    ///
    /// The contents of Memory Refresh Register R are loaded to the Accumulator.
    ///
    /// | M Cycles | T States | MHz E.T. |
    /// | -------- | -------- | -------- |
    /// | 2        | 9 (4, 5) | 2.25     |
    ///
    /// ### Condition Bits Affected
    ///
    /// S is set if, R-Register is negative; otherwise, it is reset.
    /// Z is set if the R Register is 0; otherwise, it is reset.
    /// H is reset.
    /// P/V contains contents of IFF2.
    /// N is reset.
    /// C is not affected.
    /// If an interrupt occurs during execution of this instruction, the parity
    /// contains a 0.
    pub fn ld_a_r(&mut self) -> u8 {
        self.a.set_value(self.r.value());

        // S is set if the R-Register is negative; otherwise, it is reset.
        set_s_flag_with(&mut self.f, s_flag_set(&self.r));

        // Z is set if the R Register is 0; otherwise, it is reset.
        set_z_flag_with(&mut self.f, self.r.value() == 0);

        // H is reset.
        unset_h_flag(&mut self.f);

        // P/V contains contents of IFF2.
        set_p_flag_with(&mut self.f, self.iff2);

        // N is reset.
        unset_n_flag(&mut self.f);

        // TODO: If an interrupt occurs during execution of this instruction,
        // the Parity flag contains a 0.

        // T states
        9
    }

    /// ## LD I,A
    ///
    /// ### Operation
    ///
    /// I ← A
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// I, A
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `0 1 0 0 0 1 1 1` (47)
    ///
    /// ### Description
    ///
    /// The contents of the Accumulator are loaded to the Interrupt Control
    /// Vector Register, I.
    ///
    /// | M Cycles | T States | MHz E.T. |
    /// | -------- | -------- | -------- |
    /// | 2        | 9 (4, 5) | 2.25     |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    pub fn ld_i_a(&mut self) -> u8 {
        self.i.set_value(self.a.value());

        // T states
        9
    }

    /// ## LD R,A
    ///
    /// ### Operation
    ///
    /// R ← A
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// R, A
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `0 1 0 0 1 1 1 1` (47)
    ///
    /// ### Description
    ///
    /// The contents of the Accumulator are loaded to the Memory Refresh
    /// register R.
    ///
    /// | M Cycles | T States | MHz E.T. |
    /// | -------- | -------- | -------- |
    /// | 2        | 9 (4, 5) | 2.25     |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    pub fn ld_r_a(&mut self) -> u8 {
        self.r.set_value(self.a.value());

        // T states
        9
    }
}

mod tests {
    use super::*;

    use crate::z80::{
        register_flags::{h_flag_set, n_flag_set, p_flag_set, z_flag_set},
        tests::Ram,
    };

    #[test]
    fn test_ld_r_rp() {
        #[rustfmt::skip]
        let scenarios: [(
            fn(&mut Z80) -> u8,
            fn(&mut Z80) -> &mut Register,
            fn(&mut Z80) -> &mut Register,
        ); 49] = [
            (Z80::ld_a_a, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_a_b, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_a_c, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_a_d, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_a_e, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_a_h, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_a_l, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_b_a, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_b_b, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_b_c, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_b_d, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_b_e, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_b_h, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_b_l, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_c_a, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_c_b, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_c_c, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_c_d, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_c_e, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_c_h, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_c_l, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_d_a, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_d_b, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_d_c, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_d_d, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_d_e, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_d_h, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_d_l, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_e_a, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_e_b, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_e_c, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_e_d, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_e_e, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_e_h, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_e_l, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_h_a, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_h_b, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_h_c, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_h_d, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_h_e, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_h_h, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_h_l, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_l_a, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_l_b, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_l_c, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_l_d, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_l_e, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_l_h, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_l_l, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.l),
        ];

        for (opcode, r_supplier, r_prime_supplier) in scenarios {
            let z80 = &mut Z80::new();

            r_prime_supplier(z80).set_value(0xDD);

            let t_states = opcode(z80);
            assert_eq!(4, t_states);

            let r = r_supplier(z80);
            assert_eq!(0xDD, r.value());
        }
    }

    #[test]
    fn test_ld_r_hl() {
        let scenarios: [(
            fn(&mut Z80, &dyn Z80Memory) -> u8,
            fn(&mut Z80) -> &mut Register,
        ); 7] = [
            (Z80::ld_a_mem_hl, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_b_mem_hl, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_c_mem_hl, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_d_mem_hl, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_e_mem_hl, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_h_mem_hl, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_l_mem_hl, |z80: &mut Z80| &mut z80.l),
        ];

        let bytes = &mut [0xAA, 0xBB, 0xCC];
        let ram = Ram::new(bytes);

        for (opcode, register_supplier) in scenarios {
            let mut z80 = Z80::new();
            z80.set_hl(0x0002);

            let t_states = opcode(&mut z80, &ram);
            assert_eq!(7, t_states);

            let register = register_supplier(&mut z80);
            assert_eq!(ram.read(2), register.value());
        }
    }

    #[test]
    fn test_ld_r_ixd() {
        let scenarios: [(
            fn(&mut Z80, &dyn Z80Memory) -> u8,
            fn(&mut Z80) -> &mut Register,
        ); 7] = [
            (Z80::ld_a_mem_ixd, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_b_mem_ixd, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_c_ixd, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_d_mem_ixd, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_e_mem_ixd, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_h_mem_ixd, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_l_mem_ixd, |z80: &mut Z80| &mut z80.l),
        ];

        let mut bytes = [0xDD, 0x77, (-2i8).to_le_bytes()[0], 0xCC];
        let ram = Ram::new(&mut bytes);

        for (opcode, register_supplier) in scenarios {
            let z80 = &mut Z80::new();
            z80.program_counter = 2;
            z80.ix = 5;

            let t_states = opcode(z80, &ram);
            assert_eq!(19, t_states);

            let register = register_supplier(z80);
            assert_eq!(0xCC, register.value());
        }
    }

    #[test]
    fn test_ld_r_iyd() {
        let scenarios: [(
            fn(&mut Z80, &dyn Z80Memory) -> u8,
            fn(&mut Z80) -> &mut Register,
        ); 7] = [
            (Z80::ld_a_mem_iyd, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_b_mem_iyd, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_c_mem_iyd, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_d_mem_iyd, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_e_mem_iyd, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_h_mem_iyd, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_l_mem_iyd, |z80: &mut Z80| &mut z80.l),
        ];

        let mut bytes = [0xDD, 0x77, (-2i8).to_le_bytes()[0], 0xCC];
        let ram = Ram::new(&mut bytes);

        for (opcode, register_supplier) in scenarios {
            let z80 = &mut Z80::new();
            z80.program_counter = 2;
            z80.iy = 5;

            let t_states = opcode(z80, &ram);
            assert_eq!(19, t_states);

            let register = register_supplier(z80);
            assert_eq!(0xCC, register.value());
        }
    }

    #[test]
    fn test_ld_hl_r() {
        let scenarios: [(
            fn(&Z80, &mut dyn Z80Memory) -> u8,
            fn(&mut Z80) -> &mut Register,
        ); 5] = [
            (Z80::ld_mem_hl_a, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_mem_hl_b, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_mem_hl_c, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_mem_hl_d, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_mem_hl_e, |z80: &mut Z80| &mut z80.e),
            // (Z80::ld_hl_h, |z80: &mut Z80| &mut z80.h),
            // (Z80::ld_hl_l, |z80: &mut Z80| &mut z80.l),
        ];

        let bytes = &mut [0, 0, 0];
        let ram = &mut Ram::new(bytes);

        for (opcode, register_supplier) in scenarios {
            let z80 = &mut Z80::new();
            z80.set_hl(0x0002);

            let register = register_supplier(z80);
            register.set_value(0xDD);

            let t_states = opcode(z80, ram);
            assert_eq!(7, t_states);

            assert_eq!(0xDD, ram.read(2));
        }
    }

    #[test]
    fn test_ld_hl_h() {
        let bytes = &mut [5, 5, 5];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.set_hl(0x0002);

        let t_states = z80.ld_mem_hl_h(ram);
        assert_eq!(7, t_states);

        assert_eq!(z80.h.value(), ram.read(2));
    }

    #[test]
    fn test_ld_hl_l() {
        let bytes = &mut [5, 5, 5];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.set_hl(0x0002);

        let t_states = z80.ld_mem_hl_l(ram);
        assert_eq!(7, t_states);

        assert_eq!(z80.l.value(), ram.read(2));
    }

    #[test]
    fn test_ld_ixd_r() {
        let scenarios: [(
            fn(&mut Z80, &mut dyn Z80Memory) -> u8,
            fn(&mut Z80) -> &mut Register,
        ); 7] = [
            (Z80::ld_mem_ixd_a, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_mem_ixd_b, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_mem_ixd_c, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_mem_ixd_d, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_mem_ixd_e, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_mem_ixd_h, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_mem_ixd_l, |z80: &mut Z80| &mut z80.l),
        ];

        let bytes = &mut [0xDD, 0x77, (-2i8).to_le_bytes()[0], 0x00];
        let ram = &mut Ram::new(bytes);

        for (opcode, register_supplier) in scenarios {
            let z80 = &mut Z80::new();
            z80.program_counter = 2;
            z80.ix = 5;
            register_supplier(z80).set_value(0xFF);

            let t_states = opcode(z80, ram);
            assert_eq!(19, t_states);

            assert_eq!(0xFF, ram.read(3));
        }
    }

    #[test]
    fn test_ld_iyd_r() {
        let scenarios: [(
            fn(&mut Z80, &mut dyn Z80Memory) -> u8,
            fn(&mut Z80) -> &mut Register,
        ); 7] = [
            (Z80::ld_mem_iyd_a, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_mem_iyd_b, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_mem_iyd_c, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_mem_iyd_d, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_mem_iyd_e, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_mem_iyd_h, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_mem_iyd_l, |z80: &mut Z80| &mut z80.l),
        ];

        let bytes = &mut [0xDD, 0x77, (-2i8).to_le_bytes()[0], 0x00];
        let ram = &mut Ram::new(bytes);

        for (opcode, register_supplier) in scenarios {
            let z80 = &mut Z80::new();
            z80.program_counter = 2;
            z80.iy = 5;
            register_supplier(z80).set_value(0xFF);

            let t_states = opcode(z80, ram);
            assert_eq!(19, t_states);

            assert_eq!(0xFF, ram.read(3));
        }
    }

    #[test]
    fn test_ld_hl_n() {
        let bytes = &mut [0x36, 0xFF, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.set_hl(0x0002);
        let t_states = z80.ld_mem_hl_n(ram);

        assert_eq!(3, t_states);
        assert_eq!(0xFF, ram.read(2));
    }

    #[test]
    fn test_ld_ixd_n() {
        let bytes = &mut [0xDD, 0x26, 0x02, 0xFF, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.ix = 0x02;
        let t_states = z80.ld_mem_ixd_n(ram);

        assert_eq!(19, t_states);
        assert_eq!(0xFF, ram.read(4));
    }

    #[test]
    fn test_ld_iyd_n() {
        let bytes = &mut [0xDD, 0x26, 0x02, 0xFF, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.iy = 0x02;

        let t_states = z80.ld_mem_iyd_n(ram);

        assert_eq!(19, t_states);
        assert_eq!(0xFF, ram.read(4));
    }

    #[test]
    fn test_ld_a_bc() {
        let bytes = &mut [0x0A, 0xFF];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.set_bc(0x0001);

        let t_states = z80.ld_a_mem_bc(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, z80.a.value());
    }

    #[test]
    fn test_ld_a_de() {
        let bytes = &mut [0x0A, 0xFF];
        let ram = &Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.set_de(0x0001);

        let t_states = z80.ld_a_mem_de(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, z80.a.value());
    }

    #[test]
    fn test_ld_a_nn() {
        let bytes = &mut [0x3A, 0x03, 0x00, 0xFF];
        let ram = &Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        let t_states = z80.ld_a_mem_nn(ram);

        assert_eq!(13, t_states);
        assert_eq!(0xFF, z80.a.value());
    }

    #[test]
    fn test_ld_bc_a() {
        let bytes = &mut [0x00, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.a.set_value(0xFF);
        z80.set_bc(0x0001);
        let t_states = z80.ld_mem_bc_a(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, ram.read(1));
    }

    #[test]
    fn test_ld_de_a() {
        let bytes = &mut [0x00, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.a.set_value(0xFF);
        z80.set_de(0x0001);
        let t_states = z80.ld_mem_de_a(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, ram.read(1));
    }

    #[test]
    fn test_ld_nn_a() {
        let bytes = &mut [0x32, 0x03, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.a.set_value(0xFF);
        let t_states = z80.ld_mem_nn_a(ram);

        assert_eq!(13, t_states);
        assert_eq!(0xFF, ram.read(3));
    }

    #[test]
    fn test_ld_a_i_positive() {
        let z80 = &mut Z80::new();
        z80.i.set_value(0b01111111);
        let t_states = z80.ld_a_i();
        assert_eq!(9, t_states);

        assert_eq!(z80.i, z80.a);

        // S is set if the I Register is negative; otherwise, it is reset.
        assert_eq!(false, s_flag_set(&z80.f));

        // Z is set if the I Register is 0; otherwise, it is reset.
        assert_eq!(false, z_flag_set(&z80.f));

        // H is reset.
        assert_eq!(false, h_flag_set(&z80.f));

        // P/V contains contents of IFF2.
        assert_eq!(z80.iff2, p_flag_set(&z80.f));

        // N is reset.
        assert_eq!(false, n_flag_set(&z80.f));

        // TODO: If an interrupt occurs during execution of this instruction, the Parity flag contains a 0.
    }

    #[test]
    fn test_ld_a_i_zero() {
        let z80 = &mut Z80::new();
        z80.i.set_value(0);
        let t_states = z80.ld_a_i();
        assert_eq!(9, t_states);

        assert_eq!(z80.i, z80.a);

        // S is set if the I Register is negative; otherwise, it is reset.
        assert_eq!(false, s_flag_set(&z80.f));

        // Z is set if the I Register is 0; otherwise, it is reset.
        assert_eq!(true, z_flag_set(&z80.f));

        // H is reset.
        assert_eq!(false, h_flag_set(&z80.f));

        // P/V contains contents of IFF2.
        assert_eq!(z80.iff2, p_flag_set(&z80.f));

        // N is reset.
        assert_eq!(false, n_flag_set(&z80.f));

        // TODO: If an interrupt occurs during execution of this instruction, the Parity flag contains a 0.
    }

    #[test]
    fn test_ld_a_i_negative() {
        let z80 = &mut Z80::new();
        z80.i.set_value(0b11111111);
        let t_states = z80.ld_a_i();
        assert_eq!(9, t_states);

        assert_eq!(z80.i, z80.a);

        // S is set if the I Register is negative; otherwise, it is reset.
        assert_eq!(true, s_flag_set(&z80.f));

        // Z is set if the I Register is 0; otherwise, it is reset.
        assert_eq!(false, z_flag_set(&z80.f));

        // H is reset.
        assert_eq!(false, h_flag_set(&z80.f));

        // P/V contains contents of IFF2.
        assert_eq!(z80.iff2, p_flag_set(&z80.f));

        // N is reset.
        assert_eq!(false, n_flag_set(&z80.f));

        // TODO: If an interrupt occurs during execution of this instruction, the Parity flag contains a 0.
    }

    #[test]
    fn test_ld_a_r_positive() {
        let z80 = &mut Z80::new();
        z80.r.set_value(0b01111111);
        let t_states = z80.ld_a_r();
        assert_eq!(9, t_states);

        assert_eq!(z80.r, z80.a);

        // S is set if the R Register is negative; otherwise, it is reset.
        assert_eq!(false, s_flag_set(&z80.f));

        // Z is set if the R Register is 0; otherwise, it is reset.
        assert_eq!(false, z_flag_set(&z80.f));

        // H is reset.
        assert_eq!(false, h_flag_set(&z80.f));

        // P/V contains contents of IFF2.
        assert_eq!(z80.iff2, p_flag_set(&z80.f));

        // N is reset.
        assert_eq!(false, n_flag_set(&z80.f));

        // TODO: If an interrupt occurs during execution of this instruction, the Parity flag contains a 0.
    }

    #[test]
    fn test_ld_a_r_zero() {
        let z80 = &mut Z80::new();
        z80.r.set_value(0);
        let t_states = z80.ld_a_r();
        assert_eq!(9, t_states);

        assert_eq!(z80.r, z80.a);

        // S is set if the R Register is negative; otherwise, it is reset.
        assert_eq!(false, s_flag_set(&z80.f));

        // Z is set if the R Register is 0; otherwise, it is reset.
        assert_eq!(true, z_flag_set(&z80.f));

        // H is reset.
        assert_eq!(false, h_flag_set(&z80.f));

        // P/V contains contents of IFF2.
        assert_eq!(z80.iff2, p_flag_set(&z80.f));

        // N is reset.
        assert_eq!(false, n_flag_set(&z80.f));

        // TODO: If an interrupt occurs during execution of this instruction, the Parity flag contains a 0.
    }

    #[test]
    fn test_ld_a_r_negative() {
        let z80 = &mut Z80::new();
        z80.r.set_value(0b11111111);
        let t_states = z80.ld_a_r();
        assert_eq!(9, t_states);

        assert_eq!(z80.r, z80.a);

        // S is set if the R Register is negative; otherwise, it is reset.
        assert_eq!(true, s_flag_set(&z80.f));

        // Z is set if the R Register is 0; otherwise, it is reset.
        assert_eq!(false, z_flag_set(&z80.f));

        // H is reset.
        assert_eq!(false, h_flag_set(&z80.f));

        // P/V contains contents of IFF2.
        assert_eq!(z80.iff2, p_flag_set(&z80.f));

        // N is reset.
        assert_eq!(false, n_flag_set(&z80.f));

        // TODO: If an interrupt occurs during execution of this instruction, the Parity flag contains a 0.
    }

    #[test]
    fn test_ld_i_a() {
        let z80 = &mut Z80::new();
        z80.a.set_value(0xFF);

        let t_states = z80.ld_i_a();
        assert_eq!(9, t_states);

        assert_eq!(0xFF, z80.i.value());
    }

    #[test]
    fn test_ld_r_a() {
        let z80 = &mut Z80::new();
        z80.a.set_value(0xFF);

        let t_states = z80.ld_r_a();
        assert_eq!(9, t_states);

        assert_eq!(0xFF, z80.r.value());
    }
}
