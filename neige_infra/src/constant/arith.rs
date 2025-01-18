/// 运算符号
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LuaArith {
    Add,  // 加
    Sub,  // 减
    Mul,  // 乘
    Mod,  // 取模
    Pow,  // 幂
    Div,  // 除
    IDiv, // 整除
    BAnd, // 位与
    Bor,  // 位或
    BXor, // 位异或
    Shl,  // 左移
    Shr,  // 右移
    Unm,  // 取反
    BNot, // 位取反
}

impl LuaArith {
    /// 获取元方法名称
    ///
    /// @return &str 元方法名称
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
