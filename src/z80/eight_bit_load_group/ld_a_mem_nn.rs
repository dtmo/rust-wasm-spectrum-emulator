use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD A, (nn)
    ///
    /// ### Operation
    ///
    /// A ← (nn)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// A, (nn)
    /// `0 0 1 1 1 0 1 0` (3A)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    /// The contents of the memory location specified by the operands nn are loaded
    /// to the Accumulator. The first n operand after the op code is the low-order
    /// byte of a 2-byte memoryaddress.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 13 (4, 3, 3, 3) | 3.25       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If nn contains 8832h and memory address 8832h contains byte 04h, then upon
    /// the execution of an LD A, (nn) instruction, the 04h byte is in the
    /// Accumulator.
    pub fn ld_a_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let nl = self.fetch_next_opcode(mem);
        let nh = self.fetch_next_opcode(mem);

        let address = ((nh as u16) << 8) | nl as u16;
        self.a.set_value(mem.read(address));

        // T states
        13
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_a_mem_nn() {
        let bytes = &mut [0x3A, 0x03, 0x00, 0xFF];
        let ram = &Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        let t_states = z80.ld_a_mem_nn(ram);

        assert_eq!(13, t_states);
        assert_eq!(0xFF, z80.a.value());
    }
}
