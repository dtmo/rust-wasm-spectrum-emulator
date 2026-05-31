use crate::z80::{
    register_flags::{
        s_flag, set_p_flag_with, set_s_flag_with, set_z_flag_with, unset_h_flag, unset_n_flag,
    },
    Z80,
};

impl Z80 {
    /// ## LD A, R
    ///
    /// ### Operation
    ///
    /// A ← R
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// A, R
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `0 1 0 1 1 1 1 1` (5F)
    ///
    /// ### Description
    ///
    /// The contents of Memory Refresh Register R are loaded to the Accumulator.
    ///
    /// | M Cycles | T States | MHz E.T. |
    /// | -------- | -------- | -------- |
    /// | 2        | 9 (4, 5) | 2.25     |
    ///
    /// ### Condition Bits Affected
    ///
    /// S is set if, R-Register is negative; otherwise, it is reset.
    /// Z is set if the R Register is 0; otherwise, it is reset.
    /// H is reset.
    /// P/V contains contents of IFF2.
    /// N is reset.
    /// C is not affected.
    /// If an interrupt occurs during execution of this instruction, the parity
    /// contains a 0.
    pub fn ld_a_r(&mut self) -> u8 {
        self.a.set_value(self.r.value());

        // S is set if the R-Register is negative; otherwise, it is reset.
        set_s_flag_with(&mut self.f, s_flag(&self.r));

        // Z is set if the R Register is 0; otherwise, it is reset.
        set_z_flag_with(&mut self.f, self.r.value() == 0);

        // H is reset.
        unset_h_flag(&mut self.f);

        // P/V contains contents of IFF2.
        set_p_flag_with(&mut self.f, self.iff2);

        // N is reset.
        unset_n_flag(&mut self.f);

        // TODO: If an interrupt occurs during execution of this instruction,
        // the Parity flag contains a 0.

        // T states
        9
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::register_flags::{h_flag, n_flag, p_flag, s_flag, z_flag};

    #[test]
    fn test_ld_a_r_positive() {
        let z80 = &mut Z80::new();
        z80.r.set_value(0b01111111);
        let t_states = z80.ld_a_r();
        assert_eq!(9, t_states);

        assert_eq!(z80.r, z80.a);

        // S is set if the R Register is negative; otherwise, it is reset.
        assert_eq!(false, s_flag(&z80.f));

        // Z is set if the R Register is 0; otherwise, it is reset.
        assert_eq!(false, z_flag(&z80.f));

        // H is reset.
        assert_eq!(false, h_flag(&z80.f));

        // P/V contains contents of IFF2.
        assert_eq!(z80.iff2, p_flag(&z80.f));

        // N is reset.
        assert_eq!(false, n_flag(&z80.f));

        // TODO: If an interrupt occurs during execution of this instruction, the Parity flag contains a 0.
    }

    #[test]
    fn test_ld_a_r_zero() {
        let z80 = &mut Z80::new();
        z80.r.set_value(0);
        let t_states = z80.ld_a_r();
        assert_eq!(9, t_states);

        assert_eq!(z80.r, z80.a);

        // S is set if the R Register is negative; otherwise, it is reset.
        assert_eq!(false, s_flag(&z80.f));

        // Z is set if the R Register is 0; otherwise, it is reset.
        assert_eq!(true, z_flag(&z80.f));

        // H is reset.
        assert_eq!(false, h_flag(&z80.f));

        // P/V contains contents of IFF2.
        assert_eq!(z80.iff2, p_flag(&z80.f));

        // N is reset.
        assert_eq!(false, n_flag(&z80.f));

        // TODO: If an interrupt occurs during execution of this instruction, the Parity flag contains a 0.
    }

    #[test]
    fn test_ld_a_r_negative() {
        let z80 = &mut Z80::new();
        z80.r.set_value(0b11111111);
        let t_states = z80.ld_a_r();
        assert_eq!(9, t_states);

        assert_eq!(z80.r, z80.a);

        // S is set if the R Register is negative; otherwise, it is reset.
        assert_eq!(true, s_flag(&z80.f));

        // Z is set if the R Register is 0; otherwise, it is reset.
        assert_eq!(false, z_flag(&z80.f));

        // H is reset.
        assert_eq!(false, h_flag(&z80.f));

        // P/V contains contents of IFF2.
        assert_eq!(z80.iff2, p_flag(&z80.f));

        // N is reset.
        assert_eq!(false, n_flag(&z80.f));

        // TODO: If an interrupt occurs during execution of this instruction, the Parity flag contains a 0.
    }
}
