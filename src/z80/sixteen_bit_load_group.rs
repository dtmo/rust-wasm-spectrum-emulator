use super::{Z80Memory, Z80};

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
    pub fn ld_bc_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        self.c = self.fetch_next_opcode(mem);
        self.b = self.fetch_next_opcode(mem);

        // T states
        10
    }

    pub fn ld_de_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        self.e = self.fetch_next_opcode(mem);
        self.d = self.fetch_next_opcode(mem);

        // T states
        10
    }

    pub fn ld_hl_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        self.l = self.fetch_next_opcode(mem);
        self.h = self.fetch_next_opcode(mem);

        // T states
        10
    }

    pub fn ld_sp_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
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
    pub fn ld_ix_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
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
    pub fn ld_iy_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
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
    pub fn ld_hl_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
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
    pub fn ld_ddbc_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        self.c = mem.read(&address);
        self.b = mem.read(&(address + 1));

        // T states
        20
    }

    pub fn ld_ddde_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        self.e = mem.read(&address);
        self.d = mem.read(&(address + 1));

        // T states
        20
    }

    pub fn ld_ddhl_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        self.l = mem.read(&address);
        self.h = mem.read(&(address + 1));

        // T states
        20
    }

    pub fn ld_ddsp_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        let val_low = mem.read(&address);
        let val_high = mem.read(&(address + 1));

        self.stack_pointer = ((val_high as u16) << 8) | val_low as u16;

        // T states
        20
    }

    /// ## LD IX, (nn)
    ///
    /// ### Operation
    ///
    /// IXh ← (nn + 1), IXI ← (nn)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// IX, (nn)
    /// `1 1 0 1 1 1 0 1` (DD)
    /// `0 0 1 0 1 0 1 0` (2A)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The contents of the address (nn) are loaded to the low-order portion of
    /// Index Register IX, and the contents of the next highest memory address
    /// (nn + 1) are loaded to the high-order portion of IX. The first n operand
    /// after the op code is the low-order byte of nn.
    ///
    /// | M Cycles | T States 4            | MHz E.T. |
    /// | -------- | --------------------- | -------- |
    /// | 6        | 20 (4, 4, 3, 3, 3, 3) | 5.00     |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If address 6666h contains 92h, and address 6667h contains DAh, then upon
    /// the execution of an LD IX, (6666h) instruction, Index Register IX
    /// contains DA92h.
    pub fn ld_ix_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        let val_low = mem.read(&address);
        let val_high = mem.read(&(address + 1));

        self.ix = ((val_high as u16) << 8) | val_low as u16;

        // T states
        20
    }

    /// ## LD IY, (nn)
    ///
    /// ### Operation
    ///
    /// IYh ← (nn + 1), IYI ← (nn)
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// IY, (nn)
    /// `1 1 1 1 1 1 0 1` (FD)
    /// `0 0 1 0 1 0 1 0` (2A)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The contents of the address (nn) are loaded to the low-order portion of
    /// Index Register IY, and the contents of the next highest memory address
    /// (nn + 1) are loaded to the high-order portion of IY. The first n operand
    /// after the op code is the low-order byte of nn.
    ///
    /// | M Cycles | T States 4            | MHz E.T. |
    /// | -------- | --------------------- | -------- |
    /// | 6        | 20 (4, 4, 3, 3, 3, 3) | 5.00     |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If address 6666h contains 92h, and address 6667h contains DAh, then upon
    /// the execution of an LD IY, (6666h) instruction, Index Register IY
    /// contains DA92h.
    pub fn ld_iy_mem_nn(&mut self, mem: &dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;
        let val_low = mem.read(&address);
        let val_high = mem.read(&(address + 1));

        self.iy = ((val_high as u16) << 8) | val_low as u16;

        // T states
        20
    }

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

        mem.write(&address, &self.l);
        mem.write(&(address + 1), &self.h);

        // T states
        16
    }

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

        mem.write(&address, &self.c);
        mem.write(&(address + 1), &self.b);

        // T states
        20
    }

    pub fn ld_mem_nn_ddde(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;

        mem.write(&address, &self.e);
        mem.write(&(address + 1), &self.d);

        // T states
        20
    }

    pub fn ld_mem_nn_ddhl(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;

        mem.write(&address, &self.l);
        mem.write(&(address + 1), &self.h);

        // T states
        20
    }

    pub fn ld_mem_nn_ddsp(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let low_n = self.fetch_next_opcode(mem);
        let high_n = self.fetch_next_opcode(mem);

        let address = ((high_n as u16) << 8) | low_n as u16;

        let sp_low: u8 = self.stack_pointer as u8;
        let sp_high: u8 = (self.stack_pointer >> 8) as u8;
        mem.write(&address, &sp_low);
        mem.write(&(address + 1), &sp_high);

        // T states
        20
    }

    /// ## LD (nn), IX
    ///
    /// ### Operation
    ///
    /// (nn + 1) ← IXh, (nn) ← IXI
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (nn), IX
    /// `1 1 0 1 1 1 0 1` (DD)
    /// `0 0 1 0 0 0 1 0` (22)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The low-order byte in Index Register IX is loaded to memory address
    /// (nn); the upper order byte is loaded to the next highest address (nn +
    /// 1). The first n operand after the op code is the low-order byte of nn.
    ///
    /// | M Cycles | T States           | 4 MHz E.T. |
    /// | -------- | ------------------ | ---------- |
    /// | 0        | (4, 4, 3, 3, 3, 3) | 5.00       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If Index Register IX contains 5A30h, then upon the execution of an LD
    /// (4392h), IX instruction, memory location 4392h contains number 30h and
    /// location 4393h contains 5Ah.
    pub fn ld_mem_nn_ix(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let address_low = self.fetch_next_opcode(mem);
        let address_high = self.fetch_next_opcode(mem);

        let address = ((address_high as u16) << 8) | address_low as u16;

        let ix_low: u8 = self.ix as u8;
        let ix_high: u8 = (self.ix >> 8) as u8;
        mem.write(&address, &ix_low);
        mem.write(&(address + 1), &ix_high);

        // T states
        20
    }

    /// ## LD (nn), IY
    ///
    /// ### Operation
    ///
    /// (nn + 1) ← IYh, (nn) ← IYI
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (nn), IY
    /// `1 1 0 1 1 1 0 1` (FD)
    /// `0 0 1 0 0 0 1 0` (22)
    /// `n n n n n n n n`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The low-order byte in Index Register IY is loaded to memory address
    /// (nn); the upper order byte is loaded to the next highest address (nn +
    /// 1). The first n operand after the op code is the low-order byte of nn.
    ///
    /// | M Cycles | T States           | 4 MHz E.T. |
    /// | -------- | ------------------ | ---------- |
    /// | 0        | (4, 4, 3, 3, 3, 3) | 5.00       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If Index Register IY contains 4174h, then upon the execution of an LD
    /// (8838h), IY instruction, memory location 8838h contains number 74h and
    /// location 8839h contains 41h.
    pub fn ld_mem_nn_iy(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let address_low = self.fetch_next_opcode(mem);
        let address_high = self.fetch_next_opcode(mem);

        let address = ((address_high as u16) << 8) | address_low as u16;

        let iy_low: u8 = self.iy as u8;
        let iy_high: u8 = (self.iy >> 8) as u8;
        mem.write(&address, &iy_low);
        mem.write(&(address + 1), &iy_high);

        // T states
        20
    }

    /// ## LD SP, HL
    ///
    /// ### Operation
    ///
    /// SP ← HL
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// SP, HL
    /// `1 1 1 1 1 0 0 1` (F9)
    ///
    /// ### Description
    ///
    /// The contents of the register pair HL are loaded to the Stack Pointer
    /// (SP).
    ///
    /// | M Cycles | T States | 4 MHz E.T. |
    /// | -------- | -------- | ---------- |
    /// | 1        | 6        | 1.5        |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If the register pair HL contains 442Eh, then upon the execution of an LD
    /// SP, HL instruction, the Stack Pointer also contains 442Eh.
    pub fn ld_sp_hl(&mut self) -> u8 {
        self.stack_pointer = ((self.h as u16) << 8) | self.l as u16;

        // T states
        6
    }
}

