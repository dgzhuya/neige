use anyhow::{anyhow, Ok, Result};

pub enum Instruction {
    Move(u8, u16, u16),
    LoadK(u8, u32),
    LoadKx(u8, u32),
    LoadBool(u8, u16, u16),
    LoadNil(u8),

    GetUpVal(u8, u16, u16),
    GetTabUp(u8, u16, u16),
    GetTable(u8, u16, u16),
    SetTabUp(u8, u16, u16),
    SetUpVal(u8, u16, u16),
    SetTable(u8, u16, u16),
    NetTable(u8, u16, u16),

    _Self(),

    Add(u8, u16, u16),
    Sub(u8, u16, u16),
    Mul(u8, u16, u16),
    Mod(u8, u16, u16),
    Pow(u8, u16, u16),
    Div(u8, u16, u16),
    IDiv(u8, u16, u16),
    BAnd(u8, u16, u16),
    BOr(u8, u16, u16),
    BXor(u8, u16, u16),
    Shl(u8, u16, u16),
    Shr(u8, u16, u16),
    Unm(u8, u16, u16),
    BNot(u8, u16, u16),
    Not(u8, u16, u16),

    Length(u8, u16, u16),
    Concat(u8, u16, u16),
    Jmp(u8, u16, u16),

    Eq(u8, u16, u16),
    Lt(u8, u16, u16),
    Le(u8, u16, u16),

    Test(u8, u16, u16),
    TestSet(u8, u16, u16),

    Call(u8, u16, u16),
    TailCall(u8, u16, u16),
    Return(u8, u16, u16),
    ForLoop(u8, u16, u16),
    ForPrep(u8, u16, u16),

    TForCall(u8, u16, u16),
    TForLoop(u8, u16, u16),

    SetList(u8, u16, u16),

    Closure(u8, u16, u16),
    Vararg(u8, u16, u16),
    ExtraArg(u32),
}
