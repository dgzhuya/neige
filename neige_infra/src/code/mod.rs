pub mod convert;
pub mod inst;
pub mod inst_mode;

pub const MAX_ARG_BX: isize = (1 << 18) - 1;
pub const MAX_ARG_SBX: isize = MAX_ARG_BX >> 1;
