use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## POP qq
    /// ### Operation
    /// qqH ← (SP+1), qqL ← (SP)
    /// ### Op Code
    /// POP
    /// ### Operand
    /// qq
    /// `1 1 q q 0 0 0 1`
    /// ### Description
    /// The top two bytes of the external memory last-in, first-out (LIFO) stack
    /// are popped to register pair qq. The Stack Pointer (SP) Register pair
    /// holds the 16-bit address of the current top of the Stack. This
    /// instruction first loads to the low-order portion of qq, the byte at the
    /// memory location corresponding to the contents of SP; then SP is
    /// incremented and the contents of the corresponding adjacent memory
    /// location are loaded to the high-order portion of qq and the SP is now
    /// incremented again. The operand qq identifies register pair BC, DE, HL,
    /// or AF, assembled as follows in the object code:
    ///
    /// | Pair | r  |
    /// | ---- | -- |
    /// | BC   | 00 |
    /// | DE   | 01 |
    /// | HL   | 10 |
    /// | AF   | 11 |
    ///
    /// | M Cycles | T States     | 4 MHz E.T. |
    /// | -------- | ------------ | ---------- |
    /// | 3        | 10 (4, 3, 3) | 2.50       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If the Stack Pointer contains 1000h, memory location 1000h contains 55h,
    /// and location 1001h contains 33h, the instruction POP HL results in
    /// register pair HL containing 3355h, and the Stack Pointer containing
    /// 1002h.
    pub fn pop_qqbc(&mut self, mem: &dyn Z80Memory) -> u8 {
        self.c.set_value(mem.read(self.sp));
        self.sp = self.sp.wrapping_add(1);

        self.b.set_value(mem.read(self.sp));
        self.sp = self.sp.wrapping_add(1);

        // T states
        10
    }

    pub fn pop_qqde(&mut self, mem: &dyn Z80Memory) -> u8 {
        self.e.set_value(mem.read(self.sp));
        self.sp = self.sp.wrapping_add(1);

        self.d.set_value(mem.read(self.sp));
        self.sp = self.sp.wrapping_add(1);

        // T states
        10
    }

    pub fn pop_qqhl(&mut self, mem: &dyn Z80Memory) -> u8 {
        self.l.set_value(mem.read(self.sp));
        self.sp = self.sp.wrapping_add(1);

        self.h.set_value(mem.read(self.sp));
        self.sp = self.sp.wrapping_add(1);

        // T states
        10
    }

    pub fn pop_qqaf(&mut self, mem: &dyn Z80Memory) -> u8 {
        self.f.set_value(mem.read(self.sp));
        self.sp = self.sp.wrapping_add(1);

        self.a.set_value(mem.read(self.sp));
        self.sp = self.sp.wrapping_add(1);

        // T states
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_pop_qqbc() {
        let bytes = &mut [0xC1, 0x55, 0x33];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.sp = 1;

        let t_states = z80.pop_qqbc(ram);
        assert_eq!(10, t_states);

        assert_eq!(3, z80.sp);
        assert_eq!(0x33, z80.b.value());
        assert_eq!(0x55, z80.c.value());
    }

    #[test]
    fn test_pop_qqde() {
        let bytes = &mut [0xD1, 0x55, 0x33];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.sp = 1;

        let t_states = z80.pop_qqde(ram);
        assert_eq!(10, t_states);

        assert_eq!(3, z80.sp);
        assert_eq!(0x33, z80.d.value());
        assert_eq!(0x55, z80.e.value());
    }

    #[test]
    fn test_pop_qqhl() {
        let bytes = &mut [0xE1, 0x55, 0x33];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.sp = 1;

        let t_states = z80.pop_qqhl(ram);
        assert_eq!(10, t_states);

        assert_eq!(3, z80.sp);
        assert_eq!(0x33, z80.h.value());
        assert_eq!(0x55, z80.l.value());
    }

    #[test]
    fn test_pop_qqaf() {
        let bytes = &mut [0xF1, 0x55, 0x33];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.sp = 1;

        let t_states = z80.pop_qqaf(ram);
        assert_eq!(10, t_states);

        assert_eq!(3, z80.sp);
        assert_eq!(0x33, z80.a.value());
        assert_eq!(0x55, z80.f.value());
    }
}
