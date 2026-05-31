use crate::z80::{Register, Z80Memory, Z80};

impl Z80 {
    /// ## LD (HL), r
    ///
    /// ### Operation
    ///
    /// (HL) ← r
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (HL), r
    /// `0 1 1 1 0 r r r`
    ///
    /// ### Description
    ///
    /// The contents of register r are loaded to the memory location specified
    /// by the contents of the HL register pair. The r symbol identifies
    /// registers A, B, C, D, E, H, or L, assembled as follows in the object
    /// code:
    ///
    /// | Register | r   |
    /// | -------- | --- |
    /// | A        | 111 |
    /// | B        | 000 |
    /// | C        | 001 |
    /// | D        | 010 |
    /// | E        | 011 |
    /// | H        | 100 |
    /// | L        | 101 |
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 2        | 7 (4, 3) | 1.75       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the contents of register pair HL specify memory location 2146h and
    /// Register B contains byte 29h, then upon the execution of an LD (HL), B
    /// instruction, memory address 2146h also contains 29h.
    fn ld_mem_hl_r(hl: u16, r: &Register, mem: &mut dyn Z80Memory) -> u8 {
        mem.write(hl, r.value());

        // T states
        7
    }

    pub fn ld_mem_hl_a(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.a, mem)
    }

    pub fn ld_mem_hl_b(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.b, mem)
    }

    pub fn ld_mem_hl_c(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.c, mem)
    }

    pub fn ld_mem_hl_d(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.d, mem)
    }

    pub fn ld_mem_hl_e(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.e, mem)
    }

    pub fn ld_mem_hl_h(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.h, mem)
    }

    pub fn ld_mem_hl_l(&self, mem: &mut dyn Z80Memory) -> u8 {
        Z80::ld_mem_hl_r(self.hl(), &self.l, mem)
    }
}

mod tests {
    use crate::z80::{tests::Ram, Register, Z80Memory, Z80};

    #[test]
    fn test_ld_mem_hl_r() {
        let scenarios: [(
            fn(&Z80, &mut dyn Z80Memory) -> u8,
            fn(&mut Z80) -> &mut Register,
        ); 5] = [
            (Z80::ld_mem_hl_a, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_mem_hl_b, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_mem_hl_c, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_mem_hl_d, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_mem_hl_e, |z80: &mut Z80| &mut z80.e),
            // (Z80::ld_hl_h, |z80: &mut Z80| &mut z80.h),
            // (Z80::ld_hl_l, |z80: &mut Z80| &mut z80.l),
        ];

        let bytes = &mut [0, 0, 0];
        let ram = &mut Ram::new(bytes);

        for (opcode, register_supplier) in scenarios {
            let z80 = &mut Z80::new();
            z80.set_hl(0x0002);

            let register = register_supplier(z80);
            register.set_value(0xDD);

            let t_states = opcode(z80, ram);
            assert_eq!(7, t_states);

            assert_eq!(0xDD, ram.read(2));
        }
    }
}
