use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD dd, (nn)
    ///
    /// ### Operation
    ///
    /// ddh ← (nn + 1) ddl ← (nn)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// dd, (nn)
    /// `1 1 1 0 1 1 0 1` (ED)
    /// `0 1 d d 1 0 1 1`
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The contents of address (nn) are loaded to the low-order portion of
    /// register pair dd, and the contents of the next highest memory address
    /// (nn + 1) are loaded to the high-order portion of dd. Register pair dd
    /// defines BC, DE, HL, or SP register pairs, assembled as follows in the
    /// object code:
    ///
    /// | Pair | dd |
    /// | ---- | -- |
    /// | BC   | 00 |
    /// | DE   | 01 |
    /// | HL   | 10 |
    /// | SP   | 11 |
    ///
    /// The first n operand after the op code is the low-order byte of (nn).
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
    /// If Address 2130h contains 65h and address 2131h contains 78h, then upon
    /// the execution of an LD BC, (2130h) instruction, the BC register pair
    /// contains 7865h.
    pub fn ld_ddbc_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        self.c.set_value(mem.read(address));
        self.b.set_value(mem.read(address.wrapping_add(1)));

        // T states
        20
    }

    pub fn ld_ddde_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        self.e.set_value(mem.read(address));
        self.d.set_value(mem.read(address.wrapping_add(1)));

        // T states
        20
    }

    pub fn ld_ddhl_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        self.l.set_value(mem.read(address));
        self.h.set_value(mem.read(address.wrapping_add(1)));

        // T states
        20
    }

    pub fn ld_ddsp_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        let val_low = mem.read(address);
        let val_high = mem.read(address.wrapping_add(1));

        self.stack_pointer = ((val_high as u16) << 8) | val_low as u16;

        // T states
        20
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_ddbc_mem_nn() {
        let bytes = &mut [0xED, 0x4B, 0x04, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;

        let t_states = z80.ld_ddbc_mem_nn(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xF0, z80.b.value());
        assert_eq!(0x0F, z80.c.value());
    }

    #[test]
    fn test_ld_ddde_mem_nn() {
        let bytes = &mut [0xED, 0x5B, 0x04, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;

        let t_states = z80.ld_ddde_mem_nn(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xF0, z80.d.value());
        assert_eq!(0x0F, z80.e.value());
    }

    #[test]
    fn test_ld_ddhl_mem_nn() {
        let bytes = &mut [0xED, 0x6B, 0x04, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;

        let t_states = z80.ld_ddhl_mem_nn(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xF0, z80.h.value());
        assert_eq!(0x0F, z80.l.value());
    }

    #[test]
    fn test_ld_ddsp_mem_nn() {
        let bytes = &mut [0xED, 0x7B, 0x04, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;

        let t_states = z80.ld_ddsp_mem_nn(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xF00F, z80.stack_pointer);
    }
}
