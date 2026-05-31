use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD (nn), IX
    ///
    /// ### Operation
    ///
    /// (nn + 1) ← IXh, (nn) ← IXI
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (nn), IX
    /// `1 1 0 1 1 1 0 1` (DD)
    /// `0 0 1 0 0 0 1 0` (22)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The low-order byte in Index Register IX is loaded to memory address
    /// (nn); the upper order byte is loaded to the next highest address (nn +
    /// 1). The first n operand after the op code is the low-order byte of nn.
    ///
    /// | M Cycles | T States           | 4 MHz E.T. |
    /// | -------- | ------------------ | ---------- |
    /// | 0        | (4, 4, 3, 3, 3, 3) | 5.00       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If Index Register IX contains 5A30h, then upon the execution of an LD
    /// (4392h), IX instruction, memory location 4392h contains number 30h and
    /// location 4393h contains 5Ah.
    pub fn ld_mem_nn_ix(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let address_low = self.fetch_next_opcode(mem);
        let address_high = self.fetch_next_opcode(mem);

        let address = ((address_high as u16) << 8) | address_low as u16;

        let ix_low: u8 = self.ix as u8;
        let ix_high: u8 = (self.ix >> 8) as u8;
        mem.write(address, ix_low);
        mem.write(address.wrapping_add(1), ix_high);

        // T states
        20
    }
}

mod tests {
    use crate::z80::{tests::Ram, Z80Memory, Z80};

    #[test]
    fn test_ld_mem_nn_ix() {
        let bytes = &mut [0xDD, 0x22, 0x04, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.ix = 0xEEFF;

        let t_states = z80.ld_mem_nn_ix(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xFF, ram.read(4));
        assert_eq!(0xEE, ram.read(5));
    }
}
