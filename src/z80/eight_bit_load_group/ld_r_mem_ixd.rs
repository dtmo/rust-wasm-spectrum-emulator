use crate::z80::{Register, Z80Memory, Z80};

impl Z80 {
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
}

mod tests {
    use crate::z80::{tests::Ram, Register, Z80Memory, Z80};

    #[test]
    fn test_ld_r_mem_ixd() {
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
}
