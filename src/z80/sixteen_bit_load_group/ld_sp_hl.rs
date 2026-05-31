use crate::z80::Z80;

impl Z80 {
    /// ## LD SP, HL
    ///
    /// ### Operation
    ///
    /// SP ← HL
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// SP, HL
    /// `11111001` (F9)
    ///
    /// ### Description
    ///
    /// The contents of the register pair HL are loaded to the Stack Pointer
    /// (SP).
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 1        | 6        | 1.5        |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the register pair HL contains 442Eh, then upon the execution of an LD
    /// SP, HL instruction, the Stack Pointer also contains 442Eh.
    pub fn ld_sp_hl(&mut self) -> u8 {
        self.sp = ((self.h.value() as u16) << 8) | self.l.value() as u16;

        // T states
        6
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ld_sp_hl() {
        let z80 = &mut Z80::new();
        z80.set_hl(0x442E);

        let t_states = z80.ld_sp_hl();
        assert_eq!(6, t_states);

        assert_eq!(0x442E, z80.sp);
    }
}
