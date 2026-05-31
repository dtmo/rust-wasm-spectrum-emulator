use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD (nn), A
    ///
    /// ### Operation
    ///
    /// (nn) ← A
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (nn), A
    /// `0 0 1 1 0 0 1 0` (32)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The contents of the Accumulator are loaded to the memory address
    /// specified by the operand nn. The first n operand after the op code is
    /// the low-order byte of nn.
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
    /// If the Accumulator contains byte D7h, then executing an LD (3141h), AD7h
    /// instruction results in memory location 3141h.
    pub fn ld_mem_nn_a(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let nl = self.fetch_next_opcode(mem);
        let nh = self.fetch_next_opcode(mem);
        let address = ((nh as u16) << 8) | nl as u16;
        mem.write(address, self.a.value());

        // T states
        13
    }
}

mod tests {
    use crate::z80::{tests::Ram, Z80Memory, Z80};

    #[test]
    fn test_ld_mem_nn_a() {
        let bytes = &mut [0x32, 0x03, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.a.set_value(0xFF);
        let t_states = z80.ld_mem_nn_a(ram);

        assert_eq!(13, t_states);
        assert_eq!(0xFF, ram.read(3));
    }
}
