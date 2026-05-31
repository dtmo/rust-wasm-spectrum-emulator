use crate::z80::Z80;

impl Z80 {
    // Exchange, Block Transfer, and Search Group

    /// ## EX DE, HL
    /// ### Operation
    /// DE ↔ HL
    /// ### Op Code
    /// EX
    /// ### Operands
    /// DE, HL
    /// `11101011` (EB)
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
        let temp_d = self.d.value();
        let temp_e = self.e.value();
        let temp_h = self.h.value();
        let temp_l = self.l.value();

        self.d.set_value(temp_h);
        self.e.set_value(temp_l);
        self.h.set_value(temp_d);
        self.l.set_value(temp_e);

        // T states
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_de_hl() {
        let mut z80 = Z80::new();
        z80.set_de(0x2822);
        z80.set_hl(0x499A);

        let t_states = z80.ex_de_hl();

        assert_eq!(4, t_states);
        assert_eq!(0x499A, z80.de());
        assert_eq!(0x2822, z80.hl());
    }
}
