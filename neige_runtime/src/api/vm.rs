use neige_infra::state::LuaVm;

use crate::state::LuaState;

#[allow(unused_variables)]
impl LuaVm for LuaState {
    fn pc(&self) -> isize {
        todo!()
    }

    fn add_pc(&mut self, n: isize) {
        todo!()
    }

    fn fetch(&self) -> u32 {
        todo!()
    }

    fn get_const(&mut self, idx: isize) {
        todo!()
    }

    fn get_rk(&mut self, rk: isize) {
        todo!()
    }

    fn register_count(&self) -> u8 {
        todo!()
    }

    fn load_vararg(&mut self, n: isize) {
        todo!()
    }

    fn load_proto(&mut self, idx: isize) {
        todo!()
    }

    fn close_upvalue(&mut self, a: isize) {
        todo!()
    }
}
