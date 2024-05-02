mod convert;
pub mod inst;
pub mod inst_mode;

pub const MAX_ARG_BX: i32 = 1 << 18 - 1;
pub const MAX_ARG_S_BX: i32 = MAX_ARG_BX >> 1;
