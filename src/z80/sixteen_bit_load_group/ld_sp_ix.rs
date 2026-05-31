use crate::z80::Z80;

impl Z80 {
    /// ## LD SP, IX
    /// ### Operation
    /// SP ← IX
    /// ### Op Code
    /// LD
    /// ### Operands
    /// SP, IX
    /// `1 1 0 1 1 1 0 1` (DD)
    /// `1 1 1 1 1 0 0 1` (F9)
    ///
    /// ### Description
    /// The 2-byte contents of Index Register IX are loaded to the Stack Pointer
    /// (SP).
    ///
    /// | M Cycles | T States  | 4 MHz E.T. |
    /// | -------- | --------- | ---------- |
    /// | 2        | 10 (4, 6) | 2.50       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If Index Register IX contains 98DAh, then upon the execution of an LD
    /// SP, IX instruction, the Stack Pointer also contains 98DAh.
    pub fn ld_sp_ix(&mut self) -> u8 {
        self.stack_pointer = self.ix;

        // T states
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ld_sp_ix() {
        let z80 = &mut Z80::new();
        z80.iy = 0x98DA;

        let t_states = z80.ld_sp_ix();
        assert_eq!(10, t_states);

        assert_eq!(z80.ix, z80.stack_pointer);
    }
}
