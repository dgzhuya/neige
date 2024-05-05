use super::{access::AccessApi, stack::StackApi};

pub trait LuaApi: AccessApi + StackApi {}
