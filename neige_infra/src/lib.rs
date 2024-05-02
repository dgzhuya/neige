mod code;
// mod constant;
// mod math;
mod proto;
mod state;
// mod value;

// pub use constant::{LUAI_MAXSTACK, LUA_MINSTACK, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS};
pub use proto::proto::{Constant, LocVar, Prototype, Upvalue};
