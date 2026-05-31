use crate::z80::{
    Register, Z80, Z80Memory
};

impl Z80 {
    /// ## LD r, (HL)
    ///
    /// ### Operation
    ///
    /// r ← (HL)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// r, (HL)  `0 1 r r r 1 1 0`
    ///
    /// ### Description
    ///
    /// The 8-bit contents of memory location (HL) are loaded to register r, in
    /// which r identifies registers A, B, C, D, E, H, or L, assembled as
    /// follows in the object code:
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
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If register pair HL contains the number 75A1h, and memory address 75A1h
    /// contains byte 58h, the execution of LD C, (HL) results in 58h in
    /// Register C.
    fn ld_r_mem_hl(r: &mut Register, hl: u16, mem: &dyn Z80Memory) -> u8 {
        let data = mem.read(hl);
        r.set_value(data);

        // T states
        7
    }

    pub fn ld_a_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.a, hl, mem)
    }

    pub fn ld_b_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.b, hl, mem)
    }

    pub fn ld_c_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.c, hl, mem)
    }

    pub fn ld_d_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.d, hl, mem)
    }

    pub fn ld_e_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.e, hl, mem)
    }

    pub fn ld_h_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.h, hl, mem)
    }

    pub fn ld_l_mem_hl(&mut self, mem: &dyn Z80Memory) -> u8 {
        let hl = self.hl();
        Z80::ld_r_mem_hl(&mut self.l, hl, mem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;
    
    #[test]
    fn test_ld_r_mem_hl() {
        let scenarios: [(
            fn(&mut Z80, &dyn Z80Memory) -> u8,
            fn(&mut Z80) -> &mut Register,
        ); 7] = [
            (Z80::ld_a_mem_hl, |z80: &mut Z80| &mut z80.a),
            (Z80::ld_b_mem_hl, |z80: &mut Z80| &mut z80.b),
            (Z80::ld_c_mem_hl, |z80: &mut Z80| &mut z80.c),
            (Z80::ld_d_mem_hl, |z80: &mut Z80| &mut z80.d),
            (Z80::ld_e_mem_hl, |z80: &mut Z80| &mut z80.e),
            (Z80::ld_h_mem_hl, |z80: &mut Z80| &mut z80.h),
            (Z80::ld_l_mem_hl, |z80: &mut Z80| &mut z80.l),
        ];

        let bytes = &mut [0xAA, 0xBB, 0xCC];
        let ram = Ram::new(bytes);

        for (opcode, register_supplier) in scenarios {
            let mut z80 = Z80::new();
            z80.set_hl(0x0002);

            let t_states = opcode(&mut z80, &ram);
            assert_eq!(7, t_states);

            let register = register_supplier(&mut z80);
            assert_eq!(ram.read(2), register.value());
        }
    }
}
