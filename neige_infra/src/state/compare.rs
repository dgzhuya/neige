use crate::constant::LuaCompare;

/// 比较计算API
///
/// 数量 `2`
pub trait CompareApi {
    fn compare(&mut self, idx1: isize, idx2: isize, op: LuaCompare) -> bool;
    fn raw_equal(&mut self, idx1: isize, idx2: isize) -> bool;
}
