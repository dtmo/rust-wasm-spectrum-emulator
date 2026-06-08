use crate::z80::{
    register_flags::{
        set_c_flag_with, set_h_flag_with, set_n_flag, set_p_flag_with, set_s_flag_with,
        set_z_flag_with,
    },
    Z80,
};

impl Z80 {
    /// ## NEG
    ///
    /// ### Operation
    ///
    /// A ← 0 – A
    ///
    /// ### Op Code
    ///
    /// NEG
    ///
    /// `11101101` (ED)
    /// `01000100` (44)
    ///
    /// ### Operands
    ///
    /// None.
    ///
    /// ### Description
    ///
    /// The contents of the Accumulator are negated (two’s complement). This method is the
    /// same as subtracting the contents of the Accumulator from zero.
    ///
    /// Note: The 80h address remains unchanged.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 2        | 8 (4, 4) | 2.00       |
    ///
    /// ### Condition Bits Affected
    ///
    /// * S is set if result is negative; otherwise, it is reset.
    /// * Z is set if result is 0; otherwise, it is reset.
    /// * H is set if borrow from bit 4; otherwise, it is reset.
    /// * P/V is set if Accumulator was 80h before operation; otherwise, it is reset.
    /// * N is set.
    /// * C is set if Accumulator was not 00h before operation; otherwise, it is reset.
    ///
    /// ### Example
    ///
    /// The Accumulator contains the following data:
    ///
    /// `10011000`
    ///
    /// Upon the execution of a NEG instruction, the Accumulator contains:
    ///
    /// `01101000`
    pub fn neg(&mut self) -> u8 {
        let a = self.a();
        let neg_a = a.wrapping_neg();
        self.set_a(neg_a);

        // S is set if result is negative; otherwise, it is reset.
        set_s_flag_with(&mut self.f, (neg_a & 0b10000000) == 0b10000000);

        // Z is set if result is 0; otherwise, it is reset.
        set_z_flag_with(&mut self.f, neg_a == 0);

        // H is set if borrow from bit 4; otherwise, it is reset.
        let half_carry_borrow = ((0_u8 & 0x0f).wrapping_sub(a & 0x0f)) & 0x10 == 0x10;
        set_h_flag_with(&mut self.f, half_carry_borrow);

        // P/V is set if Accumulator was 80h before operation; otherwise, it is reset.
        set_p_flag_with(&mut self.f, a == 0x80);

        // N is set.
        set_n_flag(&mut self.f);

        // C is set if Accumulator was not 00h before operation; otherwise, it is reset.
        set_c_flag_with(&mut self.f, a != 0);

        // T States
        8
    }
}

#[cfg(test)]
mod tests {
    use crate::z80::Z80;

    #[test]
    fn test_example() {
        let mut z80 = Z80::new();
        
        // The Accumulator contains the following data: 10011000
        z80.set_a(0b10011000);

        // Upon the execution of a NEG instruction,
        let t_states = z80.neg();
        assert_eq!(8, t_states);

        // the Accumulator contains: 01101000
        assert_eq!(0b01101000, z80.a());
    }
}
