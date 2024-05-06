use neige_infra::state::StackApi;

use crate::state::LuaState;

#[allow(unused_variables)]
impl StackApi for LuaState {
    fn get_top(&self) -> isize {
        todo!()
    }

    fn abs_index(&self, idx: isize) -> isize {
        todo!()
    }

    fn check_stack(&mut self, n: usize) -> bool {
        todo!()
    }

    fn pop(&mut self, n: u32) {
        todo!()
    }

    fn copy(&mut self, from_idx: isize, to_idx: isize) {
        todo!()
    }

    fn push_value(&mut self, idx: isize) {
        todo!()
    }

    fn replace(&mut self, idx: isize) {
        todo!()
    }

    fn insert(&mut self, idx: isize) {
        todo!()
    }

    fn remove(&mut self, idx: isize) {
        todo!()
    }

    fn rotate(&mut self, idx: isize, n: isize) {
        todo!()
    }

    fn set_top(&mut self, idx: isize) {
        todo!()
    }
}
