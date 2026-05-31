use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## PUSH IY
    /// ### Operation
    /// (SP – 2) ← IYL, (SP – 1) ← IYH
    /// ### Op Code
    /// PUSH
    /// ### Operand
    /// IY
    /// `1 1 1 1 1 1 0 1` (FD)
    /// `1 1 1 0 0 1 0 1` (E5)
    /// ### Description
    /// The contents of Index Register IY are pushed to the external memory
    /// last-in, first-out (LIFO) stack. The Stack Pointer (SP) Register pair
    /// holds the 16-bit address of the current top of the Stack. This
    /// instruction first decrements SP and loads the high-order byte of IY to
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
    /// If Index Register IY contains 2233h and the Stack Pointer contains
    /// 1007h, then upon the execution of a PUSH IY instruction, memory address
    /// 1006h contains 22h, memory address 1005h contains 33h, and the Stack
    /// Pointer contains 1005h.
    pub fn push_iy(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let iy_high: u8 = (self.iy >> 8) as u8;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        mem.write(self.stack_pointer, iy_high);

        let iy_low: u8 = self.iy as u8;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        mem.write(self.stack_pointer, iy_low);

        // T states
        15
    }
}

mod tests {
    use crate::z80::{tests::Ram, Z80Memory, Z80};

    #[test]
    fn test_push_iy() {
        let bytes = &mut [0xFD, 0xE5, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.stack_pointer = 4;
        z80.iy = 0x2233;

        let t_states = z80.push_iy(ram);
        assert_eq!(15, t_states);

        assert_eq!(0x22, ram.read(3));
        assert_eq!(0x33, ram.read(2));
        assert_eq!(2, z80.stack_pointer);
    }
}
