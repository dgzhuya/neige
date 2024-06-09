use std::rc::Rc;

use neige_infra::LuaCompare;

use crate::{state::LuaState, value::value::LuaValue};

pub trait CompareApi {
    fn compare(&mut self, idx1: isize, idx2: isize, op: LuaCompare) -> bool;
    fn raw_equal(&mut self, idx1: isize, idx2: isize) -> bool;
}

impl CompareApi for LuaState {
    fn compare(&mut self, idx1: isize, idx2: isize, op: LuaCompare) -> bool {
        let a = self.stack_get(idx1);
        let b = self.stack_get(idx2);
        match op {
            LuaCompare::Eq => self.eq(a, b, false),
            LuaCompare::Lt => self.lt(a, b),
            LuaCompare::Le => self.le(a, b),
        }
    }

    fn raw_equal(&mut self, idx1: isize, idx2: isize) -> bool {
        if !self.stack_is_valid(idx1) || self.stack_is_valid(idx2) {
            false
        } else {
            let a = self.stack_get(idx1);
            let b = self.stack_get(idx2);
            self.eq(a, b, true)
        }
    }
}

impl LuaState {
    fn eq(&mut self, a: LuaValue, b: LuaValue, raw: bool) -> bool {
        match (a, b) {
            (LuaValue::Nil, LuaValue::Nil) => return true,
            (LuaValue::Boolean(x), LuaValue::Boolean(y)) => x == y,
            (LuaValue::Integer(x), LuaValue::Integer(y)) => x == y,
            (LuaValue::Integer(x), LuaValue::Number(y)) => x as f64 == y,
            (LuaValue::Number(x), LuaValue::Integer(y)) => x == y as f64,
            (LuaValue::Number(x), LuaValue::Number(y)) => x == y,
            (LuaValue::Str(x), LuaValue::Str(y)) => x == y,
            (a, b) => {
                if !raw {
                    let mt = self.call_meta_method(a.clone(), b.clone(), "__eq");
                    if let Some(mt) = mt {
                        return mt.convert_to_boolean();
                    }
                }
                if let LuaValue::Table(x) = &a {
                    if let LuaValue::Table(y) = &b {
                        return Rc::ptr_eq(x, y);
                    }
                }
                panic!("comparsion error!")
            }
        }
    }

    fn lt(&mut self, a: LuaValue, b: LuaValue) -> bool {
        match (a, b) {
            (LuaValue::Integer(x), LuaValue::Integer(y)) => x < y,
            (LuaValue::Integer(x), LuaValue::Number(y)) => (x as f64) < y,
            (LuaValue::Number(x), LuaValue::Integer(y)) => x < (y as f64),
            (LuaValue::Number(x), LuaValue::Number(y)) => x < y,
            (LuaValue::Str(x), LuaValue::Str(y)) => x < y,
            (a, b) => {
                let mt = self.call_meta_method(a, b, "__lt");
                if let Some(mt) = mt {
                    return mt.convert_to_boolean();
                }

                panic!("comparsion error!")
            }
        }
    }

    fn le(&mut self, a: LuaValue, b: LuaValue) -> bool {
        match (a, b) {
            (LuaValue::Integer(x), LuaValue::Integer(y)) => x <= y,
            (LuaValue::Integer(x), LuaValue::Number(y)) => (x as f64) <= y,
            (LuaValue::Number(x), LuaValue::Integer(y)) => x <= (y as f64),
            (LuaValue::Number(x), LuaValue::Number(y)) => x <= y,
            (LuaValue::Str(x), LuaValue::Str(y)) => x <= y,
            (a, b) => {
                let mt = self.call_meta_method(a.clone(), b.clone(), "__le");
                if let Some(mt) = mt {
                    return mt.convert_to_boolean();
                }
                let mt = self.call_meta_method(b, a, "__lt");
                if let Some(mt) = mt {
                    return mt.convert_to_boolean();
                }
                panic!("comparsion error!");
            }
        }
    }
}
