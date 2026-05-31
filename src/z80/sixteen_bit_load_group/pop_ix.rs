use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## POP IX
    /// ### Operation
    /// IXH ← (SP+1), IXL ← (SP)
    /// ### Op Code
    /// POP
    /// ### Operand
    /// IX
    /// `1 1 0 1 1 1 0 1` (DD)
    /// `1 1 1 0 0 0 0 1` (E1)
    /// ### Description
    /// The top two bytes of the external memory last-in, first-out (LIFO) stack
    /// are popped to Index Register IX. The Stack Pointer (SP) Register pair
    /// holds the 16-bit address of the current top of the Stack. This
    /// instruction first loads to the low-order portion of IX the byte at the
    /// memory location corresponding to the contents of SP; then SP is
    /// incremented and the contents of the corresponding adjacent memory
    /// location are loaded to the high-order portion of IX. The SP is
    /// incremented again.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 14 (4, 4, 3, 3) | 3.50       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If the Stack Pointer contains 1000h, memory location 1000h contains 55h,
    /// and location 1001h contains 33h, the instruction POP IX results in Index
    /// Register IX containing 3355h, and the Stack Pointer containing 1002h.
    pub fn pop_ix(&mut self, mem: &dyn Z80Memory) -> u8 {
        let ix_low = mem.read(self.stack_pointer);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        let ix_high = mem.read(self.stack_pointer);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        self.ix = ((ix_high as u16) << 8) | ix_low as u16;

        // T states
        14
    }
}

mod tests {
    use crate::z80::{tests::Ram, Z80Memory, Z80};

    #[test]
    fn test_pop_ix() {
        let bytes = &mut [0xDD, 0xE1, 0x55, 0x33];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.stack_pointer = 2;

        let t_states = z80.pop_ix(ram);
        assert_eq!(14, t_states);

        assert_eq!(4, z80.stack_pointer);
        assert_eq!(0x3355, z80.ix);
    }
}
