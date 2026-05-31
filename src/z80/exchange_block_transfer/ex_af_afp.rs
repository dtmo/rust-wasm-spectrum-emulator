use crate::z80::Z80;

impl Z80 {
    /// ## EX AF, AF′
    /// ### Operation
    /// AF ↔ AF'
    /// ### Op Code
    /// EX
    /// ### Operands
    /// AF, AF′
    /// `00001000` (08)
    /// ### Description
    /// The 2-byte contents of the register pairs AF and AF' are exchanged.
    /// Register pair AF consists of registers A′ and F′.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 1        | 4        | 1.00       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If register pair AF contains 9900h and register pair AF′ contains 5944h,
    /// the contents of AF are 5944h and the contents of AF′ are 9900h upon
    /// execution of the EX AF, AF′ instruction.
    pub fn ex_af_afp(&mut self) -> u8 {
        let temp_a = self.a.value();
        let temp_f = self.f.value();
        let temp_a_prime = self.a_prime.value();
        let temp_f_prime = self.f_prime.value();

        self.a.set_value(temp_a_prime);
        self.f.set_value(temp_f_prime);
        self.a_prime.set_value(temp_a);
        self.f_prime.set_value(temp_f);

        // T states
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_af_afp() {
        let mut z80 = Z80::new();
        z80.a.set_value(0x99);
        z80.f.set_value(0x00);
        z80.a_prime.set_value(0x59);
        z80.f_prime.set_value(0x44);

        let t_states = z80.ex_af_afp();

        assert_eq!(4, t_states);

        assert_eq!(0x59, z80.a.value());
        assert_eq!(0x44, z80.f.value());
        assert_eq!(0x99, z80.a_prime.value());
        assert_eq!(0x00, z80.f_prime.value())
    }
}
