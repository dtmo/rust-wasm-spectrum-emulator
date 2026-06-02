use crate::z80::{
    register_flags::{
        set_h_flag_with, set_n_flag, set_p_flag_with, set_s_flag_with, set_x_flag_with,
        set_y_flag_with, set_z_flag_with, S_FLAG_BITMASK,
    },
    Z80Memory, Z80,
};

impl Z80 {
    /// ## CPIR
    ///
    /// ### Operation
    ///
    /// A – (HL), HL ← HL+1, BC ← BC – 1
    /// ### Op Code
    ///
    /// CPIR
    ///
    /// ### Operands
    ///
    /// None.
    /// `11101101` (ED)
    /// `10110001` (B1)
    ///
    /// ### Description
    ///
    /// The contents of the memory location addressed by the HL register pair is compared with
    /// the contents of the Accumulator. During a compare operation, a condition bit is set. HL is
    /// incremented and the Byte Counter (register pair BC) is decremented. If decrementing
    /// causes BC to go to 0 or if A = (HL), the instruction is terminated. If BC is not 0 and A ≠
    /// (HL), the program counter is decremented by two and the instruction is repeated.
    /// Interrupts are recognized and two refresh cycles are executed after each data transfer.
    ///
    /// If BC is set to 0 before instruction execution, the instruction loops through 64 KB if no
    /// match is found.
    ///
    /// For BC ≠ 0 and A ≠ (HL):
    ///
    /// | M Cycles | T States           | 4 MHz E.T. |
    /// | -------- | ------------------ | ---------- |
    /// | 5        | 21 (4, 4, 3, 5, 5) | 5.25       |
    ///
    /// For BC = 0 and A = (HL):
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 5        | 16 (4, 4, 3, 5) | 4.00       |
    ///
    /// ### Condition Bits Affected
    ///
    /// * S is set if result is negative; otherwise, it is reset.
    /// * Z is set if A equals (HL); otherwise, it is reset.
    /// * H is set if borrow from bit 4; otherwise, it is reset.
    /// * P/V is set if BC – 1 does not equal 0; otherwise, it is reset.
    /// * N is set.
    /// * C is not affected.
    ///
    /// ### Example
    ///
    /// If the HL register pair contains 1111h, the Accumulator contains F3h, the Byte Counter
    /// contains 0007h, and memory locations contain the following data.
    ///
    /// * (1111h) contains 52h
    /// * (1112h) contains 00h
    /// * (1113h) contains F3h
    ///
    /// Upon the execution of a CPIR instruction, register pair HL contains 1114h, the Byte
    /// Counter contains 0004h, the P/V flag in the F Register is set, and the Z flag in the
    /// F Register is set
    pub fn cpir(&mut self, mem: &dyn Z80Memory) -> u8 {
        // The contents of the memory location addressed by the HL register pair is compared with the contents of the Accumulator.
        // A – (HL)
        let address = self.hl();
        let hl_data = mem.read(address);

        let a = self.a.value();
        let n = a.wrapping_sub(hl_data);

        // During a compare operation, a condition bit is set.

        // S is set if result is negative; otherwise, it is reset.
        let sign_flag = (n & S_FLAG_BITMASK) == S_FLAG_BITMASK;
        set_s_flag_with(&mut self.f, sign_flag);

        // Z is set if A equals (HL); otherwise, it is reset.
        set_z_flag_with(&mut self.f, a == hl_data);

        // H is set if borrow from bit 4; otherwise, it is reset.
        let half_carry_borrow = ((a & 0x0f).wrapping_add(hl_data & 0x0f)) & 0x10 == 0x10;
        set_h_flag_with(&mut self.f, half_carry_borrow);

        // P/V is set if BC – 1 does not equal 0; otherwise, it is reset.
        let bc_dec = self.bc().wrapping_sub(1);
        set_p_flag_with(&mut self.f, bc_dec != 0);

        // N is set
        set_n_flag(&mut self.f);

        // Extra behaviour from http://www.z80.info/zip/z80-documented.pdf p.16
        // YF flag A copy of bit 1 of n
        let y_flag = (n & 0b00000010) == 0b00000010;
        set_y_flag_with(&mut self.f, y_flag);

        // XF flag Acopy of bit 3 of n.
        let x_flag = (n & 0b00001000) == 0b00001000;
        set_x_flag_with(&mut self.f, x_flag);

        // HL is incremented and the Byte Counter (register pair BC) is decremented.
        let hl_inc = address.wrapping_add(1);
        self.set_hl(hl_inc);
        self.set_bc(bc_dec);

        let t_states: u8;
        if bc_dec == 0 || a == hl_data {
            // If decrementing causes BC to go to 0 or if A = (HL), the instruction is terminated.
            t_states = 16;
        } else {
            // If BC is not 0 and A ≠ (HL), the program counter is decremented by two and the instruction is repeated.
            self.pc = self.pc.wrapping_sub(2);
            t_states = 25;
        }
        
        t_states
    }
}

