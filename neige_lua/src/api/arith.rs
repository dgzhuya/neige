use neige_infra::math::float_to_integer;

use crate::{state::LuaState, value::value::LuaValue};
use neige_infra::LuaArith as Arith;

pub trait ArithApi {
    fn arith(&mut self, op: Arith);
}

impl ArithApi for LuaState {
    fn arith(&mut self, op: Arith) {
        let b = self.stack_pop();
        let a = if op == Arith::Unm || op == Arith::BNot {
            b.clone()
        } else {
            self.stack_pop()
        };
        let c = a.clone();
        let d = b.clone();
        let result = match op {
            Arith::Add => do_arith(a, b, |x, y| x + y, |x, y| x + y),
            Arith::Sub => do_arith(a, b, |x, y| x - y, |x, y| x - y),
            Arith::Mul => do_arith(a, b, |x, y| x * y, |x, y| x * y),
            Arith::Mod => do_arith(a, b, |x, y| x % y, |x, y| x % y),
            Arith::Pow => do_arith_f(a, b, |x, y| x.powf(y)),
            Arith::Div => do_arith_f(a, b, |x, y| x / y),
            Arith::IDiv => do_arith(a, b, |x, y| x.div_euclid(y), |x, y| x.div_euclid(y)),
            Arith::BAnd => do_arith_i(a, b, |x, y| x & y),
            Arith::Bor => do_arith_i(a, b, |x, y| x | y),
            Arith::BXor => do_arith_i(a, b, |x, y| x ^ y),
            Arith::Shl => do_arith_i(a, b, |x, n| if n > 0 { x << n } else { x >> -n }),
            Arith::Shr => do_arith_i(a, b, |x, n| if n > 0 { x >> n } else { x << -n }),
            Arith::Unm => do_arith(a, b, |x, _| -x, |x, _| -x),
            Arith::BNot => do_arith_i(a, b, |x, _| !x),
        };
        if let Some(res) = result {
            self.stack_push(res);
            return;
        }
        let result = self.call_meta_method(c, d, op.get_meta_name().into());
        if let Some(res) = result {
            self.stack_push(res);
            return;
        }
        panic!("arithmetic error!")
    }
}

fn do_arith_i(a: LuaValue, b: LuaValue, arith_i: fn(i64, i64) -> i64) -> Option<LuaValue> {
    let (a, b) = match (a, b) {
        (LuaValue::Integer(i1), LuaValue::Integer(i2)) => (i1, i2),
        (LuaValue::Integer(i1), LuaValue::Number(f2)) => (i1, float_to_integer(f2).unwrap()),
        (LuaValue::Number(f1), LuaValue::Integer(i2)) => (float_to_integer(f1).unwrap(), i2),
        (LuaValue::Number(f1), LuaValue::Number(f2)) => {
            (float_to_integer(f1).unwrap(), float_to_integer(f2).unwrap())
        }
        (_, _) => return None,
    };
    Some(LuaValue::Integer(arith_i(a, b)))
}

fn do_arith_f(a: LuaValue, b: LuaValue, arith_f: fn(f64, f64) -> f64) -> Option<LuaValue> {
    let (a, b) = match (a, b) {
        (LuaValue::Integer(i1), LuaValue::Integer(i2)) => (i1 as f64, i2 as f64),
        (LuaValue::Integer(i1), LuaValue::Number(f2)) => (i1 as f64, f2),
        (LuaValue::Number(f1), LuaValue::Integer(i2)) => (f1, i2 as f64),
        (LuaValue::Number(f1), LuaValue::Number(f2)) => (f1, f2),
        (_, _) => return None,
    };
    Some(LuaValue::Number(arith_f(a, b)))
}

fn do_arith(
    a: LuaValue,
    b: LuaValue,
    arith_i: fn(i64, i64) -> i64,
    arith_f: fn(f64, f64) -> f64,
) -> Option<LuaValue> {
    match (a, b) {
        (LuaValue::Integer(i1), LuaValue::Integer(i2)) => Some(LuaValue::Integer(arith_i(i1, i2))),
        (LuaValue::Integer(i1), LuaValue::Number(f2)) => {
            Some(LuaValue::Number(arith_f(i1 as f64, f2)))
        }
        (LuaValue::Number(f1), LuaValue::Integer(i2)) => {
            Some(LuaValue::Number(arith_f(f1, i2 as f64)))
        }
        (LuaValue::Number(f1), LuaValue::Number(f2)) => Some(LuaValue::Number(arith_f(f1, f2))),
        (_, _) => None,
    }
}
