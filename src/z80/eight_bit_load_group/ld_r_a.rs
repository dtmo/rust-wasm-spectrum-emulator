use crate::z80::Z80;

impl Z80 {
    /// ## LD R,A
    ///
    /// ### Operation
    ///
    /// R ← A
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// R, A
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `0 1 0 0 1 1 1 1` (47)
    ///
    /// ### Description
    ///
    /// The contents of the Accumulator are loaded to the Memory Refresh
    /// register R.
    ///
    /// | M Cycles | T States | MHz E.T. |
    /// | -------- | -------- | -------- |
    /// | 2        | 9 (4, 5) | 2.25     |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    pub fn ld_r_a(&mut self) -> u8 {
        self.r.set_value(self.a.value());

        // T states
        9
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ld_r_a() {
        let z80 = &mut Z80::new();
        z80.a.set_value(0xFF);

        let t_states = z80.ld_r_a();
        assert_eq!(9, t_states);

        assert_eq!(0xFF, z80.r.value());
    }
}
