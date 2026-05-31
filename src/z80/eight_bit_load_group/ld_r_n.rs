use crate::z80::{Register, Z80Memory, Z80};

impl Z80 {
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
}

mod tests {
    // TODO: Implement ld_r_n tests
}
