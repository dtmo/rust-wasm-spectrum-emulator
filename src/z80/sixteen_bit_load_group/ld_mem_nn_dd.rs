use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD (nn), dd
    ///
    /// ### Operation
    ///
    /// (nn + 1) ← ddh, (nn) ← ddl
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (nn), dd
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `0 1 d d 0 0 1 1`
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The low-order byte of register pair dd is loaded to memory address (nn);
    /// the upper byte is loaded to memory address (nn + 1). Register pair dd
    /// defines either BC, DE, HL, or SP, assembled as follows in the object
    /// code:
    ///
    /// | Pair | dd |
    /// | ---- | -- |
    /// | BC   | 00 |
    /// | DE   | 01 |
    /// | HL   | 10 |
    /// | SP   | 11 |
    ///
    /// The first n operand after the op code is the low-order byte of a two
    /// byte memory address.
    ///
    /// | M Cycles | T States              | 4 MHz E.T. |
    /// | -------- | --------------------- | ---------- |
    /// | 6        | 20 (4, 4, 3, 3, 3, 3) | 5.00       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If register pair BC contains the number 4644h, the instruction LD
    /// (1000h), BC results in 44h in memory location 1000h, and 46h in memory
    /// location 1001h.
    pub fn ld_mem_nn_ddbc(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;

        mem.write(address, self.c.value());
        mem.write(address.wrapping_add(1), self.b.value());

        // T states
        20
    }

    pub fn ld_mem_nn_ddde(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;

        mem.write(address, self.e.value());
        mem.write(address.wrapping_add(1), self.d.value());

        // T states
        20
    }

    pub fn ld_mem_nn_ddhl(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;

        mem.write(address, self.l.value());
        mem.write(address.wrapping_add(1), self.h.value());

        // T states
        20
    }

    pub fn ld_mem_nn_ddsp(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;

        let sp_low: u8 = self.stack_pointer as u8;
        let sp_high: u8 = (self.stack_pointer >> 8) as u8;
        mem.write(address, sp_low);
        mem.write(address.wrapping_add(1), sp_high);

        // T states
        20
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_mem_nn_ddbc() {
        let bytes = &mut [0xED, 0x43, 0x04, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.set_bc(0xEEFF);

        let t_states = z80.ld_mem_nn_ddbc(ram);
        assert_eq!(20, t_states);

        assert_eq!(z80.c.value(), ram.read(4));
        assert_eq!(z80.b.value(), ram.read(5));
    }

    #[test]
    fn test_ld_mem_nn_ddde() {
        let bytes = &mut [0xED, 0x53, 0x04, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.set_de(0xEEFF);

        let t_states = z80.ld_mem_nn_ddde(ram);
        assert_eq!(20, t_states);

        assert_eq!(z80.e.value(), ram.read(4));
        assert_eq!(z80.d.value(), ram.read(5));
    }

    #[test]
    fn test_ld_mem_nn_ddhl() {
        let bytes = &mut [0xED, 0x63, 0x04, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.set_hl(0xEEFF);

        let t_states = z80.ld_mem_nn_ddhl(ram);
        assert_eq!(20, t_states);

        assert_eq!(z80.l.value(), ram.read(4));
        assert_eq!(z80.h.value(), ram.read(5));
    }

    #[test]
    fn test_ld_mem_nn_ddsp() {
        let bytes = &mut [0xED, 0x73, 0x04, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.stack_pointer = 0xEEFF;

        let t_states = z80.ld_mem_nn_ddsp(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xFF, ram.read(4));
        assert_eq!(0xEE, ram.read(5));
    }
}
