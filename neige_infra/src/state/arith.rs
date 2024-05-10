use crate::LuaArith;

/// 计算API
///
/// 数量 `1`
pub trait ArithApi {
    fn arith(&self, op: LuaArith);
}
