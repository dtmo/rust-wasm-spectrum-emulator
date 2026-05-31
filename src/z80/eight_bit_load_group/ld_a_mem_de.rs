use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD A, (DE)
    ///
    /// ### Operation
    ///
    /// A ← (DE)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// A, (DE)
    /// `0 0 0 1 1 0 1 0` (AA)
    ///
    /// ### Description
    ///
    /// The contents of the memory location specified by the contents of the DE
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
    /// If the DE register pair contains the number 30A2h, and memory address
    /// 30A2h contains byte 22h, then the instruction LD A, (DE) results in byte
    /// 22h in Register A.
    pub fn ld_a_mem_de(&mut self, mem: &dyn Z80Memory) -> u8 {
        self.a.set_value(mem.read(self.de()));

        // T states
        7
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_a_mem_de() {
        let bytes = &mut [0x0A, 0xFF];
        let ram = &Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.set_de(0x0001);

        let t_states = z80.ld_a_mem_de(ram);

        assert_eq!(7, t_states);
        assert_eq!(0xFF, z80.a.value());
    }
}
