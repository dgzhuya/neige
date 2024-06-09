use neige_infra::LUA_RIDX_GLOBALS;

use crate::{
    state::LuaState,
    value::{closure::RustFn, value::LuaValue},
};

use super::SetApi;

pub trait PushApi {
    fn push_nil(&mut self);
    fn push_boolean(&mut self, b: bool);
    fn push_integer(&mut self, i: i64);
    fn push_number(&mut self, f: f64);
    fn push_string(&mut self, s: &str);
    fn push_rust_fn(&mut self, f: RustFn);
    fn register(&mut self, name: &str, f: RustFn);
    fn push_global_table(&mut self);
    fn push_rust_closure(&mut self, f: RustFn, n: usize);
}

impl PushApi for LuaState {
    fn push_nil(&mut self) {
        self.stack_push(LuaValue::Nil)
    }

    fn push_boolean(&mut self, b: bool) {
        self.stack_push(LuaValue::Boolean(b))
    }

    fn push_integer(&mut self, i: i64) {
        self.stack_push(LuaValue::Integer(i))
    }

    fn push_number(&mut self, f: f64) {
        self.stack_push(LuaValue::Number(f))
    }

    fn push_string(&mut self, s: &str) {
        self.stack_push(LuaValue::Str(s.into()))
    }

    fn push_rust_fn(&mut self, f: RustFn) {
        self.stack_push(LuaValue::new_rust_closure(f, 0))
    }

    fn register(&mut self, name: &str, f: RustFn) {
        self.push_rust_fn(f);
        self.set_global(name);
    }

    fn push_global_table(&mut self) {
        let global = self.registry_get(&LuaValue::Integer(LUA_RIDX_GLOBALS));
        self.stack_push(global)
    }

    fn push_rust_closure(&mut self, f: RustFn, mut n: usize) {
        let closure = LuaValue::new_rust_closure(f, n);
        while n > 0 {
            let val = self.stack_pop();
            if let LuaValue::Function(c) = &closure {
                c.upvals.borrow_mut()[n - 1].set_val(val);
            }
            n -= 1
        }
        self.stack_push(closure)
    }
}
