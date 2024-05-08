use super::value::LuaValue;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaUpval {
    pub val: LuaValue,
}
