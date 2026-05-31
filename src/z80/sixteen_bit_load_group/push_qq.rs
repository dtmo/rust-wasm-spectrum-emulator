use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## PUSH qq
    /// ### Operation
    /// (SP – 2) ← qqL, (SP – 1) ← qqH
    /// ### Op Code
    /// PUSH
    /// ### Operand
    /// qq
    /// `1 1 q q 0 1 0 1`
    /// ### Description
    /// The contents of the register pair qq are pushed to the external memory
    /// last-in, first-out (LIFO) stack. The Stack Pointer (SP) Register pair
    /// holds the 16-bit address of the current top of the Stack. This
    /// instruction first decrements SP and loads the high-order byte of
    /// register pair qq to the memory address specified by the SP. The SP is
    /// decremented again and loads the low-order byte of qq to the memory
    /// location corresponding to this new address in the SP. The operand qq
    /// identifies register pair BC, DE, HL, or AF, assembled as follows in the
    /// object code:
    ///
    /// | Pair | qq |
    /// | ---- | -- |
    /// | BC   | 00 |
    /// | DE   | 01 |
    /// | HL   | 10 |
    /// | AF   | 11 |
    ///
    /// | M Cycles | T States     | 4 MHz E.T. |
    /// | -------- | ------------ | ---------- |
    /// | 3        | 11 (5, 3, 3) | 2.75       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If the AF Register pair contains 2233h and the Stack Pointer contains
    /// 1007h, then upon the execution of a PUSH AF instruction, memory address
    /// 1006h contains 22h, memory address 1005h contains 33h, and the Stack
    /// Pointer contains 1005h.

    pub fn push_qqbc(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        mem.write(self.stack_pointer, self.b.value());

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        mem.write(self.stack_pointer, self.c.value());

        // T states
        11
    }

    pub fn push_qqde(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        mem.write(self.stack_pointer, self.d.value());

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        mem.write(self.stack_pointer, self.e.value());

        // T states
        11
    }

    pub fn push_qqhl(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        mem.write(self.stack_pointer, self.h.value());

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        mem.write(self.stack_pointer, self.l.value());

        // T states
        11
    }

    pub fn push_qqaf(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        mem.write(self.stack_pointer, self.a.value());

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        mem.write(self.stack_pointer, self.f.value());

        // T states
        11
    }
}

mod tests {
    use crate::z80::{tests::Ram, Z80Memory, Z80};

    #[test]
    fn test_push_qqbc() {
        let bytes = &mut [0xC5, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.stack_pointer = 3;
        z80.set_bc(0x2233);

        let t_states = z80.push_qqbc(ram);
        assert_eq!(11, t_states);

        assert_eq!(0x22, ram.read(2));
        assert_eq!(0x33, ram.read(1));
        assert_eq!(1, z80.stack_pointer);
    }

    #[test]
    fn test_push_qqde() {
        let bytes = &mut [0xD5, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.stack_pointer = 3;
        z80.set_de(0x2233);

        let t_states = z80.push_qqde(ram);
        assert_eq!(11, t_states);

        assert_eq!(0x22, ram.read(2));
        assert_eq!(0x33, ram.read(1));
        assert_eq!(1, z80.stack_pointer);
    }

    #[test]
    fn test_push_qqhl() {
        let bytes = &mut [0xE5, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.stack_pointer = 3;
        z80.set_hl(0x2233);

        let t_states = z80.push_qqhl(ram);
        assert_eq!(11, t_states);

        assert_eq!(0x22, ram.read(2));
        assert_eq!(0x33, ram.read(1));
        assert_eq!(1, z80.stack_pointer);
    }

    #[test]
    fn test_push_qqaf() {
        let bytes = &mut [0xF5, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.stack_pointer = 3;
        z80.a.set_value(0x22);
        z80.f.set_value(0x33);

        let t_states = z80.push_qqaf(ram);
        assert_eq!(11, t_states);

        assert_eq!(0x22, ram.read(2));
        assert_eq!(0x33, ram.read(1));
        assert_eq!(1, z80.stack_pointer);
    }
}
