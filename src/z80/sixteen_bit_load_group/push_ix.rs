use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## PUSH IX
    /// ### Operation
    /// (SP – 2) ← IXL, (SP – 1) ← IXH
    /// ### Op Code
    /// PUSH
    /// ### Operand
    /// IX
    /// `11011101` (DD)
    /// `11100101` (E5)
    /// ### Description
    /// The contents of Index Register IX are pushed to the external memory
    /// last-in, first-out (LIFO) stack. The Stack Pointer (SP) Register pair
    /// holds the 16-bit address of the current top of the Stack. This
    /// instruction first decrements SP and loads the high-order byte of IX to
    /// the memory address specified by SP; then decrements SP again and loads
    /// the low-order byte to the memory location corresponding to this new
    /// address in SP.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 15 (4, 5, 3, 3) | 3.75       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If Index Register IX contains 2233h and the Stack Pointer contains
    /// 1007h, then upon the execution of a PUSH IX instruction, memory address
    /// 1006h contains 22h, memory address 1005h contains 33h, and the Stack
    /// Pointer contains 1005h.
    pub fn push_ix(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let ix_high: u8 = (self.ix >> 8) as u8;
        self.sp = self.sp.wrapping_sub(1);
        mem.write(self.sp, ix_high);

        let ix_low: u8 = self.ix as u8;
        self.sp = self.sp.wrapping_sub(1);
        mem.write(self.sp, ix_low);

        // T states
        15
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_push_ix() {
        let bytes = &mut [0xDD, 0xE5, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.sp = 4;
        z80.ix = 0x2233;

        let t_states = z80.push_ix(ram);
        assert_eq!(15, t_states);

        assert_eq!(0x22, ram.read(3));
        assert_eq!(0x33, ram.read(2));
        assert_eq!(2, z80.sp);
    }
}
