use neige_infra::state::GetApi;

use crate::state::LuaState;

#[allow(unused_variables)]
impl GetApi for LuaState {
    fn new_tabel(&mut self) {
        todo!()
    }

    fn create_tabel(&mut self, n_arr: usize, n_rec: usize) {
        todo!()
    }

    fn get_tabel(&mut self, idx: isize) -> neige_infra::LuaType {
        todo!()
    }

    fn get_field(&mut self, idx: isize, key: String) -> neige_infra::LuaType {
        todo!()
    }

    fn get_i(&mut self, idx: isize, i: i64) -> neige_infra::LuaType {
        todo!()
    }

    fn get_meta_table(&mut self, idx: isize) -> bool {
        todo!()
    }

    fn raw_get(&mut self, idx: isize) -> neige_infra::LuaType {
        todo!()
    }

    fn raw_get_i(&mut self, idx: isize, i: i64) -> neige_infra::LuaType {
        todo!()
    }

    fn get_global(&mut self, name: String) -> neige_infra::LuaType {
        todo!()
    }
}
