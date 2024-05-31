use std::panic::{catch_unwind, AssertUnwindSafe};

use neige_infra::{
    state::{CallApi, MiscApi},
    value::value::LuaValue,
};

use crate::state::LuaState;

impl MiscApi for LuaState {
    fn len(&mut self, idx: isize) {
        todo!()
    }

    fn concat(&mut self, n: usize) {
        todo!()
    }

    fn next(&mut self, idx: isize) {
        let val = self.stack_get(idx);
        if let LuaValue::Table(tbl) = val {
            let key = self.stack_pop();
        }
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
