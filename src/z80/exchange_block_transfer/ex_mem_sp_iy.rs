use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## EX (SP), IY
    /// ### Operation
    /// IYH ↔ (SP+1), IYL ↔ (SP)
    /// ### Op Code
    /// EX
    /// ### Operands
    /// (SP), IY
    /// `11111101` (FD)
    /// `11100011` (E3)
    /// ### Description
    /// The low-order byte contained in register IY is exchanged with the
    /// contents of the memory address specified by the contents of register
    /// pair SP (Stack Pointer), and the high-order byte of IY is exchanged with
    /// the next highest memory address (SP+1).
    ///
    /// | M Cycles | T States              | 4 MHz E.T. |
    /// | -------- | --------------------- | ---------- |
    /// | 6        | 23 (4, 4, 3, 4, 3, 5) | 5.75       |
    ///
    /// ### Condition Bits Affected
    /// None.
    /// ### Example
    /// If Index Register IY contains 3988h, the SP register pair Contains
    /// 0100h, memory location 0100h contains byte 90h, and memory location
    /// 0101h contains byte 48h, then the instruction EX (SP), IY results in the
    /// IY register pair containing number 4890h, memory location 0100h
    /// containing 88h, memory location 0101h containing 39h, and the Stack
    /// Pointer containing 0100h.
    pub fn ex_mem_sp_iy(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let iyl: u8 = self.iy as u8;
        let iyh = (self.iy >> 8) as u8;

        let mem_spl = mem.read(self.sp);
        let mem_sph = mem.read(self.sp.wrapping_add(1));

        mem.write(self.sp, iyl);
        mem.write(self.sp.wrapping_add(1), iyh);

        self.iy = (mem_sph as u16) << 8 | mem_spl as u16;

        // T states
        23
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ex_mem_sp_iy() {
        let bytes = &mut [0xDD, 0xE3, 0x90, 0x48];
        let mut ram = Ram::new(bytes);
        let mut z80 = Z80::new();
        z80.sp = 2;
        z80.iy = 0x3988;

        let t_states = z80.ex_mem_sp_iy(&mut ram);
        assert_eq!(23, t_states);

        assert_eq!(0x4890, z80.iy);
        assert_eq!(0x88, ram.read(2));
        assert_eq!(0x39, ram.read(3));
    }
}
