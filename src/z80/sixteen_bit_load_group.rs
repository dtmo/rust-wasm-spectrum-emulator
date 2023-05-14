use super::{MemoryAccessor, Z80};

impl Z80 {
    /// ## LD dd, nn
    ///
    /// ### Operation
    ///
    /// dd ← nn
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// dd, nn
    /// `0 0 d d 0 0 0 1`
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The 2-byte integer nn is loaded to the dd register pair, in which dd
    /// defines the BC, DE, HL, or SP register pairs, assembled as follows in
    /// the object code:
    ///
    /// | Pair | dd |
    /// | ---- | -- |
    /// | BC   | 00 |
    /// | DE   | 01 |
    /// | HL   | 10 |
    /// | SP   | 11 |
    ///
    /// The first n operand after the op code is the low-order byte.
    ///
    /// | M Cycles | T States ----| 4 MHz E.T. |
    /// | -------- | ------------ | ---------- |
    /// | 2        | 10 (4, 3, 3) | 2.50       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// Upon the execution of an LD HL, 5000h instruction, the HL register pair
    /// contains 5000h.
    pub fn ld_bc_nn(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        self.b = self.fetch_next_opcode(memory_accessor);
        self.c = self.fetch_next_opcode(memory_accessor);

        // T states
        10
    }

    pub fn ld_de_nn(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        self.d = self.fetch_next_opcode(memory_accessor);
        self.e = self.fetch_next_opcode(memory_accessor);

        // T states
        10
    }

    pub fn ld_hl_nn(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        self.h = self.fetch_next_opcode(memory_accessor);
        self.l = self.fetch_next_opcode(memory_accessor);

        // T states
        10
    }

    pub fn ld_sp_nn(&mut self, memory_accessor: &dyn MemoryAccessor) -> u8 {
        let high_n = self.fetch_next_opcode(memory_accessor);
        let low_n = self.fetch_next_opcode(memory_accessor);

        self.stack_pointer = ((high_n as u16) << 8) | low_n as u16;

        // T states
        10
    }
}

mod tests {
    use crate::z80::{tests::Ram, Z80};

    #[test]
    fn test_ld_bc_nn() {
        let bytes = &mut [0x01, 0x02];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();

        let t_states = z80.ld_bc_nn(ram);
        assert_eq!(10, t_states);

        assert_eq!(0x01, z80.b);
        assert_eq!(0x02, z80.c);
    }

    #[test]
    fn test_ld_de_nn() {
        let bytes = &mut [0x01, 0x02];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();

        let t_states = z80.ld_de_nn(ram);
        assert_eq!(10, t_states);

        assert_eq!(0x01, z80.d);
        assert_eq!(0x02, z80.e);
    }

    #[test]
    fn test_ld_hl_nn() {
        let bytes = &mut [0x01, 0x02];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();

        let t_states = z80.ld_hl_nn(ram);
        assert_eq!(10, t_states);

        assert_eq!(0x01, z80.h);
        assert_eq!(0x02, z80.l);
    }

    #[test]
    fn test_ld_sp_nn() {
        let bytes = &mut [0x01, 0x02];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();

        let t_states = z80.ld_sp_nn(ram);
        assert_eq!(10, t_states);

        assert_eq!(0x0102, z80.stack_pointer);
    }
}
