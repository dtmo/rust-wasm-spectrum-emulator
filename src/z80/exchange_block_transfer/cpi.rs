use crate::z80::{
    register_flags::{
        set_h_flag_with, set_n_flag, set_p_flag_with, set_s_flag_with, set_x_flag_with,
        set_y_flag_with, set_z_flag_with, S_FLAG_BITMASK,
    },
    Z80Memory, Z80,
};

impl Z80 {
    /// ## CPI
    /// 
    /// ### Operation
    /// 
    /// A – (HL), HL ← HL+1, BC ← BC – 1
    /// 
    /// ### Op Code
    /// 
    /// CPI
    /// 
    /// ### Operands
    /// 
    /// None.
    /// `11101101` (ED)
    /// `10100001` (A1)
    /// 
    /// ### Description
    /// 
    /// The contents of the memory location addressed by the HL register is
    /// compared with the contents of the Accumulator. With a true compare, a
    /// condition bit is set. Then HL is incremented and the Byte Counter
    /// (register pair BC) is decremented.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 16 (4, 4, 3, 5) | 4.00       |
    ///
    /// ### Condition Bits Affected
    /// 
    /// * S is set if result is negative; otherwise, it is reset.
    /// * Z is set if A is (HL); otherwise, it is reset.
    /// * H is set if borrow from bit 4; otherwise, it is reset.
    /// * P/V is set if BC – 1 is not 0; otherwise, it is reset.
    /// * N is set.
    /// * C is not affected.
    /// 
    /// ### Example
    /// 
    /// If the HL register pair contains 1111h, memory location 1111h contains
    /// 3Bh, the Accumulator contains 3Bh, and the Byte Counter contains 0001h.
    /// Upon the execution of a CPI instruction, the Byte Counter contains
    /// 0000h, the HL register pair contains 1112h, the Z flag in the F register
    /// is set, and the P/V flag in the F Register is reset. There is no effect
    /// on the contents of the Accumulator or to address 1111h.
    pub fn cpi(&mut self, mem: &dyn Z80Memory) -> u8 {
        // The contents of the memory location addressed by the HL register is compared with the contents of the Accumulator.
        let address = self.hl();
        let data = mem.read(address);

        let a = self.a.value();
        let n = a.wrapping_sub(data);

        // With a true compare, a condition bit is set.
        // * S is set if result is negative; otherwise, it is reset.
        let sign_flag = n & S_FLAG_BITMASK == S_FLAG_BITMASK;
        set_s_flag_with(&mut self.f, sign_flag);

        // * Z is set if A is (HL); otherwise, it is reset.
        set_z_flag_with(&mut self.f, n == 0);

        // * H is set if borrow from bit 4; otherwise, it is reset.
        let half_carry_borrow = ((a & 0x0f).wrapping_sub(data & 0x0f)) & 0x10 == 0x10;
        set_h_flag_with(&mut self.f, half_carry_borrow);
        
        // * P/V is set if BC – 1 is not 0; otherwise, it is reset.
        let bc_dec = self.bc().wrapping_sub(1);
        set_p_flag_with(&mut self.f, bc_dec != 0);
        
        // * N is set.
        set_n_flag(&mut self.f);

        // Extra behaviour from http://www.z80.info/zip/z80-documented.pdf p.16
        let y_flag = (n & 0b00000010) == 0b00000010;
        set_y_flag_with(&mut self.f, y_flag);
        let x_flag = (n & 0b00001000) == 0b00001000;
        set_x_flag_with(&mut self.f, x_flag);

        // Then HL is incremented and the Byte Counter (register pair BC) is decremented.
        let hl_inc = address.wrapping_add(1);
        self.set_hl(hl_inc);
        self.set_bc(bc_dec);

        // T states
        16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::{
        register_flags::{h_flag, n_flag, p_flag, s_flag, z_flag},
        tests::Ram,
    };

    #[test]
    fn test_example() {
        let mut z80 = Z80::new();
        
        // If the HL register pair contains 0002h,
        z80.set_hl(0x0002);

        // memory location 0002h contains 3Bh,
        let mut bytes = [0xED, 0xA1, 0x3B];
        let mem = Ram::new(&mut bytes);
        
        // the Accumulator contains 3Bh,
        z80.set_a(0x3B);

        // and the Byte Counter contains 0001h.
        z80.set_bc(0x0001);
        
        // Upon the execution of a CPI instruction,
        let t_states = z80.cpi(&mem);
        
        assert_eq!(16, t_states);
        
        // the Byte Counter contains 0000h,
        assert_eq!(0x0000, z80.bc());
        
        // the HL register pair contains 1112h,
        assert_eq!(0x0003, z80.hl());
        
        // the Z flag in the F register is set,
        assert_eq!(true, z_flag(&z80.f));
        
        // and the P/V flag in the F Register is reset.
        assert_eq!(false, p_flag(&z80.f));
        
        // There is no effect on the contents of the Accumulator
        assert_eq!(0x3B, z80.a());

        // or to address 1111h.
        assert_eq!(0x3B, mem.read(2));
    }

    #[test]
    fn test_cpi_true_compare_with_bc_zero() {
        let mut z80 = Z80::new();

        let mut bytes = [0xED, 0xA1, 0x80];
        let mem = Ram::new(&mut bytes);
        
        z80.set_hl(0x0002);
        z80.set_bc(0x0001);
        z80.set_a(0x80);

        let t_states = z80.cpi(&mem);

        assert_eq!(16, t_states);

        assert_eq!(false, s_flag(&z80.f));
        assert_eq!(true, z_flag(&z80.f));
        assert_eq!(false, h_flag(&z80.f));
        assert_eq!(false, p_flag(&z80.f));
        assert_eq!(true, n_flag(&z80.f));
    }

    #[test]
    fn test_cpi_positive_compare_with_half_carry_borrow_and_bc_not_zero() {
        let mut bytes = [0xED, 0xA1, 0x01];
        let mem = Ram::new(&mut bytes);
        let mut z80 = Z80::new();
        z80.set_hl(0x0002);
        z80.set_bc(0x0002);
        z80.set_a(0x10);

        let t_states = z80.cpi(&mem);

        assert_eq!(16, t_states);

        assert_eq!(false, s_flag(&z80.f));
        assert_eq!(false, z_flag(&z80.f));
        assert_eq!(true, h_flag(&z80.f));
        assert_eq!(true, p_flag(&z80.f));
        assert_eq!(true, n_flag(&z80.f));
    }

    #[test]
    fn test_cpi_negative_compare() {
        let mut bytes = [0xED, 0xA1, 0x10];
        let mem = Ram::new(&mut bytes);
        let mut z80 = Z80::new();
        z80.set_hl(0x0002);
        z80.set_bc(0x0002);
        z80.set_a(0x08);

        let t_states = z80.cpi(&mem);

        assert_eq!(16, t_states);

        assert_eq!(true, s_flag(&z80.f));
        assert_eq!(false, z_flag(&z80.f));
        assert_eq!(false, h_flag(&z80.f));
        assert_eq!(true, p_flag(&z80.f));
        assert_eq!(true, n_flag(&z80.f));
    }
}