mod tests {
    use crate::z80::{tests::Ram, Z80Memory, Z80};

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

    #[test]
    fn test_ld_ix_mem_nn() {
        let bytes = &mut [0xDD, 0x2A, 0x04, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;

        let t_states = z80.ld_ix_mem_nn(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xF00F, z80.ix);
    }

    #[test]
    fn test_ld_iy_mem_nn() {
        let bytes = &mut [0xFD, 0x2A, 0x04, 0x00, 0x0F, 0xF0];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;

        let t_states = z80.ld_iy_mem_nn(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xF00F, z80.iy);
    }

    #[test]
    fn test_ld_mem_nn_hl() {
        let bytes = &mut [0x22, 0x03, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 1;
        z80.h = 0x0F;
        z80.l = 0xF0;

        let t_states = z80.ld_mem_nn_hl(ram);
        assert_eq!(16, t_states);

        assert_eq!(z80.l, ram.read(&3));
        assert_eq!(z80.h, ram.read(&4));
    }

    #[test]
    fn test_ld_mem_nn_ddbc() {
        let bytes = &mut [0xED, 0x43, 0x04, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.b = 0xEE;
        z80.c = 0xFF;

        let t_states = z80.ld_mem_nn_ddbc(ram);
        assert_eq!(20, t_states);

        assert_eq!(z80.c, ram.read(&4));
        assert_eq!(z80.b, ram.read(&5));
    }

    #[test]
    fn test_ld_mem_nn_ddde() {
        let bytes = &mut [0xED, 0x53, 0x04, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.d = 0xEE;
        z80.e = 0xFF;

        let t_states = z80.ld_mem_nn_ddde(ram);
        assert_eq!(20, t_states);

        assert_eq!(z80.e, ram.read(&4));
        assert_eq!(z80.d, ram.read(&5));
    }

    #[test]
    fn test_ld_mem_nn_ddhl() {
        let bytes = &mut [0xED, 0x63, 0x04, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.h = 0xEE;
        z80.l = 0xFF;

        let t_states = z80.ld_mem_nn_ddhl(ram);
        assert_eq!(20, t_states);

        assert_eq!(z80.l, ram.read(&4));
        assert_eq!(z80.h, ram.read(&5));
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

        assert_eq!(0xFF, ram.read(&4));
        assert_eq!(0xEE, ram.read(&5));
    }

    #[test]
    fn test_ld_mem_nn_ix() {
        let bytes = &mut [0xDD, 0x22, 0x04, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.ix = 0xEEFF;

        let t_states = z80.ld_mem_nn_ix(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xFF, ram.read(&4));
        assert_eq!(0xEE, ram.read(&5));
    }

    #[test]
    fn test_ld_mem_nn_iy() {
        let bytes = &mut [0xFD, 0x22, 0x04, 0x00, 0x00, 0x00];
        let ram = &mut Ram::new(bytes);
        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.iy = 0xEEFF;

        let t_states = z80.ld_mem_nn_iy(ram);
        assert_eq!(20, t_states);

        assert_eq!(0xFF, ram.read(&4));
        assert_eq!(0xEE, ram.read(&5));
    }

    #[test]
    fn test_ld_sp_hl() {
        let z80 = &mut Z80::new();
        z80.h = 0x44;
        z80.l = 0x2E;

        let t_states = z80.ld_sp_hl();
        assert_eq!(6, t_states);

        assert_eq!(0x442E, z80.stack_pointer);
    }
}
