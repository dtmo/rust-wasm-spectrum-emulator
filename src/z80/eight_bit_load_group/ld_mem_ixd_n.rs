use crate::z80::{Z80Memory, Z80};

impl Z80 {
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
    /// `11011101` (DD)
    /// `00110110` (36)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_mem_ixd_n() {
        let bytes = &mut [0xDD, 0x26, 0x02, 0xFF, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.pc = 2;
        z80.ix = 0x02;
        let t_states = z80.ld_mem_ixd_n(ram);

        assert_eq!(19, t_states);
        assert_eq!(0xFF, ram.read(4));
    }
}
