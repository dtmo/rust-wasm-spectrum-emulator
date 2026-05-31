use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD A, (BC)
    ///
    /// ### Operation
    ///
    /// A ← (BC)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// A, (BC)
    /// `0 0 0 0 1 0 1 0` (AA)
    ///
    /// ### Description
    ///
    /// The contents of the memory location specified by the contents of the BC
    /// register pair are loaded to the Accumulator.
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
    /// If the BC register pair contains the number 4747h, and memory address
    /// 4747h contains byte 12h, then the instruction LD A, (BC) results in byte
    /// 12h in Register A.
    pub fn ld_a_mem_bc(&mut self, mem: &dyn Z80Memory) -> u8 {
        self.a.set_value(mem.read(self.bc()));

        // T states
        7
    }
}

mod tests {
    use crate::z80::{tests::Ram, Z80};

    #[test]
    fn test_ld_a_mem_bc() {
        let bytes = &mut [0x0A, 0xFF];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.set_bc(0x0001);

        let t_states = z80.ld_a_mem_bc(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, z80.a.value());
    }
}
