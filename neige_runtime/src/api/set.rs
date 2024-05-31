use neige_infra::state::SetApi;

use crate::state::LuaState;

#[allow(unused_variables)]
impl SetApi for LuaState {
    fn set_table(&mut self, idx: isize) {
        todo!()
    }

    fn set_field(&mut self, idx: isize, key: String) {
        todo!()
    }

    fn set_i(&mut self, idx: isize, i: i64) {
        todo!()
    }

    fn set_meta_table(&mut self, idx: isize) {
        todo!()
    }

    fn raw_set(&mut self, idx: isize) {
        todo!()
    }

    fn raw_set_i(&mut self, idx: isize, i: i64) {
        todo!()
    }

    fn set_global(&mut self, name: String) {
        todo!()
    }
}
