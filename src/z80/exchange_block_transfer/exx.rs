use crate::z80::Z80;

impl Z80 {
    /// ## EXX
    /// ### Operation
    /// (BC) ↔ (BC′), (DE) ↔ (DE'), (HL) ↔ (HL′)
    /// ### Op Code
    /// EXX
    /// ### Operands
    /// None.
    /// `11011001` (D9)
    /// ### Description
    /// Each 2-byte value in register pairs BC, DE, and HL is exchanged with the
    /// 2-byte value in BC', DE', and HL', respectively.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 1        | 4        | 1.00       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If register pairs BC, DE, and HL contain 445Ah, 3DA2h, and 8859h,
    /// respectively, and register pairs BC’, DE’, and HL’ contain 0988h, 9300h,
    /// and 00E7h, respectively, then upon the execution of an EXX instruction,
    /// BC contains 0988h; DE contains 9300h; HL contains 00E7h; BC’ contains
    /// 445Ah; DE’ contains 3DA2h; and HL’ contains 8859h.
    pub fn exx(&mut self) -> u8 {
        let temp_b = self.b.value();
        let temp_c = self.c.value();
        let temp_d = self.d.value();
        let temp_e = self.e.value();
        let temp_h = self.h.value();
        let temp_l = self.l.value();
        let temp_b_prime = self.b_prime.value();
        let temp_c_prime = self.c_prime.value();
        let temp_d_prime = self.d_prime.value();
        let temp_e_prime = self.e_prime.value();
        let temp_h_prime = self.h_prime.value();
        let temp_l_prime = self.l_prime.value();

        self.b_prime.set_value(temp_b);
        self.c_prime.set_value(temp_c);
        self.d_prime.set_value(temp_d);
        self.e_prime.set_value(temp_e);
        self.h_prime.set_value(temp_h);
        self.l_prime.set_value(temp_l);
        self.b.set_value(temp_b_prime);
        self.c.set_value(temp_c_prime);
        self.d.set_value(temp_d_prime);
        self.e.set_value(temp_e_prime);
        self.h.set_value(temp_h_prime);
        self.l.set_value(temp_l_prime);

        // T states
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exx() {
        let mut z80 = Z80::new();
        z80.b.set_value(0x44);
        z80.c.set_value(0x5A);
        z80.d.set_value(0x3D);
        z80.e.set_value(0xA2);
        z80.h.set_value(0x88);
        z80.l.set_value(0x59);

        z80.b_prime.set_value(0x09);
        z80.c_prime.set_value(0x88);
        z80.d_prime.set_value(0x93);
        z80.e_prime.set_value(0x00);
        z80.h_prime.set_value(0x00);
        z80.l_prime.set_value(0xE7);

        let t_states = z80.exx();

        assert_eq!(4, t_states);

        assert_eq!(0x09, z80.b.value());
        assert_eq!(0x88, z80.c.value());
        assert_eq!(0x93, z80.d.value());
        assert_eq!(0x00, z80.e.value());
        assert_eq!(0x00, z80.h.value());
        assert_eq!(0xE7, z80.l.value());

        assert_eq!(0x44, z80.b_prime.value());
        assert_eq!(0x5A, z80.c_prime.value());
        assert_eq!(0x3D, z80.d_prime.value());
        assert_eq!(0xA2, z80.e_prime.value());
        assert_eq!(0x88, z80.h_prime.value());
        assert_eq!(0x59, z80.l_prime.value());
    }
}
