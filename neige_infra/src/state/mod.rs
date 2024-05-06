mod access;
mod push;
mod stack;
pub mod vm;

pub use self::{access::AccessApi, push::PushApi, stack::StackApi, vm::LuaVm};

pub trait LuaApi: AccessApi + StackApi + PushApi {}
