use neige_infra::state::ArithApi;

use crate::state::LuaState;
use neige_infra::LuaArith as Arith;

impl ArithApi for LuaState {
    fn arith(&self, op: Arith) {
        match op {
            Arith::Add => todo!(),
            Arith::Sub => todo!(),
            Arith::Mul => todo!(),
            Arith::Mod => todo!(),
            Arith::Pow => todo!(),
            Arith::Div => todo!(),
            Arith::IDiv => todo!(),
            Arith::BAnd => todo!(),
            Arith::Bor => todo!(),
            Arith::BXor => todo!(),
            Arith::Shl => todo!(),
            Arith::Shr => todo!(),
            Arith::Unm => todo!(),
            Arith::BNot => todo!(),
        }
    }
}
