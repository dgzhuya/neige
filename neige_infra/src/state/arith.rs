use crate::LuaArith;

pub trait ArithApi {
    fn arith(&self, op: LuaArith);
}
