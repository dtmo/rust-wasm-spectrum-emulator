use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD (nn), HL
    ///
    /// ### Operation
    ///
    /// (nn + 1) ← H, (nn) ← L
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (nn), HL
    /// `0 0 1 0 0 0 1 0` (22)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The contents of the low-order portion of register pair HL (Register L)
    /// are loaded to memory address (nn), and the contents of the high-order
    /// portion of HL (Register H) are loaded to the next highest memory address
    /// (nn + 1). The first n operand after the op code is the low-order byte of
    /// nn.
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
    /// If register pair HL contains 483Ah, then upon the execution of an LD
    /// (B2291 – 1), HL instruction, address B229h contains 3Ah and address
    /// B22Ah contains 48h.
    pub fn ld_mem_nn_hl(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;

        mem.write(address, self.l.value());
        mem.write(address.wrapping_add(1), self.h.value());

        // T states
        16
    }
}

mod tests {
    use crate::z80::{tests::Ram, Z80Memory, Z80};

    #[test]
    fn test_ld_mem_nn_hl() {
        let bytes = &mut [0x22, 0x03, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.set_hl(0x0FF0);

        let t_states = z80.ld_mem_nn_hl(ram);
        assert_eq!(16, t_states);

        assert_eq!(z80.l.value(), ram.read(3));
        assert_eq!(z80.h.value(), ram.read(4));
    }
}
