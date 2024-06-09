use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::{api::AccessApi, state::LuaState, value::value::LuaValue};

use super::CallApi;

pub trait MiscApi {
    fn len(&mut self, idx: isize);
    fn concat(&mut self, n: usize);
    fn next(&mut self, idx: isize) -> bool;
    fn error(&mut self) -> isize;
    fn pcall(&mut self, n_args: isize, n_results: isize, msg: isize) -> isize;
}

impl MiscApi for LuaState {
    fn len(&mut self, idx: isize) {
        let val = self.stack_get(idx);
        match val {
            LuaValue::Str(s) => self.stack_push(LuaValue::Integer(s.len() as i64)),
            LuaValue::Table(tb) => self.stack_push(LuaValue::Integer(tb.len() as i64)),
            _ => {
                let result = self.call_meta_method(val.clone(), val, "__len");
                if let Some(LuaValue::Integer(i)) = result {
                    self.stack_push(LuaValue::Integer(i))
                } else {
                    panic!("length error!")
                }
            }
        }
    }

    fn concat(&mut self, n: usize) {
        if n == 0 {
            self.stack_push(LuaValue::Str("".into()))
        } else if n >= 2 {
            for _ in 1..n {
                if self.is_string(-1) && self.is_string(-2) {
                    let s2 = self.to_string(-1);
                    let s1 = self.to_string(-2);
                    self.stack_pop();
                    self.stack_pop();
                    self.stack_push(LuaValue::Str(format!("{}{}", s1, s2)));
                    continue;
                }
                let b = self.stack_pop();
                let a = self.stack_pop();
                let result = self.call_meta_method(a, b, "__concat");
                if let Some(r) = result {
                    self.stack_push(r);
                    continue;
                }
                panic!("concatenation error!")
            }
        }
    }

    fn next(&mut self, idx: isize) -> bool {
        let val = self.stack_get(idx);
        if let LuaValue::Table(tbl) = val {
            let key = self.stack_pop();
            let next_key = tbl.next_key(&key);
            if !next_key.is_nil() {
                let next_val = tbl.get(&next_key);
                self.stack_push(next_key);
                self.stack_push(next_val);
                return true;
            }
        }
        false
    }

    fn error(&mut self) -> isize {
        let val = self.stack_pop();
        panic!("{:?}", val)
    }

    fn pcall(&mut self, n_args: isize, n_results: isize, msgh: isize) -> isize {
        let node = self.get_node().clone();
        let caller = node.get_stack();
        let mut status = 2;

        let result = catch_unwind(AssertUnwindSafe(|| {
            self.call(n_args, n_results);
            status = 0;
        }));
        if let Err(err) = result {
            if msgh != 0 {
                panic!("{:?}", err);
            }
            let stack = node.get_stack();
            while *stack != *caller {
                self.pop_lua_stack()
            }
            let err_msg = if let Some(err_msg) = err.downcast_ref::<&str>() {
                err_msg.to_string()
            } else if let Some(err_msg) = err.downcast_ref::<String>() {
                err_msg.clone()
            } else {
                "Unknown error".to_string()
            };
            self.stack_push(LuaValue::Str(err_msg))
        }
        status
    }
}
