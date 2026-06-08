use crate::z80::Z80;

impl Z80 {
    /// ## Operation
    ///
    /// —
    ///
    /// ### Op Code
    ///
    /// NOP
    ///
    /// `00000000` 00
    ///
    /// ### Operands
    ///
    /// None.
    ///
    /// ### Description
    ///
    /// The CPU performs no operation during this machine cycle.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 1        | 4        | 1.00       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    pub fn nop(&mut self) -> u8 {
        // T States
        4
    }
}
