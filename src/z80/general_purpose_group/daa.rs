use crate::z80::{
    register_flags::{
        c_flag, h_flag, n_flag, set_c_flag, set_h_flag, set_z_flag, unset_c_flag, unset_h_flag,
    },
    Z80,
};

impl Z80 {
    /// ### Operation
    ///
    /// @
    ///
    /// ### Op Code
    ///
    /// DAA: `00100111` (0x27)
    ///
    /// ### Operands
    ///
    /// None.
    ///
    /// ### Description
    ///
    /// This instruction conditionally adjusts the Accumulator for BCD addition
    /// and subtraction operations. For addition (ADD, ADC, INC) or subtraction
    /// (SUB, SBC, DEC, NEG), the following table indicates the operation being
    /// performed:
    ///
    /// | Operation | C Before DAA | Hex Value In Upper Digit (Bits 7–4) | H Before DAA | Hex Value In Lower Digit (Bits 3–0) | Number Added To Byte | C After DAA |
    /// | --------- | ------------ | ----------------------------------- | ------------ | ----------------------------------- | -------------------- | ----------- |
    /// |           | 0            | 9–0                                 | 0            | 0–9                                 | 00                   | 0           |
    /// |           | 0            | 0–8                                 | 0            | A–F                                 | 06                   | 0           |
    /// |           | 0            | 0–9                                 | 1            | 0–3                                 | 06                   | 0           |
    /// | ADD       | 0            | A–F                                 | 0            | 0–9                                 | 60                   | 1           |
    /// | ADC       | 0            | 9–F                                 | 0            | A–F                                 | 66                   | 1           |
    /// | INC       | 0            | A–F                                 | 1            | 0–3                                 | 66                   | 1           |
    /// |           | 1            | 0–2                                 | 0            | 0–9                                 | 60                   | 1           |
    /// |           | 1            | 0–2                                 | 0            | A–F                                 | 66                   | 1           |
    /// |           | 1            | 0–3                                 | 1            | 0–3                                 | 66                   | 1           |
    /// | SUB       | 0            | 0–9                                 | 0            | 0–9                                 | 00                   | 0           |
    /// | SBC       | 0            | 0–8                                 | 1            | 6–F                                 | FA                   | 0           |
    /// | DEC       | 1            | 7–F                                 | 0            | 0–9                                 | A0                   | 1           |
    /// | NEG       | 1            | 6–7                                 | 1            | 6–F                                 | 9A                   | 1           |
    ///
    /// ### Condition Bits Affected
    ///
    /// S is set if most-significant bit of the
    /// Accumulator is 1 after an operation; otherwise, it is reset.
    ///
    /// Z is set if the Accumulator is 0 after an operation; otherwise, it is
    /// reset.
    ///
    /// H: see the DAA instruction table on the previous page.
    ///
    /// P/V is set if the Accumulator is at even parity after an operation;
    /// otherwise, it is reset.
    ///
    /// N is not affected.
    ///
    /// C: see the DAA instruction table on the previous page.
    ///
    /// ### Example
    ///
    /// An addition operation is performed between 15 (BCD) and 27 (BCD); simple
    /// decimal arithmetic provides the following result:
    /// ```
    ///   15
    /// + 27
    ///   --
    ///   42
    /// ```
    ///
    /// The binary representations are added in the Accumulator according to
    /// standard binary arithmetic, as follows:
    /// ```
    ///   0001 0101
    /// + 0010 0111
    ///   ---- ----
    ///   0011 1100 = 3C
    /// ```
    ///
    /// The sum is ambiguous. The DAA instruction adjusts this result so that
    /// the correct BCD representation is obtained, as follows:
    /// ```
    ///   0011 1100
    /// + 0000 0110
    ///   ---- ----
    ///   0100 0010 = 42
    /// ```
    pub fn daa(&mut self) -> u8 {
        let a_high = self.a.value() & 0xF0;
        let a_low = self.a.value() & 0x0F;
        let diff: u8 = match (c_flag(&self.f), a_high, h_flag(&self.f), a_low) {
            (false, a_high, false, a_low) if a_high <= 0x09 && a_low <= 0x09 => 0x00,
            (false, a_high, true, a_low) if a_high <= 0x09 && a_low <= 0x09 => 0x06,
            (false, a_high, _, a_low) if a_high <= 0x08 && a_low >= 0x0A => 0x06,
            (false, a_high, false, a_low) if a_high >= 0x0A && a_low <= 0x09 => 0x60,
            (true, _, false, a_low) if a_low <= 0x09 => 0x60,
            (true, _, true, a_low) if a_low <= 0x09 => 0x66,
            (true, _, _, a_low) if a_low >= 0x0A => 0x66,
            (false, a_high, _, a_low) if a_high >= 0x09 && a_low >= 0x0A => 0x66,
            (false, a_high, true, a_low) if a_high >= 0x0A && a_low <= 0x09 => 0x66,
            _ => 0x00,
        };

        let new_c_flag: bool = match (c_flag(&self.f), a_high, a_low) {
            (false, a_high, a_low) if a_high <= 0x09 && a_low <= 0x09 => false,
            (false, a_high, a_low) if a_high <= 0x08 && a_low >= 0x0A => false,
            (false, a_high, a_low) if a_high >= 0x09 && a_low >= 0x0A => true,
            (false, a_high, a_low) if a_high >= 0x0A && a_low <= 0x09 => true,
            (true, _, _) => true,
            _ => c_flag(&self.f),
        };

        let new_h_flag: bool = match (n_flag(&self.f), h_flag(&self.f), a_low) {
            (false, _, a_low) if a_low <= 0x09 => false,
            (false, _, a_low) if a_low >= 0x0A => true,
            (true, false, _) => false,
            (true, true, a_low) if a_low >= 0x06 => false,
            (true, true, a_low) if a_low <= 0x05 => true,
            _ => h_flag(&self.f),
        };

        self.a.add(diff);

        if new_c_flag {
            set_c_flag(&mut self.f);
        } else {
            unset_c_flag(&mut self.f);
        }

        if new_h_flag {
            set_h_flag(&mut self.f);
        } else {
            unset_h_flag(&mut self.f);
        }

        // SF, YF, XF are copies of bit 7,5,3 of the result respectively;
        let new_f = (self.f.value() & 0b10101011u8) | (self.a.value() & 0b01010100u8);
        self.f.set_value(new_f);

        // ZF is set according to the result and NF is always unchanged.
        if self.a.value() == 0 {
            set_z_flag(&mut self.f);
        }

        // T states
        4
    }
}
