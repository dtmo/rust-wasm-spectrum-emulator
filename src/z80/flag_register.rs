use std::ops::Not;

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

fn flags_set(flag_bitmask: &u8, register: &u8) -> bool {
    flag_bitmask & register == *flag_bitmask
}

fn unset_flags(flag_bitmask: &u8, register: &mut u8) {
    *register = *register & flag_bitmask.not();
}

fn set_flags(flag_bitmask: &u8, register: &mut u8) {
    unset_flags(flag_bitmask, register);
    *register = *register | *flag_bitmask;
}

fn set_flags_with(flag_bitmask: &u8, register: &mut u8, value: bool) {
    if (value) {
        set_flags(flag_bitmask, register);
    } else {
        unset_flags(flag_bitmask, register);
    }
}

pub fn s_flag_set(register: &u8) -> bool {
    flags_set(&S_FLAG_BITMASK, register)
}

pub fn set_s_flag(register: &mut u8) {
    set_flags(&S_FLAG_BITMASK, register);
}

pub fn unset_s_flag(register: &mut u8) {
    unset_flags(&S_FLAG_BITMASK, register);
}

pub fn set_s_flag_with(register: &mut u8, value: bool) {
    set_flags_with(&S_FLAG_BITMASK, register, value);
}

pub fn z_flag_set(register: &u8) -> bool {
    flags_set(&Z_FLAG_BITMASK, register)
}

pub fn set_z_flag(register: &mut u8) {
    set_flags(&Z_FLAG_BITMASK, register);
}

pub fn unset_z_flag(register: &mut u8) {
    unset_flags(&Z_FLAG_BITMASK, register);
}

pub fn set_z_flag_with(register: &mut u8, value: bool) {
    set_flags_with(&Z_FLAG_BITMASK, register, value);
}

pub fn y_flag_set(register: &u8) -> bool {
    flags_set(&Y_FLAG_BITMASK, register)
}

pub fn set_y_flag(register: &mut u8) {
    set_flags(&Y_FLAG_BITMASK, register);
}

pub fn unset_y_flag(register: &mut u8) {
    unset_flags(&Y_FLAG_BITMASK, register);
}

pub fn set_y_flag_with(register: &mut u8, value: bool) {
    set_flags_with(&Y_FLAG_BITMASK, register, value);
}

pub fn h_flag_set(register: &u8) -> bool {
    flags_set(&H_FLAG_BITMASK, register)
}

pub fn set_h_flag(register: &mut u8) {
    set_flags(&H_FLAG_BITMASK, register);
}

pub fn unset_h_flag(register: &mut u8) {
    unset_flags(&H_FLAG_BITMASK, register);
}

pub fn set_h_flag_with(register: &mut u8, value: bool) {
    set_flags_with(&H_FLAG_BITMASK, register, value);
}

pub fn x_flag_set(register: &u8) -> bool {
    flags_set(&X_FLAG_BITMASK, register)
}

pub fn set_x_flag(register: &mut u8) {
    set_flags(&X_FLAG_BITMASK, register);
}

pub fn unset_x_flag(register: &mut u8) {
    unset_flags(&X_FLAG_BITMASK, register);
}

pub fn set_x_flag_with(register: &mut u8, value: bool) {
    set_flags_with(&X_FLAG_BITMASK, register, value);
}

pub fn p_flag_set(register: &u8) -> bool {
    flags_set(&P_FLAG_BITMASK, register)
}

pub fn set_p_flag(register: &mut u8) {
    set_flags(&P_FLAG_BITMASK, register);
}

pub fn unset_p_flag(register: &mut u8) {
    unset_flags(&P_FLAG_BITMASK, register);
}

pub fn set_p_flag_with(register: &mut u8, value: bool) {
    set_flags_with(&P_FLAG_BITMASK, register, value);
}

pub fn n_flag_set(register: &u8) -> bool {
    flags_set(&N_FLAG_BITMASK, register)
}

pub fn set_n_flag(register: &mut u8) {
    set_flags(&N_FLAG_BITMASK, register);
}

pub fn unset_n_flag(register: &mut u8) {
    unset_flags(&N_FLAG_BITMASK, register);
}

pub fn set_n_flag_with(register: &mut u8, value: bool) {
    set_flags_with(&N_FLAG_BITMASK, register, value);
}

pub fn c_flag_set(register: &u8) -> bool {
    flags_set(&C_FLAG_BITMASK, register)
}

pub fn set_c_flag(register: &mut u8) {
    set_flags(&C_FLAG_BITMASK, register);
}

pub fn unset_c_flag(register: &mut u8) {
    unset_flags(&C_FLAG_BITMASK, register);
}

pub fn set_c_flag_with(register: &mut u8, value: bool) {
    set_flags_with(&C_FLAG_BITMASK, register, value);
}

mod tests {
    use super::*;

    #[test]
    fn test_unset_flags() {
        let flag_register = &mut 0b11111111;

        unset_flags(&S_FLAG_BITMASK, flag_register);

        assert_eq!(&0b01111111, flag_register);
    }

    #[test]
    fn test_s_flag() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_s_flag(flag_register);

        assert_eq!(true, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        unset_s_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_set_s_flag_with() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_s_flag_with(flag_register, true);

        assert_eq!(true, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_s_flag_with(flag_register, false);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_z_flag() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_z_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(true, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        unset_z_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_set_z_flag_with() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_z_flag_with(flag_register, true);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(true, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_z_flag_with(flag_register, false);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_y_flag() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_y_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(true, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        unset_y_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_set_y_flag_with() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_y_flag_with(flag_register, true);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(true, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_y_flag_with(flag_register, false);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_h_flag() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_h_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(true, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        unset_h_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_set_h_flag_with() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_h_flag_with(flag_register, true);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(true, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_h_flag_with(flag_register, false);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_x_flag() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_x_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(true, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        unset_x_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_set_x_flag_with() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_x_flag_with(flag_register, true);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(true, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_x_flag_with(flag_register, false);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_p_flag() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_p_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(true, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        unset_p_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_set_p_flag_with() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_p_flag_with(flag_register, true);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(true, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_p_flag_with(flag_register, false);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_n_flag() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_n_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(true, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        unset_n_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_set_n_flag_with() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_n_flag_with(flag_register, true);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(true, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_n_flag_with(flag_register, false);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_c_flag() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_c_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(true, c_flag_set(flag_register));

        unset_c_flag(flag_register);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }

    #[test]
    fn test_set_c_flag_with() {
        let flag_register = &mut 0u8;

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));

        set_c_flag_with(flag_register, true);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(true, c_flag_set(flag_register));

        set_c_flag_with(flag_register, false);

        assert_eq!(false, s_flag_set(flag_register));
        assert_eq!(false, z_flag_set(flag_register));
        assert_eq!(false, y_flag_set(flag_register));
        assert_eq!(false, h_flag_set(flag_register));
        assert_eq!(false, x_flag_set(flag_register));
        assert_eq!(false, p_flag_set(flag_register));
        assert_eq!(false, n_flag_set(flag_register));
        assert_eq!(false, c_flag_set(flag_register));
    }
}
