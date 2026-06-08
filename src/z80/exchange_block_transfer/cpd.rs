use crate::z80::{
    register_flags::{
        set_h_flag_with, set_n_flag, set_p_flag_with, set_s_flag_with, set_x_flag_with,
        set_y_flag_with, set_z_flag_with, S_FLAG_BITMASK,
    },
    Z80Memory, Z80,
};

impl Z80 {
    /// ## CPD
    /// 
    /// ### Operation
    /// 
    /// A – (HL), HL ← HL – 1, BC ← BC – 1
    /// 
    /// ## Op Code
    /// 
    /// CPD
    /// 
    /// ## Operands
    /// 
    /// None.
    /// 
    /// `11101101` (ED)
    /// `10101001` (A9)
    /// 
    /// ### Description
    /// 
    /// The contents of the memory location addressed by the HL register pair is compared with
    /// the contents of the Accumulator. During a compare operation, a condition bit is set. The
    /// HL and Byte Counter (register pair BC) are decremented.
    /// 
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4       | 16 (4, 4, 3, 5) | 4.00       |
    /// 
    /// ### Condition Bits Affected
    /// 
    /// * S is set if result is negative; otherwise, it is reset.
    /// * Z is set if A equals (HL); otherwise, it is reset.
    /// * H is set if borrow from bit 4; otherwise, it is reset.
    /// * P/V is set if BC – 1≠ 0; otherwise, it is reset.
    /// * N is set.
    /// * C is not affected.
    /// 
    /// ### Example
    /// 
    /// If the HL register pair contains 1111h, memory location 1111h contains 3Bh, the Accumulator
    /// contains 3Bh, and the Byte Counter contains 0001h. Upon the execution of a
    /// CPD instruction, the Byte Counter contains 0000h, the HL register pair contains 1110h,
    /// the flag in the F Register is set, and the P/V flag in the F Register is reset. There is no
    /// effect on the contents of the Accumulator or address 1111h.
    pub fn cpd(&mut self, mem: &dyn Z80Memory) -> u8 {
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
        let half_carry_borrow = ((a & 0x0f).wrapping_add(hl_data & 0x0f)) & 0x10 == 0x10;
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
        self.set_bc(self.bc().wrapping_sub(1));

        // T States
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
}
