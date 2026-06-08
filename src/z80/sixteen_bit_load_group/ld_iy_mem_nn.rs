use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD IY, (nn)
    ///
    /// ### Operation
    ///
    /// IYh ← (nn + 1), IYI ← (nn)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// IY, (nn)
    /// `11111101` (FD)
    /// `00101010` (2A)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The contents of the address (nn) are loaded to the low-order portion of
    /// Index Register IY, and the contents of the next highest memory address
    /// (nn + 1) are loaded to the high-order portion of IY. The first n operand
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
    /// the execution of an LD IY, (6666h) instruction, Index Register IY
    /// contains DA92h.
    pub fn ld_iy_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        let val_low = mem.read(address);
        let val_high = mem.read(address + 1);

        self.iy = ((val_high as u16) << 8) | val_low as u16;

        // T states
        20
    }
}

#[cfg(test)]
mod tests {
    use crate::z80::{Z80, tests::Ram};

    #[test]
    fn test_example() {
        let z80 = &mut Z80::new();
        
        // If address 0004h contains 92h,
        // and address 0005h contains DAh,
        let bytes = &mut [0xFD, 0x2A, 0x04, 0x00, 0x92, 0xDA];
        let ram = &mut Ram::new(bytes);

        z80.pc = 2;
        
        // then upon the execution of an LD IY, (0004h) instruction,
        let t_states = z80.ld_iy_mem_nn(ram);
        assert_eq!(20, t_states);

        // Index Register IY contains DA92h.
        assert_eq!(0xDA92, z80.iy);
    }
}
