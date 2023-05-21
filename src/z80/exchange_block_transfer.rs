use super::{
    Z80,
};

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
}
