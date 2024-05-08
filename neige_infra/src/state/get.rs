use crate::LuaType;

pub trait GetApi {
    fn new_tabel(&mut self);
    fn create_tabel(&mut self, n_arr: usize, n_rec: usize);
    fn get_tabel(&mut self, idx: isize) -> LuaType;
    fn get_field(&mut self, idx: isize, key: String) -> LuaType;
    fn get_i(&mut self, idx: isize, i: i64) -> LuaType;
    fn get_meta_table(&mut self, idx: isize) -> bool;
    fn raw_get(&mut self, idx: isize) -> LuaType;
    fn raw_get_i(&mut self, idx: isize, i: i64) -> LuaType;
    fn get_global(&mut self, name: String) -> LuaType;
}
