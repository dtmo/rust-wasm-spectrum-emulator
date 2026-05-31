use crate::z80::{Z80Memory, Z80};

impl Z80 {
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
    pub fn ld_mem_bc_a(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        mem.write(self.bc(), self.a.value());

        // T states
        7
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_mem_bc_a() {
        let bytes = &mut [0x00, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.a.set_value(0xFF);
        z80.set_bc(0x0001);
        let t_states = z80.ld_mem_bc_a(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, ram.read(1));
    }
}