#[cfg(test)]
mod tests {
    use crate::z80::{
        register_flags::{p_flag, z_flag},
        tests::Ram,
        Z80
    };

    #[test]
    fn test_example() {
        let mut z80 = Z80::new();
        
        // If the HL register pair contains 0002h,
        z80.set_hl(0x0002);

        // the Accumulator contains F3h,
        z80.set_a(0xf3);

        // the Byte Counter contains 0007h,
        z80.set_bc(0x0007);

        // and memory locations contain the following data.
        // * (0002h) contains 52h
        // * (0003h) contains 00h
        // * (0004h) contains F3h
        let mut bytes = [0xED, 0xA1, 0x52, 0x00, 0xf3];
        let mem = Ram::new(&mut bytes);
        
        // Upon the execution of a CPIR instruction,
        let t_states = z80.cpir(&mem);
        assert_eq!(25, t_states);
        
        let t_states = z80.cpir(&mem);
        assert_eq!(25, t_states);
        
        let t_states = z80.cpir(&mem);
        assert_eq!(16, t_states);

        // register pair HL contains 0005h,
        assert_eq!(0x0005, z80.hl());

        // the Byte Counter contains 0004h,
        assert_eq!(0x0004, z80.bc());

        // the P/V flag in the F Register is set,
        assert_eq!(true, p_flag(&z80.f));
        
        // and the Z flag in the F Register is set
        assert_eq!(true, z_flag(&z80.f));
    }

    // #[test]
    // fn test_cpi_true_compare_with_bc_zero() {
    //     let mut bytes = [0xED, 0xA1, 0x80];
    //     let mem = Ram::new(&mut bytes);
    //     let mut z80 = Z80::new();
    //     z80.set_hl(0x0002);
    //     z80.set_bc(0x0001);
    //     z80.set_a(0x80);

    //     let t_states = z80.cpi(&mem);

    //     assert_eq!(16, t_states);

    //     assert_eq!(false, s_flag(&z80.f));
    //     assert_eq!(true, z_flag(&z80.f));
    //     assert_eq!(false, h_flag(&z80.f));
    //     assert_eq!(false, p_flag(&z80.f));
    //     assert_eq!(true, n_flag(&z80.f));
    // }

    // #[test]
    // fn test_cpi_positive_compare_with_half_carry_borrow_and_bc_not_zero() {
    //     let mut bytes = [0xED, 0xA1, 0x01];
    //     let mem = Ram::new(&mut bytes);
    //     let mut z80 = Z80::new();
    //     z80.set_hl(0x0002);
    //     z80.set_bc(0x0002);
    //     z80.set_a(0x10);

    //     let t_states = z80.cpi(&mem);

    //     assert_eq!(16, t_states);

    //     assert_eq!(false, s_flag(&z80.f));
    //     assert_eq!(false, z_flag(&z80.f));
    //     assert_eq!(true, h_flag(&z80.f));
    //     assert_eq!(true, p_flag(&z80.f));
    //     assert_eq!(true, n_flag(&z80.f));
    // }

    // #[test]
    // fn test_cpi_negative_compare() {
    //     let mut bytes = [0xED, 0xA1, 0x10];
    //     let mem = Ram::new(&mut bytes);
    //     let mut z80 = Z80::new();
    //     z80.set_hl(0x0002);
    //     z80.set_bc(0x0002);
    //     z80.set_a(0x08);

    //     let t_states = z80.cpi(&mem);

    //     assert_eq!(16, t_states);

    //     assert_eq!(true, s_flag(&z80.f));
    //     assert_eq!(false, z_flag(&z80.f));
    //     assert_eq!(true, h_flag(&z80.f));
    //     assert_eq!(true, p_flag(&z80.f));
    //     assert_eq!(true, n_flag(&z80.f));
    // }
}
