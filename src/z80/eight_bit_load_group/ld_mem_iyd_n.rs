use crate::z80::{Z80Memory, Z80};

impl Z80 {
    /// ## LD (IY+d), n
    ///
    /// ### Operation
    ///
    /// (IY+d) ← n
    ///
    /// ### Op Code
    ///
    /// LD
    ///
    /// ### Operands
    ///
    /// (IY+d), n
    /// `1 1 1 1 1 1 0 1` (FD)
    /// `0 0 1 1 0 1 1 0` (36)
    /// `d d d d d d d d`
    /// `n n n n n n n n`
    ///
    /// ### Description
    ///
    /// The n operand is loaded to the memory address specified by the sum of
    /// Index Register IX and the two’s complement displacement operand d.
    ///
    /// | M Cycles | T States         | 4 MHz E.T. |
    /// | -------- | ---------------- | ---------- |
    /// | 5        | 19 (4, 4, 3,5,3) | 4.75       |
    ///
    /// ### Condition Bits Affected
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// If Index Register IY contains the number 219Ah, then upon execution of
    /// an LD (IY+5h), 5Ah instruction, byte 5Ah is contained in memory address
    /// 219Fh.
    pub fn ld_mem_iyd_n(&mut self, mem: &mut dyn Z80Memory) -> u8 {
        let displacement = self.fetch_next_opcode(mem);
        let n = self.fetch_next_opcode(mem);
        let address = self.iy.wrapping_add_signed(displacement as i16);
        mem.write(address, n);

        // T states
        19
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::tests::Ram;

    #[test]
    fn test_ld_mem_iyd_n() {
        let bytes = &mut [0xDD, 0x26, 0x02, 0xFF, 0x00];
        let ram = &mut Ram::new(bytes);

        let z80 = &mut Z80::new();
        z80.program_counter = 2;
        z80.iy = 0x02;

        let t_states = z80.ld_mem_iyd_n(ram);

        assert_eq!(19, t_states);
        assert_eq!(0xFF, ram.read(4));
    }
}
