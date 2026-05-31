use crate::z80::{Register, Z80Memory, Z80};

impl Z80 {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_mem_ixd_r() {
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
}
