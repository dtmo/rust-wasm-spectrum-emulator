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
    /// `0 0 1 0 1 0 1 0` (2A)
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
    pub fn ld_hl_mem_nn(&mut self, mem: &dyn MemoryAccessor) -> u8 {
        let nl = self.fetch_next_opcode(mem);
        let nh = self.fetch_next_opcode(mem);

        let address = ((nh as u16) << 8) | nl as u16;
        self.l = mem.read(&address);
        self.h = mem.read(&(address + 1));

        // T states
        16
    }

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
    pub fn ld_ddbc_mem_nn(&mut self, mem: &dyn MemoryAccessor) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        self.c = mem.read(&address);
        self.b = mem.read(&(address + 1));

        // T states
        20
    }

    pub fn ld_ddde_mem_nn(&mut self, mem: &dyn MemoryAccessor) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        self.e = mem.read(&address);
        self.d = mem.read(&(address + 1));

        // T states
        20
    }

    pub fn ld_ddhl_mem_nn(&mut self, mem: &dyn MemoryAccessor) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        self.l = mem.read(&address);
        self.h = mem.read(&(address + 1));

        // T states
        20
    }

    pub fn ld_ddsp_mem_nn(&mut self, mem: &dyn MemoryAccessor) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        let val_low = mem.read(&address);
        let val_high = mem.read(&(address + 1));

        self.stack_pointer = ((val_high as u16) << 8) | val_low as u16;

        // T states
        20
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

    #[test]
    fn test_ld_hl_mem_nn() {
        let bytes = &mut [0x2A, 0x03, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 1;

        let t_states = z80.ld_hl_mem_nn(ram);
        assert_eq!(16, t_states);

        assert_eq!(0xF0, z80.h);
        assert_eq!(0x0F, z80.l);
    }

    #[test]
    fn test_ld_ddbc_mem_nn() {
        let bytes = &mut [0xED, 0x4B, 0x04, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;

        let t_states = z80.ld_ddbc_mem_nn(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xF0, z80.b);
        assert_eq!(0x0F, z80.c);
    }

    #[test]
    fn test_ld_ddde_mem_nn() {
        let bytes = &mut [0xED, 0x5B, 0x04, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;

        let t_states = z80.ld_ddde_mem_nn(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xF0, z80.d);
        assert_eq!(0x0F, z80.e);
    }

    #[test]
    fn test_ld_ddhl_mem_nn() {
        let bytes = &mut [0xED, 0x6B, 0x04, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;

        let t_states = z80.ld_ddhl_mem_nn(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xF0, z80.h);
        assert_eq!(0x0F, z80.l);
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
