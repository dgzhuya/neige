mod access;
mod arith;
mod call;
mod compare;
mod get;
mod misc;
mod push;
mod set;
mod stack;
pub mod vm;

pub use self::{
    access::AccessApi, arith::ArithApi, call::CallApi, compare::CompareApi, get::GetApi,
    misc::MiscApi, push::PushApi, set::SetApi, stack::StackApi, vm::LuaVm,
};

pub trait LuaApi:
    AccessApi + ArithApi + CallApi + CompareApi + GetApi + MiscApi + PushApi + SetApi + StackApi
{
}
