use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD HL, (nn)
    ///
    /// ### Operation
    ///
    /// H ← (nn + 1), L ← (nn)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// HL, (nn)
    /// `00101010` (2A)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The contents of memory address (nn) are loaded to the low-order portion
    /// of register pair HL (Register L), and the contents of the next highest
    /// memory address (nn + 1) are loaded to the high-order portion of HL
    /// (Register H). The first n operand after the op code is the low-order byte
    /// of nn.
    ///
    /// | M Cycles | T States           | 4 MHz E.T. |
    /// | -------- | ------------------ | ---------- |
    /// | 5        | 16 (4, 3, 3, 3, 3) | 4.00       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If address 4545h contains 37h and address 4546h contains A1h, then upon
    /// the execution of an LD HL, (4545h) instruction, the HL register pair
    /// contains A137h.
    pub fn ld_hl_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let nl = self.fetch_next_opcode(mem);
        let nh = self.fetch_next_opcode(mem);

        let address = ((nh as u16) << 8) | nl as u16;
        self.l.set_value(mem.read(address));
        self.h.set_value(mem.read(address.wrapping_add(1)));

        // T states
        16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_hl_mem_nn() {
        let bytes = &mut [0x2A, 0x03, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.pc = 1;

        let t_states = z80.ld_hl_mem_nn(ram);
        assert_eq!(16, t_states);

        assert_eq!(0xF0, z80.h.value());
        assert_eq!(0x0F, z80.l.value());
    }
}
