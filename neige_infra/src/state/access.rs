use crate::constant::LuaType;

#[allow(dead_code)]
pub trait AccessApi {
    fn tp_name(&self, tp: i8) -> &str;
    fn ty_id(&self, idx: isize) -> LuaType;
    fn is_none(&self, idx: isize) -> bool;
}
