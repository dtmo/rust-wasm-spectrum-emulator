use crate::z80::Z80;

impl Z80 {
    /// ## LD SP, IY
    /// ### Operation
    /// SP ← IY
    /// ### Op Code
    /// LD
    /// ### Operands
    /// SP, IY
    /// `1 1 0 1 1 1 0 1` (FD)
    /// `1 1 1 1 1 0 0 1` (F9)
    ///
    /// ### Description
    /// The 2-byte contents of Index Register IY are loaded to the Stack Pointer
    /// (SP).
    ///
    /// | M Cycles | T States  | 4 MHz E.T. |
    /// | -------- | --------- | ---------- |
    /// | 2        | 10 (4, 6) | 2.50       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If Index Register IY contains A227h, then upon the execution of an LD
    /// SP, IY instruction, the Stack Pointer also contains A227h.
    pub fn ld_sp_iy(&mut self) -> u8 {
        self.stack_pointer = self.iy;

        // T states
        10
    }
}

mod tests {
    use crate::z80::Z80;

    #[test]
    fn test_ld_sp_iy() {
        let z80 = &mut Z80::new();
        z80.iy = 0xA227;

        let t_states = z80.ld_sp_iy();
        assert_eq!(10, t_states);

        assert_eq!(z80.iy, z80.stack_pointer);
    }
}
