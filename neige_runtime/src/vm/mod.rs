mod arith;
mod call;
mod utils;

use neige_infra::{
    code::inst::Instruction,
    state::{
        AccessApi, ArithApi, CallApi, CompareApi, GetApi, LuaVm, MiscApi, PushApi, SetApi, StackApi,
    },
    value::fpb::fb_2_isize,
    LuaArith, LuaCompare,
};
use utils::{i32_isize, lua_upvalue_index, u16_isize, u32_isize, u8_isize};

use crate::state::LuaState;

impl LuaState {
    pub fn execute(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Move(a, b, _) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b) + 1;
                self.copy(b, a)
            }
            Instruction::LoadK(a, bx) => {
                let a = u8_isize(a) + 1;
                let bx = u32_isize(bx);
                self.get_const(bx);
                self.replace(a)
            }
            Instruction::LoadKx(a, _) => {
                let a = u8_isize(a) + 1;
                let inst = self.fetch();
                if let Instruction::ExtraArg(ax) = inst {
                    let ax = u32_isize(&ax);
                    self.get_const(ax);
                    self.replace(a)
                }
            }
            Instruction::LoadBool(a, b, c) => {
                let a = u8_isize(a) + 1;
                self.push_boolean(*b != 0);
                self.replace(a);
                if *c != 0 {
                    self.add_pc(1)
                }
            }
            Instruction::LoadNil(a, b, _) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b);
                self.push_nil();
                for i in a..=b {
                    self.copy(-1, i)
                }
                self.pop(1)
            }
            Instruction::GetUpVal(a, b, _) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b) + 1;
                self.copy(lua_upvalue_index(b), a)
            }
            Instruction::GetTabUp(a, b, c) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b) + 1;
                let c = u16_isize(c);
                self.get_rk(c);
                self.get_table(lua_upvalue_index(b));
                self.replace(a)
            }
            Instruction::GetTable(a, b, c) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b) + 1;
                let c = u16_isize(c);
                self.get_rk(c);
                self.get_table(b);
                self.replace(a);
            }
            Instruction::SetTabUp(a, b, c) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b);
                let c = u16_isize(c);
                self.get_rk(b);
                self.get_rk(c);
                self.set_table(lua_upvalue_index(a))
            }
            Instruction::SetUpVal(a, b, _) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b) + 1;
                self.copy(a, lua_upvalue_index(b))
            }
            Instruction::SetTable(a, b, c) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b);
                let c = u16_isize(c);
                self.get_rk(b);
                self.get_rk(c);
                self.set_table(a)
            }
            Instruction::NetTable(a, b, c) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b);
                let c = u16_isize(c);
                self.create_table(fb_2_isize(b), fb_2_isize(c));
                self.replace(a)
            }
            Instruction::_Self(a, b, c) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b) + 1;
                let c = u16_isize(c);
                self.copy(b, a + 1);
                self.get_rk(c);
                self.get_table(b);
                self.replace(a);
            }
            Instruction::Add(a, b, c) => self.binary_arith(LuaArith::Add, a, b, c),
            Instruction::Sub(a, b, c) => self.binary_arith(LuaArith::Sub, a, b, c),
            Instruction::Mul(a, b, c) => self.binary_arith(LuaArith::Mul, a, b, c),
            Instruction::Mod(a, b, c) => self.binary_arith(LuaArith::Mod, a, b, c),
            Instruction::Pow(a, b, c) => self.binary_arith(LuaArith::Pow, a, b, c),
            Instruction::Div(a, b, c) => self.binary_arith(LuaArith::Div, a, b, c),
            Instruction::IDiv(a, b, c) => self.binary_arith(LuaArith::IDiv, a, b, c),
            Instruction::BAnd(a, b, c) => self.binary_arith(LuaArith::BAnd, a, b, c),
            Instruction::Bor(a, b, c) => self.binary_arith(LuaArith::Bor, a, b, c),
            Instruction::BXor(a, b, c) => self.binary_arith(LuaArith::BXor, a, b, c),
            Instruction::Shl(a, b, c) => self.binary_arith(LuaArith::Shl, a, b, c),
            Instruction::Shr(a, b, c) => self.binary_arith(LuaArith::Shr, a, b, c),
            Instruction::Unm(a, b, _) => self.unary_arith(LuaArith::Unm, a, b),
            Instruction::BNot(a, b, _) => self.unary_arith(LuaArith::BNot, a, b),
            Instruction::Not(a, b, _) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b) + 1;
                self.push_boolean(!self.to_boolean(b));
                self.replace(a)
            }
            Instruction::Length(a, b, _) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b) + 1;
                self.len(b);
                self.replace(a)
            }
            Instruction::Concat(a, b, c) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b) + 1;
                let c = u16_isize(c) + 1;
                let n = (c - b + 1) as usize;
                self.check_stack(n);
                for i in b..=c {
                    self.push_value(i)
                }
                self.concat(n);
                self.replace(a)
            }
            Instruction::Jmp(a, sbx) => {
                let a = u8_isize(a);
                let sbx = i32_isize(sbx);
                self.add_pc(sbx);
                if a != 0 {
                    self.close_upvalue(a)
                }
            }
            Instruction::Eq(a, b, c) => self.inline_compare(LuaCompare::Eq, a, b, c),
            Instruction::Lt(a, b, c) => self.inline_compare(LuaCompare::Lt, a, b, c),
            Instruction::Le(a, b, c) => self.inline_compare(LuaCompare::Le, a, b, c),
            Instruction::Test(a, _, c) => {
                let a = u8_isize(a) + 1;
                let c = u16_isize(c);
                if self.to_boolean(a) != (c != 0) {
                    self.add_pc(1)
                }
            }
            Instruction::TestSet(a, b, c) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b) + 1;
                let c = u16_isize(c);
                if self.to_boolean(b) != (c != 0) {
                    self.copy(b, a)
                } else {
                    self.add_pc(1)
                }
            }
            Instruction::Call(a, b, c) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b);
                let c = u16_isize(c);
                let n_args = self.push_func_and_args(a, b);
                self.call(n_args, c - 1);
                self.pop_result(a, c)
            }
            Instruction::TailCall(a, b, _) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b);
                let n_args = self.push_func_and_args(a, b);
                self.call(n_args, -1);
                self.pop_result(a, 0)
            }
            Instruction::Return(a, b, _) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b);
                if b > 1 {
                    self.check_stack(b as usize - 1);
                    for i in a..=a + b - 2 {
                        self.push_value(i)
                    }
                } else if b < 1 {
                    self.fix_stack(a)
                }
            }
            Instruction::ForLoop(a, sbx) => {
                let a = u8_isize(a) + 1;
                let sbx = i32_isize(sbx);
                self.push_value(a + 2);
                self.push_value(a);
                self.arith(neige_infra::LuaArith::Add);
                self.replace(a);
                let is_positive_step = self.to_number(a + 2) >= 0.0;
                if (is_positive_step && self.compare(a, a + 1, neige_infra::LuaCompare::Le))
                    || (!is_positive_step && self.compare(a + 1, a, neige_infra::LuaCompare::Le))
                {
                    self.add_pc(sbx);
                    self.copy(a, a + 3)
                }
            }
            Instruction::ForPrep(a, sbx) => {
                let a = u8_isize(a) + 1;
                let sbx = i32_isize(sbx);

                self.push_value(a);
                self.push_value(a + 2);
                self.arith(LuaArith::Sub);
                self.replace(a);

                self.add_pc(sbx)
            }
            Instruction::TForCall(a, _, c) => {
                let a = u8_isize(a) + 1;
                let c = u16_isize(c);
                self.push_func_and_args(a, 3);
                self.call(2, c);
                self.pop_result(a + 3, c + 1)
            }
            Instruction::TForLoop(a, sbx) => {
                let a = u8_isize(a) + 1;
                let sbx = i32_isize(sbx);
                if !self.is_nil(a + 1) {
                    self.copy(a + 1, a);
                    self.add_pc(sbx)
                }
            }
            Instruction::SetList(a, b, c) => {
                let a = u8_isize(a) + 1;
                let b_is_zero = *b == 0;
                let b = if b_is_zero {
                    let b = self.to_integer(-1) as isize;
                    self.pop(1);
                    b - a - 1
                } else {
                    u16_isize(b)
                };
                let c = if *c > 0 {
                    u16_isize(c) - 1
                } else {
                    if let Instruction::ExtraArg(ax) = self.fetch() {
                        u32_isize(&ax)
                    } else {
                        panic!("SetList error")
                    }
                };
                let mut idx = (c * 50) as i64;
                for j in 1..=b {
                    idx += 1;
                    self.push_value(a + j);
                    self.set_i(a, idx)
                }

                if b_is_zero {
                    let register_count = self.register_count() as isize;
                    for j in register_count..=self.get_top() {
                        idx += 1;
                        self.push_value(j);
                        self.set_i(a, idx)
                    }
                    self.set_top(register_count)
                }
            }
            Instruction::Closure(a, bx) => {
                let a = u8_isize(a) + 1;
                let bx = u32_isize(bx);
                self.load_proto(bx);
                self.replace(a)
            }
            Instruction::Vararg(a, b, _) => {
                let a = u8_isize(a) + 1;
                let b = u16_isize(b);
                if b != 1 {
                    self.load_vararg(b - 1);
                    self.pop_result(a, b)
                }
            }
            Instruction::ExtraArg(_) => todo!(),
        }
    }
}
