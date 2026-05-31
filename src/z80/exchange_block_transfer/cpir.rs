use crate::z80::{
    register_flags::{
        set_h_flag_with, set_n_flag, set_p_flag_with, set_s_flag_with, set_x_flag_with,
        set_y_flag_with, set_z_flag_with, S_FLAG_BITMASK,
    },
    Z80Memory, Z80,
};

impl Z80 {
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z80::{
        register_flags::{h_flag, n_flag, p_flag, s_flag, z_flag},
        tests::Ram,
    };
}
