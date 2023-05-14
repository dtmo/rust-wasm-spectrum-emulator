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
    pub fn ld_bc_nn(&mut self, mem: &dyn MemoryAccessor) -> u8 {
        self.c = self.fetch_next_opcode(mem);
        self.b = self.fetch_next_opcode(mem);

        // T states
        10
    }

    pub fn ld_de_nn(&mut self, mem: &dyn MemoryAccessor) -> u8 {
        self.e = self.fetch_next_opcode(mem);
        self.d = self.fetch_next_opcode(mem);

        // T states
        10
    }

    pub fn ld_hl_nn(&mut self, mem: &dyn MemoryAccessor) -> u8 {
        self.l = self.fetch_next_opcode(mem);
        self.h = self.fetch_next_opcode(mem);

        // T states
        10
    }

    pub fn ld_sp_nn(&mut self, mem: &dyn MemoryAccessor) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        self.stack_pointer = ((high_n as u16) << 8) | low_n as u16;

        // T states
        10
    }

    /// ## LD IX, nn
    ///
    /// ### Operation
    ///
    /// IX ← nn
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// IX, nn
    /// `1 1 0 1 1 1 0 1` (DD)
    /// `0 0 1 0 0 0 0 1` (21)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The nn integer is loaded to Index Register IX. The first n operand after
    /// the op code is the low-order byte.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 14 (4, 4, 3, 3) | 3.50       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// Upon the execution of an LD IX, 45A2h instruction, the index register
    /// contains integer 45A2h.
    pub fn ld_ix_nn(&mut self, mem: &dyn MemoryAccessor) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        self.ix = ((high_n as u16) << 8) | low_n as u16;

        // T states
        14
    }

    /// ## LD IY, nn
    ///
    /// ### Operation
    ///
    /// IY ← nn
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// IY, nn
    /// `1 1 1 1 1 1 0 1` (DD)
    /// `0 0 1 0 0 0 0 1` (21)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The nn integer is loaded to Index Register IY. The first n operand after
    /// the op code is the low-order byte.
    ///
    /// | M Cycles | T States        | 4 MHz E.T. |
    /// | -------- | --------------- | ---------- |
    /// | 4        | 14 (4, 4, 3, 3) | 3.50       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// Upon the execution of an LD IY, 7733h instruction, Index Register IY
    /// contains the integer 7733h.
    pub fn ld_iy_nn(&mut self, mem: &dyn MemoryAccessor) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        self.iy = ((high_n as u16) << 8) | low_n as u16;

        // T states
        14
    }
}

mod tests {
    use crate::z80::{tests::Ram, Z80};

    #[test]
    fn test_ld_bc_nn() {
        let bytes = &mut [0x02, 0x01];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();

        let t_states = z80.ld_bc_nn(ram);
        assert_eq!(10, t_states);

        assert_eq!(0x01, z80.b);
        assert_eq!(0x02, z80.c);
    }

    #[test]
    fn test_ld_de_nn() {
        let bytes = &mut [0x02, 0x01];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();

        let t_states = z80.ld_de_nn(ram);
        assert_eq!(10, t_states);

        assert_eq!(0x01, z80.d);
        assert_eq!(0x02, z80.e);
    }

    #[test]
    fn test_ld_hl_nn() {
        let bytes = &mut [0x02, 0x01];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();

        let t_states = z80.ld_hl_nn(ram);
        assert_eq!(10, t_states);

        assert_eq!(0x01, z80.h);
        assert_eq!(0x02, z80.l);
    }

    #[test]
    fn test_ld_sp_nn() {
        let bytes = &mut [0x02, 0x01];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();

        let t_states = z80.ld_sp_nn(ram);
        assert_eq!(10, t_states);

        assert_eq!(0x0102, z80.stack_pointer);
    }

    #[test]
    fn test_ld_ix_nn() {
        let bytes = &mut [0x02, 0x01];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();

        let t_states = z80.ld_ix_nn(ram);
        assert_eq!(14, t_states);

        assert_eq!(0x0102, z80.ix);
    }

    #[test]
    fn test_ld_iy_nn() {
        let bytes = &mut [0x02, 0x01];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();

        let t_states = z80.ld_iy_nn(ram);
        assert_eq!(14, t_states);

        assert_eq!(0x0102, z80.iy);
    }
}
