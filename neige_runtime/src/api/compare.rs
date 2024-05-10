use neige_infra::{state::CompareApi, LuaCompare};

use crate::state::LuaState;

#[allow(unused_variables)]
impl CompareApi for LuaState {
    fn compare(&self, idx1: isize, idx2: isize, op: LuaCompare) -> bool {
        todo!()
    }

    fn raw_equal(&self, idx1: isize, idx2: isize) -> bool {
        todo!()
    }
}
