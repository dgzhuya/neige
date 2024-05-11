use super::value::LuaValue;

#[derive(Debug, Clone)]
pub struct LuaUpval {
    pub val: LuaValue,
}

impl LuaUpval {
    pub fn new(val: LuaValue) -> Self {
        Self { val }
    }

    pub fn set_val(&mut self, val: LuaValue) {
        self.val = val
    }
}
