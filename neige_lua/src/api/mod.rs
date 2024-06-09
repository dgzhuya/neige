mod access;
mod arith;
mod call;
mod compare;
mod get;
mod misc;
mod push;
mod set;
mod stack;
mod vm;

pub(crate) use vm::LuaVm;

pub use self::{
    access::AccessApi, arith::ArithApi, call::CallApi, compare::CompareApi, get::GetApi,
    misc::MiscApi, push::PushApi, set::SetApi, stack::StackApi,
};

pub trait LuaApi:
    AccessApi + ArithApi + CallApi + CompareApi + GetApi + MiscApi + PushApi + SetApi + StackApi
{
}
