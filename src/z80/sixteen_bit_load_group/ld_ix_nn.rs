use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD IX, nn
    ///
    /// ### Operation
    ///
    /// IX ← nn
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// IX, nn
    /// `11011101` (DD)
    /// `00100001` (21)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The nn integer is loaded to Index Register IX. The first n operand after
    /// the op code is the low-order byte.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 14 (4, 4, 3, 3) | 3.50       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// Upon the execution of an LD IX, 45A2h instruction, the index register
    /// contains integer 45A2h.
    pub fn ld_ix_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        self.ix = ((high_n as u16) << 8) | low_n as u16;

        // T states
        14
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_ix_nn() {
        let bytes = &mut [0x02, 0x01];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();

        let t_states = z80.ld_ix_nn(ram);
        assert_eq!(14, t_states);

        assert_eq!(0x0102, z80.ix);
    }
}
