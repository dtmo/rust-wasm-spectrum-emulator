use super::{MemoryAccessor, Z80};

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
    fn ld_r_rp(r: &mut u8, r_prime: &u8) -> u8 {
        *r = *r_prime;

        // T states
        4
    }

    pub fn ld_b_b(&mut self) -> u8 {
        let b = self.b;
        Z80::ld_r_rp(&mut self.b, &b)
    }

    pub fn ld_b_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, &self.c)
    }

    pub fn ld_b_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, &self.d)
    }

    pub fn ld_b_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, &self.e)
    }

    pub fn ld_b_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, &self.h)
    }

    pub fn ld_b_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, &self.l)
    }

    pub fn ld_b_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, &self.a)
    }

    pub fn ld_c_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, &self.b)
    }

    pub fn ld_c_c(&mut self) -> u8 {
        let c = self.c;
        Z80::ld_r_rp(&mut self.c, &c)
    }

    pub fn ld_c_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, &self.d)
    }

    pub fn ld_c_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, &self.e)
    }

    pub fn ld_c_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, &self.h)
    }

    pub fn ld_c_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, &self.l)
    }

    pub fn ld_c_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, &self.a)
    }

    pub fn ld_d_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, &self.b)
    }

    pub fn ld_d_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, &self.c)
    }

    pub fn ld_d_d(&mut self) -> u8 {
        let d = self.d;
        Z80::ld_r_rp(&mut self.d, &d)
    }

    pub fn ld_d_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, &self.e)
    }

    pub fn ld_d_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, &self.h)
    }

    pub fn ld_d_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, &self.l)
    }

    pub fn ld_d_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, &self.a)
    }

    pub fn ld_e_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, &self.b)
    }

    pub fn ld_e_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, &self.c)
    }

    pub fn ld_e_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, &self.d)
    }

    pub fn ld_e_e(&mut self) -> u8 {
        let e = self.e;
        Z80::ld_r_rp(&mut self.e, &e)
    }

    pub fn ld_e_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, &self.h)
    }

    pub fn ld_e_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, &self.l)
    }

    pub fn ld_e_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, &self.a)
    }

    pub fn ld_h_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, &self.b)
    }

    pub fn ld_h_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, &self.c)
    }

    pub fn ld_h_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, &self.d)
    }

    pub fn ld_h_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, &self.e)
    }

    pub fn ld_h_h(&mut self) -> u8 {
        let h = self.h;
        Z80::ld_r_rp(&mut self.h, &h)
    }

    pub fn ld_h_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, &self.l)
    }

    pub fn ld_h_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, &self.a)
    }

    pub fn ld_l_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, &self.b)
    }

    pub fn ld_l_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, &self.c)
    }

    pub fn ld_l_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, &self.d)
    }

    pub fn ld_l_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, &self.e)
    }

    pub fn ld_l_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, &self.h)
    }

    pub fn ld_l_l(&mut self) -> u8 {
        let l = self.l;
        Z80::ld_r_rp(&mut self.l, &l)
    }

    pub fn ld_l_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, &self.a)
    }

    pub fn ld_a_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, &self.b)
    }

    pub fn ld_a_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, &self.c)
    }

    pub fn ld_a_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, &self.d)
    }

    pub fn ld_a_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, &self.e)
    }

    pub fn ld_a_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, &self.h)
    }

    pub fn ld_a_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, &self.l)
    }

    pub fn ld_a_a(&mut self) -> u8 {
        let a = self.a;
        Z80::ld_r_rp(&mut self.a, &a)
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

    fn ld_r_n(r: &mut u8, n: &u8) -> u8 {
        *r = *n;

        // T states
        7
    }

    pub fn ld_a_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.a, &n)
    }

    pub fn ld_b_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.b, &n)
    }

    pub fn ld_c_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.c, &n)
    }

    pub fn ld_d_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.d, &n)
    }

    pub fn ld_e_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.e, &n)
    }

    pub fn ld_h_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.h, &n)
    }

    pub fn ld_l_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let n = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_n(&mut self.l, &n)
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
    fn ld_r_hl(r: &mut u8, h: &u8, l: &u8, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let address = ((*h as u16) << 8) | *l as u16;
        let data = memory_accessor.read(&address);

        *r = data;

        // T states
        7
    }

    pub fn ld_a_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        Z80::ld_r_hl(&mut self.a, &self.h, &self.l, memory_accessor)
    }

    pub fn ld_b_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        Z80::ld_r_hl(&mut self.b, &self.h, &self.l, memory_accessor)
    }

    pub fn ld_c_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        Z80::ld_r_hl(&mut self.c, &self.h, &self.l, memory_accessor)
    }

    pub fn ld_d_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        Z80::ld_r_hl(&mut self.d, &self.h, &self.l, memory_accessor)
    }

    pub fn ld_e_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        Z80::ld_r_hl(&mut self.e, &self.h, &self.l, memory_accessor)
    }

    pub fn ld_h_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let h = self.h;
        Z80::ld_r_hl(&mut self.h, &h, &self.l, memory_accessor)
    }

    pub fn ld_l_hl(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let l = self.l;
        Z80::ld_r_hl(&mut self.l, &self.h, &l, memory_accessor)
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
    fn ld_r_ixd(r: &mut u8, ix: &u16, d: u8, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let displacement = i8::from_ne_bytes(d.to_ne_bytes());
        let address = ix.wrapping_add_signed(displacement as i16);
        let data = memory_accessor.read(&address);
        *r = data;

        // T states
        19
    }

    pub fn ld_a_ixd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_ixd(&mut self.a, &self.ix, d, memory_accessor)
    }

    pub fn ld_b_ixd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_ixd(&mut self.b, &self.ix, d, memory_accessor)
    }

    pub fn ld_c_ixd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_ixd(&mut self.c, &self.ix, d, memory_accessor)
    }

    pub fn ld_d_ixd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_ixd(&mut self.d, &self.ix, d, memory_accessor)
    }

    pub fn ld_e_ixd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_ixd(&mut self.e, &self.ix, d, memory_accessor)
    }

    pub fn ld_h_ixd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_ixd(&mut self.h, &self.ix, d, memory_accessor)
    }

    pub fn ld_l_ixd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_ixd(&mut self.l, &self.ix, d, memory_accessor)
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
    fn ld_r_iyd(r: &mut u8, iy: &u16, d: u8, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let displacement = i8::from_ne_bytes(d.to_ne_bytes());
        let address = iy.wrapping_add_signed(displacement as i16);
        let data = memory_accessor.read(&address);
        *r = data;

        // T states
        19
    }

    pub fn ld_a_iyd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_iyd(&mut self.a, &self.iy, d, memory_accessor)
    }

    pub fn ld_b_iyd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_iyd(&mut self.b, &self.iy, d, memory_accessor)
    }

    pub fn ld_c_iyd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_iyd(&mut self.c, &self.iy, d, memory_accessor)
    }

    pub fn ld_d_iyd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_iyd(&mut self.d, &self.iy, d, memory_accessor)
    }

    pub fn ld_e_iyd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_iyd(&mut self.e, &self.iy, d, memory_accessor)
    }

    pub fn ld_h_iyd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_iyd(&mut self.h, &self.iy, d, memory_accessor)
    }

    pub fn ld_l_iyd(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_r_iyd(&mut self.l, &self.iy, d, memory_accessor)
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
    fn ld_hl_r(h: &u8, l: &u8, r: &u8, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let address = ((*h as u16) << 8) | *l as u16;
        memory_accessor.write(&address, r);

        // T states
        7
    }

    pub fn ld_hl_a(&self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        Z80::ld_hl_r(&self.h, &self.l, &self.a, memory_accessor)
    }

    pub fn ld_hl_b(&self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        Z80::ld_hl_r(&self.h, &self.l, &self.b, memory_accessor)
    }

    pub fn ld_hl_c(&self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        Z80::ld_hl_r(&self.h, &self.l, &self.c, memory_accessor)
    }

    pub fn ld_hl_d(&self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        Z80::ld_hl_r(&self.h, &self.l, &self.d, memory_accessor)
    }

    pub fn ld_hl_e(&self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        Z80::ld_hl_r(&self.h, &self.l, &self.e, memory_accessor)
    }

    pub fn ld_hl_h(&self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        Z80::ld_hl_r(&self.h, &self.l, &self.h, memory_accessor)
    }

    pub fn ld_hl_l(&self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        Z80::ld_hl_r(&self.h, &self.l, &self.l, memory_accessor)
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
    fn ld_ixd_r(ix: &u16, d: &u8, r: &u8, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let displacement = i8::from_ne_bytes(d.to_ne_bytes());
        let address = ix.wrapping_add_signed(displacement as i16);
        memory_accessor.write(&address, r);

        // T states
        19
    }

    pub fn ld_ixd_a(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_ixd_r(&self.ix, &d, &self.a, memory_accessor)
    }

    pub fn ld_ixd_b(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_ixd_r(&self.ix, &d, &self.b, memory_accessor)
    }

    pub fn ld_ixd_c(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_ixd_r(&self.ix, &d, &self.c, memory_accessor)
    }

    pub fn ld_ixd_d(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_ixd_r(&self.ix, &d, &self.d, memory_accessor)
    }

    pub fn ld_ixd_e(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_ixd_r(&self.ix, &d, &self.e, memory_accessor)
    }

    pub fn ld_ixd_h(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_ixd_r(&self.ix, &d, &self.h, memory_accessor)
    }

    pub fn ld_ixd_l(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_ixd_r(&self.ix, &d, &self.l, memory_accessor)
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
    fn ld_iyd_r(iy: &u16, d: &u8, r: &u8, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let displacement = i8::from_ne_bytes(d.to_ne_bytes());
        let address = iy.wrapping_add_signed(displacement as i16);
        memory_accessor.write(&address, r);

        // T states
        19
    }

    pub fn ld_iyd_a(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_iyd_r(&self.iy, &d, &self.a, memory_accessor)
    }

    pub fn ld_iyd_b(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_iyd_r(&self.iy, &d, &self.b, memory_accessor)
    }

    pub fn ld_iyd_c(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_iyd_r(&self.iy, &d, &self.c, memory_accessor)
    }

    pub fn ld_iyd_d(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_iyd_r(&self.iy, &d, &self.d, memory_accessor)
    }

    pub fn ld_iyd_e(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_iyd_r(&self.iy, &d, &self.e, memory_accessor)
    }

    pub fn ld_iyd_h(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_iyd_r(&self.iy, &d, &self.h, memory_accessor)
    }

    pub fn ld_iyd_l(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let d = self.fetch_next_opcode(memory_accessor);
        Z80::ld_iyd_r(&self.iy, &d, &self.l, memory_accessor)
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
    pub fn ld_hl_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let address = ((self.h as u16) << 8) | self.l as u16;
        let n = self.fetch_next_opcode(memory_accessor);
        memory_accessor.write(&address, &n);

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
    pub fn ld_ixd_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let displacement = self.fetch_next_opcode(memory_accessor);
        let n = self.fetch_next_opcode(memory_accessor);
        let address = self.ix.wrapping_add_signed(displacement as i16);
        memory_accessor.write(&address, &n);

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
    pub fn ld_iyd_n(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let displacement = self.fetch_next_opcode(memory_accessor);
        let n = self.fetch_next_opcode(memory_accessor);
        let address = self.iy.wrapping_add_signed(displacement as i16);
        memory_accessor.write(&address, &n);

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
    pub fn ld_a_bc(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let address = ((self.b as u16) << 8) | self.c as u16;
        self.a = memory_accessor.read(&address);

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
    pub fn ld_a_de(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let address = ((self.d as u16) << 8) | self.e as u16;
        self.a = memory_accessor.read(&address);

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
    pub fn ld_a_nn(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let nh = self.fetch_next_opcode(memory_accessor);
        let nl = self.fetch_next_opcode(memory_accessor);

        let address = ((nh as u16) << 8) | nl as u16;
        self.a = memory_accessor.read(&address);

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
    pub fn ld_bc_a(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let address = ((self.b as u16) << 8) | self.c as u16;

        memory_accessor.write(&address, &self.a);

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
    pub fn ld_de_a(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let address = ((self.d as u16) << 8) | self.e as u16;

        memory_accessor.write(&address, &self.a);

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
    pub fn ld_nn_a(&mut self, memory_accessor: &mut dyn MemoryAccessor) -> u8 {
        let nh = self.fetch_next_opcode(memory_accessor);
        let nl = self.fetch_next_opcode(memory_accessor);
        let address = ((nh as u16) << 8) | nl as u16;
        memory_accessor.write(&address, &self.a);

        // T states
        13
    }
}

mod tests {
    use crate::z80::tests::Ram;

    use super::*;

    #[test]
    fn test_ld_r_rp() {
        #[rustfmt::skip]
        let scenarios: [(
            fn(&mut Z80) -> u8,
            fn(&mut Z80) -> &mut u8,
            fn(&mut Z80) -> &mut u8,
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

            *r_prime_supplier(z80) = 0xDD;

            let t_states = opcode(z80);
            assert_eq!(4, t_states);

            let r = r_supplier(z80);
            assert_eq!(0xDD, *r);
        }
    }

    #[test]
    fn test_ld_r_hl() {
        let scenarios: [(
            fn(&mut Z80, &dyn MemoryAccessor) -> u8,
            fn(&mut Z80) -> &mut u8,
        ); 7] = [
            (Z80::ld_a_hl, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_b_hl, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_c_hl, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_d_hl, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_e_hl, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_h_hl, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_l_hl, |z80: &mut Z80| &mut z80.l),
        ];

        let bytes = &mut [0xAA, 0xBB, 0xCC];
        let ram = Ram::new(bytes);

        for (opcode, register_supplier) in scenarios {
            let z80 = &mut Z80::new();
            z80.h = 0x00;
            z80.l = 0x02;

            let t_states = opcode(z80, &ram);
            assert_eq!(7, t_states);

            let register = register_supplier(z80);
            assert_eq!(ram.read(&2), *register);
        }
    }

    #[test]
    fn test_ld_r_ixd() {
        let scenarios: [(
            fn(&mut Z80, &dyn MemoryAccessor) -> u8,
            fn(&mut Z80) -> &mut u8,
        ); 7] = [
            (Z80::ld_a_ixd, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_b_ixd, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_c_ixd, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_d_ixd, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_e_ixd, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_h_ixd, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_l_ixd, |z80: &mut Z80| &mut z80.l),
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
            assert_eq!(0xCC, *register);
        }
    }

    #[test]
    fn test_ld_r_iyd() {
        let scenarios: [(
            fn(&mut Z80, &dyn MemoryAccessor) -> u8,
            fn(&mut Z80) -> &mut u8,
        ); 7] = [
            (Z80::ld_a_iyd, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_b_iyd, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_c_iyd, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_d_iyd, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_e_iyd, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_h_iyd, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_l_iyd, |z80: &mut Z80| &mut z80.l),
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
            assert_eq!(0xCC, *register);
        }
    }

    #[test]
    fn test_ld_hl_r() {
        let scenarios: [(
            fn(&Z80, &mut dyn MemoryAccessor) -> u8,
            fn(&mut Z80) -> &mut u8,
        ); 5] = [
            (Z80::ld_hl_a, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_hl_b, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_hl_c, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_hl_d, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_hl_e, |z80: &mut Z80| &mut z80.e),
            // (Z80::ld_hl_h, |z80: &mut Z80| &mut z80.h),
            // (Z80::ld_hl_l, |z80: &mut Z80| &mut z80.l),
        ];

        let bytes = &mut [0, 0, 0];
        let ram = &mut Ram::new(bytes);

        for (opcode, register_supplier) in scenarios {
            let z80 = &mut Z80::new();

            z80.h = 0x00;
            z80.l = 0x02;

            let register = register_supplier(z80);
            *register = 0xDD;

            let t_states = opcode(z80, ram);
            assert_eq!(7, t_states);

            assert_eq!(0xDD, ram.read(&2));
        }
    }

    #[test]
    fn test_ld_hl_h() {
        let bytes = &mut [5, 5, 5];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();

        z80.h = 0x00;
        z80.l = 0x02;

        let t_states = z80.ld_hl_h(ram);
        assert_eq!(7, t_states);

        assert_eq!(z80.h, ram.read(&2));
    }

    #[test]
    fn test_ld_hl_l() {
        let bytes = &mut [5, 5, 5];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();

        z80.h = 0x00;
        z80.l = 0x02;

        let t_states = z80.ld_hl_l(ram);
        assert_eq!(7, t_states);

        assert_eq!(z80.l, ram.read(&2));
    }

    #[test]
    fn test_ld_ixd_r() {
        let scenarios: [(
            fn(&mut Z80, &mut dyn MemoryAccessor) -> u8,
            fn(&mut Z80) -> &mut u8,
        ); 7] = [
            (Z80::ld_ixd_a, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_ixd_b, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_ixd_c, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_ixd_d, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_ixd_e, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_ixd_h, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_ixd_l, |z80: &mut Z80| &mut z80.l),
        ];

        let bytes = &mut [0xDD, 0x77, (-2i8).to_le_bytes()[0], 0x00];
        let ram = &mut Ram::new(bytes);

        for (opcode, register_supplier) in scenarios {
            let z80 = &mut Z80::new();
            z80.program_counter = 2;
            z80.ix = 5;
            *register_supplier(z80) = 0xFF;

            let t_states = opcode(z80, ram);
            assert_eq!(19, t_states);

            assert_eq!(0xFF, ram.read(&3));
        }
    }

    #[test]
    fn test_ld_iyd_r() {
        let scenarios: [(
            fn(&mut Z80, &mut dyn MemoryAccessor) -> u8,
            fn(&mut Z80) -> &mut u8,
        ); 7] = [
            (Z80::ld_iyd_a, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_iyd_b, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_iyd_c, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_iyd_d, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_iyd_e, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_iyd_h, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_iyd_l, |z80: &mut Z80| &mut z80.l),
        ];

        let bytes = &mut [0xDD, 0x77, (-2i8).to_le_bytes()[0], 0x00];
        let ram = &mut Ram::new(bytes);

        for (opcode, register_supplier) in scenarios {
            let z80 = &mut Z80::new();
            z80.program_counter = 2;
            z80.iy = 5;
            *register_supplier(z80) = 0xFF;

            let t_states = opcode(z80, ram);
            assert_eq!(19, t_states);

            assert_eq!(0xFF, ram.read(&3));
        }
    }

    #[test]
    fn test_ld_hl_n() {
        let bytes = &mut [0x36, 0xFF, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.h = 0x00;
        z80.l = 0x02;
        let t_states = z80.ld_hl_n(ram);

        assert_eq!(3, t_states);
        assert_eq!(0xFF, ram.read(&2));
    }

    #[test]
    fn test_ld_ixd_n() {
        let bytes = &mut [0xDD, 0x26, 0x02, 0xFF, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.ix = 0x02;
        let t_states = z80.ld_ixd_n(ram);

        assert_eq!(19, t_states);
        assert_eq!(0xFF, ram.read(&4));
    }

    #[test]
    fn test_ld_iyd_n() {
        let bytes = &mut [0xDD, 0x26, 0x02, 0xFF, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.iy = 0x02;
        let t_states = z80.ld_iyd_n(ram);

        assert_eq!(19, t_states);
        assert_eq!(0xFF, ram.read(&4));
    }

    #[test]
    fn test_ld_a_bc() {
        let bytes = &mut [0x0A, 0xFF];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.b = 0;
        z80.c = 1;
        let t_states = z80.ld_a_bc(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, z80.a);
    }

    #[test]
    fn test_ld_a_de() {
        let bytes = &mut [0x0A, 0xFF];
        let ram = &Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.d = 0;
        z80.e = 1;
        let t_states = z80.ld_a_de(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, z80.a);
    }

    #[test]
    fn test_ld_a_nn() {
        let bytes = &mut [0x3A, 0x00, 0x03, 0xFF];
        let ram = &Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        let t_states = z80.ld_a_nn(ram);

        assert_eq!(13, t_states);
        assert_eq!(0xFF, z80.a);
    }

    #[test]
    fn test_ld_bc_a() {
        let bytes = &mut [0x00, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.a = 0xFF;
        z80.b = 0x00;
        z80.c = 0x01;
        let t_states = z80.ld_bc_a(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, ram.read(&1));
    }

    #[test]
    fn test_ld_de_a() {
        let bytes = &mut [0x00, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.a = 0xFF;
        z80.d = 0x00;
        z80.e = 0x01;
        let t_states = z80.ld_de_a(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, ram.read(&1));
    }

    #[test]
    fn test_ld_nn_a() {
        let bytes = &mut [0x32, 0x00, 0x03, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.a = 0xFF;
        let t_states = z80.ld_nn_a(ram);

        assert_eq!(13, t_states);
        assert_eq!(0xFF, ram.read(&3));
    }
}
