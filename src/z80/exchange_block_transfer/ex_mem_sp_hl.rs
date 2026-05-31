use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## EX (SP), HL
    /// ### Operation
    /// H ↔ (SP+1), L ↔ (SP)
    /// ### Op Code
    /// EX
    /// ### Operands
    /// (SP), HL
    /// `11100011` (E3)
    /// ### Description
    /// The low-order byte contained in register pair HL is exchanged with the
    /// contents of the memory address specified by the contents of register
    /// pair SP (Stack Pointer), and the high-order byte of HL is exchanged with
    /// the next highest memory address (SP+1).
    ///
    /// | M Cycles | T States           | 4 MHz E.T. |
    /// | -------- | ------------------ | ---------- |
    /// | 5        | 19 (4, 3, 4, 3, 5) | 4.75       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If the HL register pair contains 7012h, the SP register pair contains
    /// 8856h, the memory location 8856h contains byte 11h, and memory location
    /// 8857h contains byte 22h, then the instruction EX (SP), HL results in the
    /// HL register pair containing number 2211h, memory location 8856h
    /// containing byte 12h, memory location 8857h containing byte 70h and Stack
    /// Pointer containing 8856h.
    pub fn ex_mem_sp_hl(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let mem_sp = mem.read(self.sp);
        mem.write(self.sp, self.l.value());
        self.l.set_value(mem_sp);

        let mem_sp = mem.read(self.sp + 1);
        mem.write(self.sp + 1, self.h.value());
        self.h.set_value(mem_sp);

        // T states
        19
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ex_mem_sp_hl() {
        let bytes = &mut [0xE3, 0x11, 0x22];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.sp = 1;
        z80.h.set_value(0x70);
        z80.l.set_value(0x12);

        let t_states = z80.ex_mem_sp_hl(&mut ram);
        assert_eq!(19, t_states);

        assert_eq!(0x22, z80.h.value());
        assert_eq!(0x11, z80.l.value());
        assert_eq!(0x12, ram.read(1));
        assert_eq!(0x70, ram.read(2));
    }
}
