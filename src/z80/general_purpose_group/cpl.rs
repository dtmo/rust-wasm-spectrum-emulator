use crate::z80::{Z80, register_flags::{set_h_flag, set_n_flag}};

impl Z80 {
    /// ## CPL
    ///
    /// ### Operation
    ///
    /// A ← A
    ///
    /// ### Op Code
    ///
    /// CPL
    ///
    /// ### Operands
    ///
    /// None.
    ///
    /// `00101111` (2F)
    ///
    /// ### Description
    ///
    /// The contents of the Accumulator (Register A) are inverted (one’s complement).
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 1        | 4        | 1.00       |
    ///
    /// ### Condition Bits Affected
    ///
    /// * S is not affected.
    /// * Z is not affected.
    /// * H is set.
    /// * P/V is not affected.
    /// * N is set.
    /// * C is not affected.
    ///
    /// ### Example
    ///
    /// If the Accumulator contains 1011 0100, then upon the execution of a CPL instruction, the
    /// Accumulator contains 0100 1011.
    pub fn cpl(&mut self) -> u8 {
        let a = self.a();
        self.set_a(!a);

        set_h_flag(&mut self.f);
        set_n_flag(&mut self.f);

        // T States
        4
    }
}

#[cfg(test)]
mod tests {
    use crate::z80::Z80;

    #[test]
    fn test_example() {
        let mut z80 = Z80::new();

        // If the Accumulator contains 1011 0100,
        z80.set_a(0b10110100);
        
        // then upon the execution of a CPL instruction,
        let t_states = z80.cpl();

        assert_eq!(4, t_states);
        
        // the Accumulator contains 0100 1011.
        assert_eq!(0b01001011, z80.a());
    }
}
