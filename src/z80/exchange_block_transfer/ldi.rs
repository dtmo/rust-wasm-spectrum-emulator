use crate::z80::{
    register_flags::{
        set_p_flag_with, set_x_flag_with, set_y_flag_with, unset_h_flag, unset_n_flag,
    },
    Z80Memory, Z80,
};

impl Z80 {
    /// ## LDI
    /// ### Operation
    /// (DE) ← (HL), DE ← DE + 1, HL ← HL + 1, BC ← BC – 1
    /// ### Op Code
    /// LDI
    /// ### Operands
    /// None
    /// `11101101` (ED)
    /// `10100000` (A0)
    /// ### Description
    /// A byte of data is transferred from the memory location addressed, by the
    /// contents of the HL register pair to the memory location addressed by the
    /// contents of the DE register pair. Then both these register pairs are
    /// incremented and the Byte Counter (BC) Register pair is decremented.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 16 (4, 4, 3, 5) | 4.00       |
    ///
    /// ### Condition Bits Affected
    /// * S is not affected.
    /// * Z is not affected.
    /// * H is reset.
    /// * P/V is set if BC – 1 ≠ 0; otherwise, it is reset.
    /// * N is reset.
    /// * C is not affected.
    /// ### Example
    /// If the HL register pair contains 1111h, memory location 1111h contains
    /// byte 88h, the DE register pair contains 2222h, the memory location 2222h
    /// contains byte 66h, and the BC register pair contains 7h, then the
    /// instruction LDI results in the following contents in register pairs and
    /// memory addresses:
    ///
    /// |         |          |       |
    /// | ------- | -------- | ----- |
    /// | HL      | contains | 1112h |
    /// | (1111h) | contains | 88h   |
    /// | DE      | contains | 2223h |
    /// | (2222h) | contains | 88h   |
    /// | BC      | contains | 6H    |
    pub fn ldi(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        // Transfer data from HL address to DE address
        let hl = self.hl();
        let data = mem.read(hl);
        let de = self.de();
        mem.write(de, data);

        // Increment HL and DE
        self.set_hl(hl.wrapping_add(1));
        self.set_de(de.wrapping_add(1));

        // Read and decrement BC
        let bc = self.bc().wrapping_sub(1);
        self.set_bc(bc);

        unset_h_flag(&mut self.f);
        set_p_flag_with(&mut self.f, bc != 0);
        unset_n_flag(&mut self.f);

        // Extra behaviour from http://www.z80.info/zip/z80-documented.pdf p.16
        let n = self.a.value().wrapping_add(data);
        set_y_flag_with(&mut self.f, n & 0b00000010 != 0);
        set_x_flag_with(&mut self.f, n & 0b00001000 != 0);

        // T states
        16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::{
        register_flags::{h_flag, n_flag, p_flag},
        tests::Ram,
    };

    #[test]
    fn test_ldi() {
        let bytes = &mut [0xED, 0xA0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.pc = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0100);

        let t_states = z80.ldi(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0x00FF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }

    #[test]
    fn test_ldi_bc_result_0() {
        let bytes = &mut [0xED, 0xA0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.pc = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0001);

        let t_states = z80.ldi(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0x0000, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(!p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }

    #[test]
    fn test_ldi_bc_result_ffff() {
        let bytes = &mut [0xED, 0xA0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.pc = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0000);

        let t_states = z80.ldi(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0xFFFF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }
}
