use crate::z80::{Z80Memory, Z80};

impl Z80 {
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
}

mod tests {
    use crate::z80::{tests::Ram, Z80, Z80Memory};

    #[test]
    fn test_ld_mem_hl_n() {
        let bytes = &mut [0x36, 0xFF, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.set_hl(0x0002);
        let t_states = z80.ld_mem_hl_n(ram);

        assert_eq!(3, t_states);
        assert_eq!(0xFF, ram.read(2));
    }
}
