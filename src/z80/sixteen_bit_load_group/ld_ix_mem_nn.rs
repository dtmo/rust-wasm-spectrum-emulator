use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD IX, (nn)
    ///
    /// ### Operation
    ///
    /// IXh ← (nn + 1), IXI ← (nn)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// IX, (nn)
    /// `1 1 0 1 1 1 0 1` (DD)
    /// `0 0 1 0 1 0 1 0` (2A)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The contents of the address (nn) are loaded to the low-order portion of
    /// Index Register IX, and the contents of the next highest memory address
    /// (nn + 1) are loaded to the high-order portion of IX. The first n operand
    /// after the op code is the low-order byte of nn.
    ///
    /// | M Cycles | T States 4            | MHz E.T. |
    /// | -------- | --------------------- | -------- |
    /// | 6        | 20 (4, 4, 3, 3, 3, 3) | 5.00     |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If address 6666h contains 92h, and address 6667h contains DAh, then upon
    /// the execution of an LD IX, (6666h) instruction, Index Register IX
    /// contains DA92h.
    pub fn ld_ix_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        let val_low = mem.read(address);
        let val_high = mem.read(address.wrapping_add(1));

        self.ix = ((val_high as u16) << 8) | val_low as u16;

        // T states
        20
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_ix_mem_nn() {
        let bytes = &mut [0xDD, 0x2A, 0x04, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;

        let t_states = z80.ld_ix_mem_nn(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xF00F, z80.ix);
    }
}
