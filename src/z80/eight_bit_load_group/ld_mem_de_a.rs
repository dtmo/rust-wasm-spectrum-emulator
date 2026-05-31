use crate::z80::{Z80Memory, Z80};

impl Z80 {
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
}

mod tests {
    use crate::z80::{tests::Ram, Z80Memory, Z80};

    #[test]
    fn test_ld_mem_de_a() {
        let bytes = &mut [0x00, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.a.set_value(0xFF);
        z80.set_de(0x0001);
        let t_states = z80.ld_mem_de_a(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, ram.read(1));
    }
}
