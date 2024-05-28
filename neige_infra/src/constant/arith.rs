#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LuaArith {
    Add,
    Sub,
    Mul,
    Mod,
    Pow,
    Div,
    IDiv,
    BAnd,
    Bor,
    BXor,
    Shl,
    Shr,
    Unm,
    BNot,
}

impl LuaArith {
    pub fn get_meta_name(&self) -> &str {
        match self {
            LuaArith::Add => "__add",
            LuaArith::Sub => "__sub",
            LuaArith::Mul => "__sub",
            LuaArith::Mod => "__mod",
            LuaArith::Pow => "__pow",
            LuaArith::Div => "__div",
            LuaArith::IDiv => "__idiv",
            LuaArith::BAnd => "__band",
            LuaArith::Bor => "__bor",
            LuaArith::BXor => "__bxor",
            LuaArith::Shl => "__shl",
            LuaArith::Shr => "__shr",
            LuaArith::Unm => "__unm",
            LuaArith::BNot => "__bnot",
        }
    }
}
