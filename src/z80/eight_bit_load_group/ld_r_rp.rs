use crate::z80::{Register, Z80};

impl Z80 {
    /// ## LD r, r'
    ///
    /// ### Operation
    ///
    /// r, ← r′
    ///
    /// ### Op Code
    ///
    /// LD: `0 1 r r r r' r' r'`
    ///
    /// ### Operands
    ///
    /// r, r′
    ///
    /// ### Description
    ///
    /// The contents of any register r' are loaded to any other register r.
    /// r, r' identifies any of the registers A, B, C, D, E, H, or L, assembled
    /// as follows in the object code:
    ///
    /// | Register | r, C |
    /// | -------- | ---- |
    /// | A        | 111  |
    /// | B        | 000  |
    /// | C        | 001  |
    /// | D        | 010  |
    /// | E        | 011  |
    /// | H        | 100  |
    /// | L        | 101  |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the H Register contains the number 8Ah, and the E register contains 10h, the instruction
    /// LD H, E results in both registers containing 10h.
    ///
    fn ld_r_rp(r: &mut Register, r_prime: u8) -> u8 {
        r.set_value(r_prime);

        // T states
        4
    }

    pub fn ld_b_b(&mut self) -> u8 {
        let b = self.b.value();
        Z80::ld_r_rp(&mut self.b, b)
    }

    pub fn ld_b_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.c.value())
    }

    pub fn ld_b_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.d.value())
    }

    pub fn ld_b_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.e.value())
    }

    pub fn ld_b_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.h.value())
    }

    pub fn ld_b_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.l.value())
    }

    pub fn ld_b_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.b, self.a.value())
    }

    pub fn ld_c_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.b.value())
    }

    pub fn ld_c_c(&mut self) -> u8 {
        let c = self.c.value();
        Z80::ld_r_rp(&mut self.c, c)
    }

    pub fn ld_c_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.d.value())
    }

    pub fn ld_c_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.e.value())
    }

    pub fn ld_c_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.h.value())
    }

    pub fn ld_c_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.l.value())
    }

    pub fn ld_c_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.c, self.a.value())
    }

    pub fn ld_d_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.b.value())
    }

    pub fn ld_d_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.c.value())
    }

    pub fn ld_d_d(&mut self) -> u8 {
        let d = self.d.value();
        Z80::ld_r_rp(&mut self.d, d)
    }

    pub fn ld_d_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.e.value())
    }

    pub fn ld_d_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.h.value())
    }

    pub fn ld_d_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.l.value())
    }

    pub fn ld_d_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.d, self.a.value())
    }

    pub fn ld_e_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.b.value())
    }

    pub fn ld_e_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.c.value())
    }

    pub fn ld_e_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.d.value())
    }

    pub fn ld_e_e(&mut self) -> u8 {
        let e = self.e.value();
        Z80::ld_r_rp(&mut self.e, e)
    }

    pub fn ld_e_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.h.value())
    }

    pub fn ld_e_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.l.value())
    }

    pub fn ld_e_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.e, self.a.value())
    }

    pub fn ld_h_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.b.value())
    }

    pub fn ld_h_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.c.value())
    }

    pub fn ld_h_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.d.value())
    }

    pub fn ld_h_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.e.value())
    }

    pub fn ld_h_h(&mut self) -> u8 {
        let h = self.h.value();
        Z80::ld_r_rp(&mut self.h, h)
    }

    pub fn ld_h_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.l.value())
    }

    pub fn ld_h_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.h, self.a.value())
    }

    pub fn ld_l_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.b.value())
    }

    pub fn ld_l_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.c.value())
    }

    pub fn ld_l_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.d.value())
    }

    pub fn ld_l_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.e.value())
    }

    pub fn ld_l_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.h.value())
    }

    pub fn ld_l_l(&mut self) -> u8 {
        let l = self.l.value();
        Z80::ld_r_rp(&mut self.l, l)
    }

    pub fn ld_l_a(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.l, self.a.value())
    }

    pub fn ld_a_b(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.b.value())
    }

    pub fn ld_a_c(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.c.value())
    }

    pub fn ld_a_d(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.d.value())
    }

    pub fn ld_a_e(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.e.value())
    }

    pub fn ld_a_h(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.h.value())
    }

    pub fn ld_a_l(&mut self) -> u8 {
        Z80::ld_r_rp(&mut self.a, self.l.value())
    }

    pub fn ld_a_a(&mut self) -> u8 {
        let a = self.a.value();
        Z80::ld_r_rp(&mut self.a, a)
    }
}

mod tests {
    use crate::z80::{Register, Z80};

    #[test]
    fn test_ld_r_rp() {
        #[rustfmt::skip]
        let scenarios: [(
            fn(&mut Z80) -> u8,
            fn(&mut Z80) -> &mut Register,
            fn(&mut Z80) -> &mut Register,
        ); 49] = [
            (Z80::ld_a_a, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_a_b, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_a_c, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_a_d, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_a_e, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_a_h, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_a_l, |z80: &mut Z80| &mut z80.a, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_b_a, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_b_b, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_b_c, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_b_d, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_b_e, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_b_h, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_b_l, |z80: &mut Z80| &mut z80.b, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_c_a, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_c_b, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_c_c, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_c_d, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_c_e, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_c_h, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_c_l, |z80: &mut Z80| &mut z80.c, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_d_a, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_d_b, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_d_c, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_d_d, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_d_e, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_d_h, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_d_l, |z80: &mut Z80| &mut z80.d, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_e_a, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_e_b, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_e_c, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_e_d, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_e_e, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_e_h, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_e_l, |z80: &mut Z80| &mut z80.e, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_h_a, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_h_b, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_h_c, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_h_d, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_h_e, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_h_h, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_h_l, |z80: &mut Z80| &mut z80.h, |z80: &mut Z80| &mut z80.l),
            (Z80::ld_l_a, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_l_b, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_l_c, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_l_d, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_l_e, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_l_h, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_l_l, |z80: &mut Z80| &mut z80.l, |z80: &mut Z80| &mut z80.l),
        ];

        for (opcode, r_supplier, r_prime_supplier) in scenarios {
            let z80 = &mut Z80::new();

            r_prime_supplier(z80).set_value(0xDD);

            let t_states = opcode(z80);
            assert_eq!(4, t_states);

            let r = r_supplier(z80);
            assert_eq!(0xDD, r.value());
        }
    }
}
