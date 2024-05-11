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
