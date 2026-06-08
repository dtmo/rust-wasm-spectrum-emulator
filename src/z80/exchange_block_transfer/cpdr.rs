use crate::z80::{
    register_flags::{
        set_h_flag_with, set_n_flag, set_p_flag_with, set_s_flag_with, set_x_flag_with,
        set_y_flag_with, set_z_flag_with, S_FLAG_BITMASK,
    },
    Z80Memory, Z80,
};

impl Z80 {
    /// ## CPDR
    ///
    /// ### Operation
    ///
    /// A – (HL), HL ← HL – 1, BC ← BC – 1
    ///
    /// ### Op Code
    ///
    /// CPDR
    ///
    /// ### Operands
    ///
    /// None.
    ///
    /// `11101101` (ED)
    /// `10111001` (B9)
    ///
    /// ### Description
    ///
    /// The contents of the memory location addressed by the HL register pair is compared with
    /// the contents of the Accumulator. During a compare operation, a condition bit is set. The
    /// HL and Byte Counter (BC) Register pairs are decremented. If decrementing allows the BC
    /// to go to 0 or if A = (HL), the instruction is terminated. If BC is not 0 and A = (HL), the
    /// program counter is decremented by two and the instruction is repeated. Interrupts are recognized
    /// and two refresh cycles execute after each data transfer. When the BC is set to 0,
    /// prior to instruction execution, the instruction loops through 64 KB if no match is found.
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
    /// | 4        | 16 (4, 4, 3, 5) | 4.00       |
    ///
    /// ### Condition Bits Affected
    ///
    /// * S is set if result is negative; otherwise, it is reset.
    /// * Z is set if A = (HL); otherwise, it is reset.
    /// * H is set if borrow form bit 4; otherwise, it is reset.
    /// * P/V is set if BC – 1 ≠ 0; otherwise, it is reset.
    /// * N is set.
    /// * C is not affected.
    ///
    /// ### Example
    ///
    /// The HL register pair contains 1118h, the Accumulator contains F3h, the Byte Counter
    /// contains 0007h, and memory locations contain the following data.
    ///
    /// * (1118h) contains 52h
    /// * (1117h) contains 00h
    /// * (1116h) contains F3h
    ///
    /// Upon the execution of a CPDR instruction, register pair HL contains 1115h, the Byte
    /// Counter contains 0004h, the P/V flag in the F Register is set, and the Z flag in the F Register
    /// is set.
    pub fn cpdr(&mut self, mem: &dyn Z80Memory) -> u8 {
        // The contents of the memory location addressed by the HL register pair
        let hl_address = self.hl();
        let hl_data = mem.read(hl_address);

        // is compared with the contents of the Accumulator.
        let a = self.a();

        let n = a.wrapping_sub(hl_data);

        // During a compare operation, a condition bit is set.

        // S is set if result is negative; otherwise, it is reset.
        let sign_flag = (n & S_FLAG_BITMASK) == S_FLAG_BITMASK;
        set_s_flag_with(&mut self.f, sign_flag);

        // Z is set if A equals (HL); otherwise, it is reset.
        set_z_flag_with(&mut self.f, a == hl_data);

        // H is set if borrow from bit 4; otherwise, it is reset.
        let half_carry_borrow = ((a & 0x0f).wrapping_sub(hl_data & 0x0f)) & 0x10 == 0x10;
        set_h_flag_with(&mut self.f, half_carry_borrow);

        // P/V is set if BC – 1 ≠ 0; otherwise, it is reset.
        let bc_dec = self.bc().wrapping_sub(1);
        set_p_flag_with(&mut self.f, bc_dec != 0);

        // N is set.
        set_n_flag(&mut self.f);

        // C is not affected.

        // Extra behaviour from http://www.z80.info/zip/z80-documented.pdf p.16
        // YF flag A copy of bit 1 of n
        let y_flag = (n & 0b00000010) == 0b00000010;
        set_y_flag_with(&mut self.f, y_flag);

        // XF flag Acopy of bit 3 of n.
        let x_flag = (n & 0b00001000) == 0b00001000;
        set_x_flag_with(&mut self.f, x_flag);

        // The HL and Byte Counter (register pair BC) are decremented.
        self.set_hl(self.hl().wrapping_sub(1));
        self.set_bc(bc_dec);

        let t_states: u8;
        if bc_dec == 0 || a == hl_data {
            // If decrementing causes BC to go to 0 or if A = (HL), the instruction is terminated.
            t_states = 16;
        } else {
            // If BC is not 0 and A ≠ (HL), the program counter is decremented by two and the instruction is repeated.
            self.pc = self.pc.wrapping_sub(2);
            t_states = 21;
        }

        t_states
    }
}

#[cfg(test)]
mod tests {
    use crate::z80::{
        register_flags::{p_flag, z_flag},
        tests::Ram,
        Z80,
    };

    /// The HL register pair contains 1118h, the Accumulator contains F3h, the Byte Counter
    /// contains 0007h, and memory locations contain the following data.
    ///
    /// * (1118h) contains 52h
    /// * (1117h) contains 00h
    /// * (1116h) contains F3h
    ///
    /// Upon the execution of a CPDR instruction, register pair HL contains 1115h, the Byte
    /// Counter contains 0004h, the P/V flag in the F Register is set, and the Z flag in the F Register
    /// is set.
    #[test]
    fn test_example() {
        let mut z80 = Z80::new();

        // If the HL register pair contains 0004h,
        z80.set_hl(0x0004);

        // the Accumulator contains F3h,
        z80.set_a(0xf3);

        // the Byte Counter contains 0007h,
        z80.set_bc(0x0007);

        // and memory locations contain the following data.
        // * (0002h) contains F3h
        // * (0003h) contains 00h
        // * (0004h) contains 52h
        let mut bytes = [0xED, 0xA1, 0xf3, 0x00, 0x52];
        let mem = Ram::new(&mut bytes);

        // Upon the execution of a CPDR instruction,
        let t_states = z80.cpdr(&mem);
        assert_eq!(21, t_states);

        let t_states = z80.cpdr(&mem);
        assert_eq!(21, t_states);

        let t_states = z80.cpdr(&mem);
        assert_eq!(16, t_states);

        // register pair HL contains 0001h,
        assert_eq!(0x0001, z80.hl());

        // the Byte Counter contains 0004h,
        assert_eq!(0x0004, z80.bc());

        // the P/V flag in the F Register is set,
        assert_eq!(true, p_flag(&z80.f));

        // and the Z flag in the F Register is set
        assert_eq!(true, z_flag(&z80.f));
    }
}
