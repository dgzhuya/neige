#[cfg(feature = "wasm")]
use std::rc::Rc;
#[cfg(not(feature = "wasm"))]
use std::{fs::File, io::BufReader, rc::Rc};

use neige_infra::{
    code::inst::Instruction,
    state::{CallApi, LuaVm, StackApi},
    value::{closure::Closure, value::LuaValue},
    LUA_RIDX_GLOBALS,
};
use neige_undump::undump;

use crate::state::{LuaStack, LuaState};

impl CallApi for LuaState {
    #[cfg(not(feature = "wasm"))]
    fn load(&mut self, chunk: BufReader<File>, chunk_name: &str, _mode: &str) {
        let proto = undump(chunk, chunk_name);
        let proto_len = proto.upvalues.len();
        let env = self.registry_get(&LuaValue::Integer(LUA_RIDX_GLOBALS));
        let c = LuaValue::new_lua_closure(proto);
        if proto_len > 0 {
            if let LuaValue::Function(c) = &c {
                c.upvals.borrow_mut()[0].set_val(env)
            }
        }
        self.stack_push(c);
    }

    #[cfg(feature = "wasm")]
    fn load(&mut self, chunk: Vec<u8>, chunk_name: &str, _mode: &str) {
        let proto = undump(chunk, chunk_name);
        let proto_len = proto.upvalues.len();
        let env = self.registry_get(&LuaValue::Integer(LUA_RIDX_GLOBALS));
        let c = LuaValue::new_lua_closure(proto);
        if proto_len > 0 {
            if let LuaValue::Function(c) = &c {
                c.upvals.borrow_mut()[0].set_val(env)
            }
        }
        self.stack_push(c);
    }

    fn call(&mut self, mut n_args: isize, n_results: isize) {
        let val = self.stack_get(-(n_args + 1));
        let c = if let LuaValue::Function(c) = val {
            Some(c)
        } else {
            let val = self.inline_get_meta_field(&val, "__call");
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
        {
            let mut new_stack_mut = new_stack.borrow_mut();
            let mut func_and_arg = self.stack_pop_n((n_args as usize) + 1);
            let mut param_or_arg = func_and_arg.split_off(1);
            param_or_arg.reverse();
            let len = param_or_arg.len();
            let un = if n_params < 0 { len } else { n_params as usize };
            for i in 0..un {
                if i < len {
                    new_stack_mut.push(param_or_arg.pop().unwrap());
                } else {
                    new_stack_mut.push(LuaValue::Nil)
                }
            }
            new_stack_mut.top = n_regs;
            if n_args > n_params && is_var_arg {
                param_or_arg.reverse();
                new_stack_mut.varargs = param_or_arg
            }
        }
        self.push_lua_stack(new_stack.clone());
        self.run_lua_closure();
        self.pop_lua_stack();
        if n_results != 0 {
            let mut new_stack_mut = new_stack.borrow_mut();
            let top = new_stack_mut.top;
            let results = new_stack_mut.pop_n(top - n_regs);
            self.check_stack(results.len());
            self.stack_push_n(results, n_results)
        }
    }

    fn run_lua_closure(&mut self) {
        loop {
            let inst = self.fetch();
            self.execute(&inst);
            if let Instruction::Return(_, _, _) = inst {
                break;
            }
        }
    }

    fn call_rust_closure(&mut self, n_args: isize, n_results: isize, c: Rc<Closure>) {
        let rust_fn = c.rust_fn.unwrap();

        let new_stack = LuaStack::new(n_args as usize + 20, &self.node, c);
        {
            let mut new_stack_mut = new_stack.borrow_mut();
            let args = self.stack_pop_n(n_args as usize);
            new_stack_mut.push_n(args, n_args);
        }
        self.stack_pop();
        self.push_lua_stack(new_stack.clone());
        let r = rust_fn(self);
        self.pop_lua_stack();
        if n_results != 0 {
            let mut new_stack_mut = new_stack.borrow_mut();
            let results = new_stack_mut.pop_n(r);
            self.stack_check(results.len());
            self.stack_push_n(results, n_results)
        }
    }
}
