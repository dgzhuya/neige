mod arith;
mod compare;
mod token;
mod types;

pub use types::LuaType;

pub const LUA_MINSTACK: usize = 20;
pub const LUAI_MAXSTACK: usize = 1000000;
pub const LUA_REGISTRYINDEX: isize = -(LUAI_MAXSTACK as isize) - 1000;
pub const LUA_RIDX_GLOBALS: isize = 2;
