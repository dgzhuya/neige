use super::LuaApi;

pub trait LuaVm: LuaApi {
    fn pc(&self) -> isize;
    fn add_pc(&mut self, n: isize);
    fn fetch(&self) -> u32;
    fn get_const(&mut self, idx: isize);
    fn get_rk(&mut self, rk: isize);
    fn register_count(&self) -> u8;
    fn load_vararg(&mut self, n: isize);
    fn load_proto(&mut self, idx: isize);
    fn close_upvalue(&mut self, a: isize);
}
