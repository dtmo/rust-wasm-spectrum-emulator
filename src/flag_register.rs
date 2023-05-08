use crate::register::Register;

/// Set if the 2-complement value is negative. It’s simply a copy of the
/// most significant bit.
pub const S_FLAG_BITMASK: u8 = 0b10000000;

/// Set if the result is zero.
pub const Z_FLAG_BITMASK: u8 = 0b01000000;

/// A copy of bit 5 of the result.
pub const Y_FLAG_BITMASK: u8 = 0b00100000;

/// The half-carry of an addition/subtraction (from bit 3 to 4). Needed for
/// BCD correction with DAA.
pub const H_FLAG_BITMASK: u8 = 0b00010000;

/// A copy of bit 3 of the result.
pub const X_FLAG_BITMASK: u8 = 0b00001000;

/// This flag can either be the parity of the result (PF), or the
/// 2-compliment signed overflow (VF): set if 2-compliment value doesn’t fit
/// in the register.
pub const P_FLAG_BITMASK: u8 = 0b00000100;

/// Shows whether the last operation was an addition (0) or an subtraction
/// (1). This information is needed for DAA.
pub const N_FLAG_BITMASK: u8 = 0b00000010;

/// The carry flag, set if there was a carry after the most significant bit.
pub const C_FLAG_BITMASK: u8 = 0b00000001;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlagRegister {
    value: u8,
}

impl Register for FlagRegister {
    fn get(&self) -> u8 {
        self.value
    }

    fn load(&mut self, r_prime: u8) {
        self.value = r_prime;
    }
}

impl FlagRegister {
    pub fn new() -> FlagRegister {
        FlagRegister { value: 0 }
    }

    fn flags(&self, flag_bitmask: u8) -> bool {
        self.value & flag_bitmask != 0
    }

    fn unset_flags(&mut self, flag_bitmask: u8) {
        self.value = self.value & flag_bitmask.reverse_bits();
    }

    fn set_flags(&mut self, flag_bitmask: u8) {
        self.unset_flags(flag_bitmask);
        self.value = self.value | flag_bitmask;
    }

    pub fn s_flag(&self) -> bool {
        self.flags(S_FLAG_BITMASK)
    }

    pub  fn set_s_flag(&mut self) {
        self.set_flags(S_FLAG_BITMASK);
    }

    pub fn unset_s_flag(&mut self) {
        self.unset_flags(S_FLAG_BITMASK);
    }

    pub fn z_flag(&self) -> bool {
        self.flags(Z_FLAG_BITMASK)
    }

    pub fn set_z_flag(&mut self) {
        self.set_flags(Z_FLAG_BITMASK);
    }

    pub fn unset_z_flag(&mut self) {
        self.unset_flags(Z_FLAG_BITMASK);
    }

    pub fn y_flag(&self) -> bool {
        self.flags(Y_FLAG_BITMASK)
    }

    pub fn set_y_flag(&mut self) {
        self.set_flags(Y_FLAG_BITMASK);
    }

    pub fn unset_y_flag(&mut self) {
        self.unset_flags(Y_FLAG_BITMASK);
    }

    pub fn h_flag(&self) -> bool {
        self.flags(H_FLAG_BITMASK)
    }

    pub fn set_h_flag(&mut self) {
        self.set_flags(H_FLAG_BITMASK);
    }

    pub fn unset_h_flag(&mut self) {
        self.unset_flags(H_FLAG_BITMASK);
    }

    pub fn x_flag(&self) -> bool {
        self.flags(X_FLAG_BITMASK)
    }

    pub fn set_x_flag(&mut self) {
        self.set_flags(X_FLAG_BITMASK);
    }

    pub fn unset_x_flag(&mut self) {
        self.unset_flags(X_FLAG_BITMASK);
    }

    pub fn p_flag(&self) -> bool {
        self.flags(P_FLAG_BITMASK)
    }

    pub fn set_p_flag(&mut self) {
        self.set_flags(P_FLAG_BITMASK);
    }

    pub fn unset_p_flag(&mut self) {
        self.unset_flags(P_FLAG_BITMASK);
    }

    pub fn n_flag(&self) -> bool {
        self.flags(N_FLAG_BITMASK)
    }

    pub fn set_n_flag(&mut self) {
        self.set_flags(N_FLAG_BITMASK);
    }

    pub fn unset_n_flag(&mut self) {
        self.unset_flags(N_FLAG_BITMASK);
    }

    pub fn c_flag(&self) -> bool {
        self.flags(C_FLAG_BITMASK)
    }

    pub fn set_c_flag(&mut self) {
        self.set_flags(C_FLAG_BITMASK);
    }

    pub fn unset_c_flag(&mut self) {
        self.unset_flags(C_FLAG_BITMASK);
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_get_load() {
        let mut flag_register = FlagRegister::new();

        assert_eq!(0x00, flag_register.get());

        flag_register.load(0xDD);

        assert_eq!(0xDD, flag_register.get());
    }

    #[test]
    fn test_s_flag() {
        let mut flag_register = FlagRegister::new();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.set_s_flag();

        assert_eq!(true, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.unset_s_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());
    }

    #[test]
    fn test_z_flag() {
        let mut flag_register = FlagRegister::new();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.set_z_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(true, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.unset_z_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());
    }

    #[test]
    fn test_y_flag() {
        let mut flag_register = FlagRegister::new();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.set_y_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(true, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.unset_y_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());
    }

    #[test]
    fn test_h_flag() {
        let mut flag_register = FlagRegister::new();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.set_h_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(true, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.unset_h_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());
    }

    #[test]
    fn test_x_flag() {
        let mut flag_register = FlagRegister::new();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.set_x_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(true, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.unset_x_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());
    }

    #[test]
    fn test_p_flag() {
        let mut flag_register = FlagRegister::new();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.set_p_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(true, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.unset_p_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());
    }

    #[test]
    fn test_n_flag() {
        let mut flag_register = FlagRegister::new();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.set_n_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(true, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.unset_n_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());
    }

    #[test]
    fn test_c_flag() {
        let mut flag_register = FlagRegister::new();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());

        flag_register.set_c_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(true, flag_register.c_flag());

        flag_register.unset_c_flag();

        assert_eq!(false, flag_register.s_flag());
        assert_eq!(false, flag_register.z_flag());
        assert_eq!(false, flag_register.y_flag());
        assert_eq!(false, flag_register.h_flag());
        assert_eq!(false, flag_register.x_flag());
        assert_eq!(false, flag_register.p_flag());
        assert_eq!(false, flag_register.n_flag());
        assert_eq!(false, flag_register.c_flag());
    }
}
