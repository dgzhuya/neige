#[allow(dead_code)]
pub trait LuaApi {
    fn get_top(&self) -> isize;
}
