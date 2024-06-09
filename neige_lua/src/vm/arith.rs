use neige_infra::{LuaArith, LuaCompare};

use crate::{
    api::{ArithApi, CompareApi, LuaVm, StackApi},
    state::LuaState,
};

use super::utils::{u16_isize, u8_isize};

impl LuaState {
    pub(super) fn inline_compare(&mut self, op: LuaCompare, a: &u8, b: &u16, c: &u16) {
        let a = u8_isize(a);
        let b = u16_isize(b);
        let c = u16_isize(c);
        self.get_rk(b);
        self.get_rk(c);
        if self.compare(-2, -1, op) != (a != 0) {
            self.add_pc(1)
        }
        self.pop(2)
    }

    pub(super) fn binary_arith(&mut self, op: LuaArith, a: &u8, b: &u16, c: &u16) {
        let a = u8_isize(a) + 1;
        let b = u16_isize(b);
        let c = u16_isize(c);
        self.get_rk(b);
        self.get_rk(c);
        self.arith(op);
        self.replace(a);
    }

    pub(super) fn unary_arith(&mut self, op: LuaArith, a: &u8, b: &u16) {
        let a = u8_isize(a) + 1;
        let b = u16_isize(b) + 1;
        self.push_value(b);
        self.arith(op);
        self.replace(a);
    }
}
