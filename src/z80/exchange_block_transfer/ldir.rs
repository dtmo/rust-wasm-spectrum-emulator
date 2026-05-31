use crate::z80::{
    register_flags::{
        set_p_flag_with, set_x_flag_with, set_y_flag_with, unset_h_flag, unset_n_flag,
    },
    Z80Memory, Z80,
};

impl Z80 {
    /// ## LDIR
    /// ### Operation
    /// repeat {(DE) ← (HL), DE ← DE + 1, HL ← HL + 1, BC ← BC – 1} while (BC ≠ 0)
    /// ### Op Code
    /// LDIR
    /// ### Operand
    /// None
    /// `11101101` (ED)
    /// `10110000` (B0)
    /// ### Description
    /// This 2-byte instruction transfers a byte of data from the memory
    /// location addressed by the contents of the HL register pair to the memory
    /// location addressed by the DE register pair. Both these register pairs
    /// are incremented and the Byte Counter (BC) Register pair is decremented.
    /// If decrementing allows the BC to go to 0, the instruction is terminated.
    /// If BC is not 0, the program counter is decremented by two and the
    /// instruction is repeated. Interrupts are recognized and two refresh
    /// cycles are executed after each data transfer. When the BC is set to 0
    /// prior to instruction execution, the instruction loops through 64 KB.
    /// For BC ≠ 0:
    ///
    /// | M Cycles | T States           | 4 MHz E.T. |
    /// | -------- | ------------------ | ---------- |
    /// | 5        | 21 (4, 4, 3, 5, 5) | 5.25       |
    ///
    /// For BC = 0:
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
    /// The HL register pair contains 11111h, the DE register pair contains
    /// 2222h, the BC register pair contains 0003h, and memory locations contain
    /// the following data.
    ///
    /// |         |          |     |         |          |     |
    /// | ------- | -------- | --- | ------- | -------- | --- |
    /// | (1111h) | contains | 88h | (2222h) | contains | 66h |
    /// | (1112h) | contains | 36h | (2223h) | contains | 59h |
    /// | (1113h) | contains | A5h | (2224h) | contains | C5h |
    ///
    /// Upon the execution of an LDIR instruction, the contents of register
    /// pairs and memory locations now contain:
    ///
    /// |         |          |       |         |          |     |
    /// | ------- | -------- | ----- | ------- | -------- | --- |
    /// | HL      | contains | 1114h |         |          |     |
    /// | DE      | contains | 2225h |         |          |     |
    /// | BC      | contains | 0000h |         |          |     |
    /// | (1111h) | contains | 88h   | (2222h) | contains | 88h |
    /// | (1112h) | contains | 36h   | (2223h) | contains | 36h |
    /// | (1113h) | contains | A5h   | (2224h) | contains | A5h |
    pub fn ldir(&mut self, mem: &mut dyn Z80Memory) -> u8 {
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

        let t_states;

        if bc != 0 {
            self.pc -= 2;
            t_states = 21;
        } else {
            t_states = 16;
        }

        unset_h_flag(&mut self.f);
        set_p_flag_with(&mut self.f, bc != 0);
        unset_n_flag(&mut self.f);

        // Extra behaviour from http://www.z80.info/zip/z80-documented.pdf p.16
        let n = self.a.value().wrapping_add(data);
        set_y_flag_with(&mut self.f, n & 0b00000010 != 0);
        set_x_flag_with(&mut self.f, n & 0b00001000 != 0);

        t_states
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
    fn test_ldir() {
        let bytes = &mut [0xED, 0xB0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.pc = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0100);

        let t_states = z80.ldir(&mut ram);
        assert_eq!(21, t_states);

        assert_eq!(0, z80.pc);
        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0x00FF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }

    #[test]
    fn test_ldir_bc_result_0() {
        let bytes = &mut [0xED, 0xB0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.pc = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0001);

        let t_states = z80.ldir(&mut ram);
        assert_eq!(16, t_states);

        assert_eq!(2, z80.pc);
        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0x0000, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(!p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }

    #[test]
    fn test_ldir_bc_result_ffff() {
        let bytes = &mut [0xED, 0xB0, 0xFF, 0x00];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.pc = 2;
        z80.set_hl(0x0002);
        z80.set_de(0x0003);
        z80.set_bc(0x0000);

        let t_states = z80.ldir(&mut ram);
        assert_eq!(21, t_states);

        assert_eq!(0, z80.pc);
        assert_eq!(ram.read(2), ram.read(3));
        assert_eq!(0x0003, z80.hl());
        assert_eq!(0x0004, z80.de());
        assert_eq!(0xFFFF, z80.bc());

        assert!(!h_flag(&z80.f));
        assert!(p_flag(&z80.f));
        assert!(!n_flag(&z80.f));
    }
}
