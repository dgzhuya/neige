use std::{fs::File, io::BufReader, rc::Rc};

use neige_infra::{
    state::{CallApi, LuaVm, StackApi},
    value::{closure::Closure, upval::LuaUpval, value::LuaValue},
    LUA_RIDX_GLOBALS,
};
use neige_undump::undump;

use crate::state::{LuaStack, LuaState};

impl CallApi for LuaState {
    fn load(&mut self, chunk: BufReader<File>, chunk_name: &str, mode: &str) {
        println!("{}", mode);
        let proto = undump(chunk, chunk_name);
        let proto_len = proto.upvalues.len();
        let env = LuaUpval::new(self.registry_get(&LuaValue::Integer(LUA_RIDX_GLOBALS)));
        let c = LuaValue::new_lua_closure(proto);
        if proto_len > 0 {
            if let LuaValue::Function(c) = &c {
                c.upvals.borrow_mut()[0] = env
            }
        }
        self.stack_push(c);
    }

    fn call(&mut self, mut n_args: isize, n_results: isize) {
        let val = self.stack_get(-(n_args + 1));
        let c = if let LuaValue::Function(c) = val {
            Some(c)
        } else {
            let val = self.get_meta_field(&val, "__call");
            if let LuaValue::Function(c) = val.clone() {
                self.stack_push(val);
                self.insert(-(n_args + 2));
                n_args += 1;
                Some(c)
            } else {
                None
            }
        };
        if let Some(c) = c {
            if let Some(_) = &c.proto {
                self.call_lua_closure(n_args, n_results, c)
            } else {
                self.call_rust_closure(n_args, n_results, c)
            }
        } else {
            panic!("not function")
        }
    }
}

impl LuaState {
    fn call_lua_closure(&mut self, n_args: isize, n_results: isize, c: Rc<Closure>) {
        let proto = c.proto.clone().unwrap();
        let n_regs = proto.max_stack_size as usize;
        let n_params = proto.num_params as isize;
        let is_var_arg = proto.is_vararg == 1;

        let new_stack = LuaStack::new(n_regs + 20, &self.node, c);
        let mut new_stack_mut = new_stack.borrow_mut();

        let mut func_and_arg = self.stack_pop_n((n_args as usize) + 1);

        new_stack_mut.push_n(func_and_arg.split_off(1), n_params);
        if n_args > n_params && is_var_arg {
            new_stack_mut.varargs = func_and_arg.split_off((n_params as usize) + 1)
        }
        self.push_lua_stack(new_stack.clone());
        self.run_lua_closure();
        self.pop_lua_stack();
        if n_results != 0 {
            let top = new_stack_mut.slots.len();
            let results = new_stack_mut.pop_n(top - n_regs);
            self.check_stack(results.len());
            self.stack_push_n(results, n_results)
        }
    }

    fn run_lua_closure(&mut self) {
        loop {
            let code = self.fetch();
        }
    }

    fn call_rust_closure(&mut self, n_args: isize, n_results: isize, c: Rc<Closure>) {
        let rust_fn = c.rust_fn.unwrap();

        let new_stack = LuaStack::new(n_args as usize + 20, &self.node, c);
        let mut new_stack_mut = new_stack.borrow_mut();

        let args = self.stack_pop_n(n_args as usize);
        new_stack_mut.push_n(args, n_args);
        self.stack_pop();
        self.push_lua_stack(new_stack.clone());
        let r = rust_fn(self);
        self.pop_lua_stack();
        if n_results != 0 {
            let results = new_stack_mut.pop_n(r);
            self.stack_check(results.len());
            self.stack_push_n(results, n_results)
        }
    }
}
