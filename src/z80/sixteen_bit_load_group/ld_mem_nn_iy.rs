use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD (nn), IY
    ///
    /// ### Operation
    ///
    /// (nn + 1) ← IYh, (nn) ← IYI
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (nn), IY
    /// `1 1 0 1 1 1 0 1` (FD)
    /// `0 0 1 0 0 0 1 0` (22)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The low-order byte in Index Register IY is loaded to memory address
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
    /// If Index Register IY contains 4174h, then upon the execution of an LD
    /// (8838h), IY instruction, memory location 8838h contains number 74h and
    /// location 8839h contains 41h.
    pub fn ld_mem_nn_iy(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let address_low = self.fetch_next_opcode(mem);
        let address_high = self.fetch_next_opcode(mem);

        let address = ((address_high as u16) << 8) | address_low as u16;

        let iy_low: u8 = self.iy as u8;
        let iy_high: u8 = (self.iy >> 8) as u8;
        mem.write(address, iy_low);
        mem.write(address.wrapping_add(1), iy_high);

        // T states
        20
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_mem_nn_iy() {
        let bytes = &mut [0xFD, 0x22, 0x04, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.iy = 0xEEFF;

        let t_states = z80.ld_mem_nn_iy(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xFF, ram.read(4));
        assert_eq!(0xEE, ram.read(5));
    }
}
