use super::{convert::DecodeOrder, inst_mode::OpMode};

/// Lua指令集
/// Kst(B) 从常量索引B加载数据
/// Rk(B) 从变量或常量中加载数据，B>0x100则从常量中加载
/// UpValue[B] 获取上值索引B中的数据
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Move(u8, u16, u16),     // R(A) := R(B)
    LoadK(u8, u32),         // R(A) := Kst(Bx)
    LoadKx(u8, u32),        // R(A) := Kst(extra arg)
    LoadBool(u8, u16, u16), // R(A) := (bool)B; if(C) pc++
    LoadNil(u8, u16, u16),  // R(A)..R(A+B) := nil
    GetUpVal(u8, u16, u16), // R(A) := UpValue[B]
    GetTabUp(u8, u16, u16), // R(A) := UpValue[B][Rk(C)]
    GetTable(u8, u16, u16), // R(A) := R(B)[Rk(C)]
    SetTabUp(u8, u16, u16), // UpValue[A][Rk(B)] := Rk(C)
    SetUpVal(u8, u16, u16), // UpVlaue[B] := R(A)
    SetTable(u8, u16, u16), // R(A)[Rk(B)] := Rk(C)
    NetTable(u8, u16, u16), // R(A) := {} (size = B,C)
    _Self(u8, u16, u16),    // R(A+1) := R(B);R(A) := R(B)[Rk(C)]
    Add(u8, u16, u16),      // R(A) := Rk(B) + Rk(C)
    Sub(u8, u16, u16),      // R(A) := Rk(B) - Rk(C)
    Mul(u8, u16, u16),      // R(A) := Rk(B) * Rk(C)
    Mod(u8, u16, u16),      // R(A) := Rk(B) % Rk(C)
    Pow(u8, u16, u16),      // R(A) := Rk(B) ^ Rk(C)
    Div(u8, u16, u16),      // R(A) := Rk(B) / Rk(C)
    IDiv(u8, u16, u16),     // R(A) := Rk(B) // Rk(C)
    BAnd(u8, u16, u16),     // R(A) := Rk(B) & Rk(C)
    Bor(u8, u16, u16),      // R(A) := Rk(B) | Rk(C)
    BXor(u8, u16, u16),     // R(A) := Rk(B) ~ Rk(C)
    Shl(u8, u16, u16),      // R(A) := Rk(B) << Rk(C)
    Shr(u8, u16, u16),      // R(A) := Rk(B) >> Rk(C)
    Unm(u8, u16, u16),      // R(A) := -Rk(B)
    BNot(u8, u16, u16),     // R(A) := ~Rk(B)
    Not(u8, u16, u16),      // R(A) := not Rk(B)
    Length(u8, u16, u16),   // R(A) := length of Rk(B)
    Concat(u8, u16, u16),   // R(A) := R(B)..R(C)
    Jmp(u8, i32),           // pc+=sBx;if(A) close all upvalues >= R(A-1)
    Eq(u8, u16, u16),       // if((Rk(B) == Rk(C)) ~= A) then pc++
    Lt(u8, u16, u16),       // if((Rk(B) < Rk(C)) ~= A) then pc++
    Le(u8, u16, u16),       // if((Rk(B) <= Rk(C)) ~= A) then pc++
    Test(u8, u16, u16),     // if not (R(A)<==> C) then pc++
    TestSet(u8, u16, u16),  // if(R(A)<==> C) then R(A) := R(B) else pc++
    Call(u8, u16, u16),     // R(A)..R(A+C-2) := R(A)(R(A+1)..R(A+B-1))
    TailCall(u8, u16, u16), // return R(R(A+1)..R(A+B-1))
    Return(u8, u16, u16),   // return R(A)..R(A+B-2)
    ForLoop(u8, i32),       // R(A) += R(A+2); if(R(A) <= R(A+1)) then { pc+=sBx; R(A+3):=R(A) }
    ForPrep(u8, i32),       // R(A)-=R(A=2); pc+=sBx
    TForCall(u8, u16, u16), // R(A+3)..R(A+2+C) := R(A)(R(A+1),R(A+2))
    TForLoop(u8, i32),      // if(R(A+1) ~= nil) then { R(A):=R(A+1);pc+=sBx  }
    SetList(u8, u16, u16),  // R(A)[(C-1)*FPF+i] := R(A+i),1<=i<=B
    Closure(u8, u32),       // R(A) := closure(KPROTO[Bx])
    Vararg(u8, u16, u16),   // R(A)..R(A+B-2) := vararg
    ExtraArg(u32),          // extra (larger) argument for previous opcode
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let op_code = value.get_op();
        let op_mode: OpMode = value.into();
        match op_mode {
            OpMode::IABC(a, b, c) => match op_code {
                0 => Instruction::Move(a, b, c),
                3 => Instruction::LoadBool(a, b, c),
                4 => Instruction::LoadNil(a, b, c),
                5 => Instruction::GetUpVal(a, b, c),
                6 => Instruction::GetTabUp(a, b, c),
                7 => Instruction::GetTable(a, b, c),
                8 => Instruction::SetTabUp(a, b, c),
                9 => Instruction::SetUpVal(a, b, c),
                10 => Instruction::SetTable(a, b, c),
                11 => Instruction::NetTable(a, b, c),
                12 => Instruction::_Self(a, b, c),
                13 => Instruction::Add(a, b, c),
                14 => Instruction::Sub(a, b, c),
                15 => Instruction::Mul(a, b, c),
                16 => Instruction::Mod(a, b, c),
                17 => Instruction::Pow(a, b, c),
                18 => Instruction::Div(a, b, c),
                19 => Instruction::IDiv(a, b, c),
                20 => Instruction::BAnd(a, b, c),
                21 => Instruction::Bor(a, b, c),
                22 => Instruction::BXor(a, b, c),
                23 => Instruction::Shl(a, b, c),
                24 => Instruction::Shr(a, b, c),
                25 => Instruction::Unm(a, b, c),
                26 => Instruction::BNot(a, b, c),
                27 => Instruction::Not(a, b, c),
                28 => Instruction::Length(a, b, c),
                29 => Instruction::Concat(a, b, c),
                31 => Instruction::Eq(a, b, c),
                32 => Instruction::Lt(a, b, c),
                33 => Instruction::Le(a, b, c),
                34 => Instruction::Test(a, b, c),
                35 => Instruction::TestSet(a, b, c),
                36 => Instruction::Call(a, b, c),
                37 => Instruction::TailCall(a, b, c),
                38 => Instruction::Return(a, b, c),
                41 => Instruction::TForCall(a, b, c),
                43 => Instruction::SetList(a, b, c),
                45 => Instruction::Vararg(a, b, c),
                over_code => panic!("code {over_code} is overflow!"),
            },
            OpMode::IABx(a, bx) => match op_code {
                1 => Instruction::LoadK(a, bx),
                2 => Instruction::LoadKx(a, bx),
                44 => Instruction::Closure(a, bx),
                over_code => panic!("code {over_code} is overflow!"),
            },
            OpMode::IAsBx(a, s_bx) => match op_code {
                30 => Instruction::Jmp(a, s_bx),
                39 => Instruction::ForLoop(a, s_bx),
                40 => Instruction::ForPrep(a, s_bx),
                42 => Instruction::TForLoop(a, s_bx),
                over_code => panic!("code {over_code} is overflow!"),
            },
            OpMode::IAx(ax) => Instruction::ExtraArg(ax),
        }
    }
}
