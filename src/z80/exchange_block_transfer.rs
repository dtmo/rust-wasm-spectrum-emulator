use super::Z80;

impl Z80 {
    // Exchange, Block Trabsfer, and Search Group

    /// ## EX DE, HL
    /// ### Operation
    /// DE ↔ HL
    /// ### Op Code
    /// EX
    /// ### Operands
    /// DE, HL
    /// `1 1 1 0 1 0 1 1` (EB)
    /// ### Description
    /// The 2-byte contents of register pairs DE and HL are exchanged.
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 1        | 4        | 1.00       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If register pair DE contains 2822h and register pair HL contains 499Ah,
    /// then upon the execution of an EX DE, HL instruction, register pair DE
    /// contains 499Ah and register pair HL contains 2822h.
    pub fn ex_de_hl(&mut self) -> u8 {
        (self.d, self.e, self.h, self.l) = (self.h, self.l, self.d, self.e);

        // T states
        4
    }

    /// ## EX AF, AF′
    /// ### Operation
    /// AF ↔ AF'
    /// ### Op Code
    /// EX
    /// ### Operands
    /// AF, AF′
    /// `0 0 0 0 1 0 0 0` (08)
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
        (self.a, self.f, self.a_prime, self.f_prime) = (self.a_prime, self.f_prime, self.a, self.f);

        // T states
        4
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_ex_de_hl() {
        let mut z80 = Z80::new();
        z80.d = 0x28;
        z80.e = 0x22;
        z80.h = 0x49;
        z80.l = 0x9A;

        let t_states = z80.ex_de_hl();

        assert_eq!(4, t_states);
        assert_eq!(0x49, z80.d);
        assert_eq!(0x9A, z80.e);
        assert_eq!(0x28, z80.h);
        assert_eq!(0x22, z80.l)
    }

    #[test]
    fn test_ex_af_afp() {
        let mut z80 = Z80::new();
        z80.a = 0x99;
        z80.f = 0x00;
        z80.a_prime = 0x59;
        z80.f_prime = 0x44;

        let t_states = z80.ex_af_afp();

        assert_eq!(4, t_states);
        assert_eq!(0x59, z80.a);
        assert_eq!(0x44, z80.f);
        assert_eq!(0x99, z80.a_prime);
        assert_eq!(0x00, z80.f_prime)
    }
}
