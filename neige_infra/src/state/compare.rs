use std::usize;

use crate::constant::LuaCompare;

pub trait CompareApi {
    fn compare(&self, idx1: isize, idx2: isize, op: LuaCompare) -> bool;
    fn raw_equal(&self, idx1: isize, idx2: isize) -> bool;
}
