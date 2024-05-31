use std::rc::Rc;

use neige_infra::{
    code::inst::Instruction,
    state::{LuaVm, StackApi},
    value::{closure::Closure, upval::LuaUpval, value::LuaValue},
};

use crate::state::LuaState;

impl LuaVm for LuaState {
    fn pc(&self) -> isize {
        let node = self.get_node();
        let stack = node.get_stack();
        stack.pc
    }

    fn add_pc(&mut self, n: isize) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.pc += n;
    }

    fn fetch(&self) -> Instruction {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        if let Some(proto) = &stack.closure.proto {
            let i = proto.code[stack.pc as usize];
            stack.pc += 1;
            i.into()
        } else {
            panic!("state overflow")
        }
    }

    fn get_const(&mut self, idx: isize) {
        let node = self.get_node();
        let stack = node.get_stack_mut();
        if let Some(proto) = &stack.closure.proto {
            let i = &proto.constants[idx as usize];
            self.stack_push(i.into());
        } else {
            panic!("state overflow")
        }
    }

    fn get_rk(&mut self, rk: isize) {
        if rk > 0xff {
            self.get_const(rk)
        } else {
            self.push_value(rk + 1)
        }
    }

    fn register_count(&self) -> u8 {
        let node = self.get_node();
        let stack = node.get_stack();
        if let Some(proto) = &stack.closure.proto {
            proto.max_stack_size
        } else {
            panic!("state overflow")
        }
    }

    fn load_vararg(&mut self, n: isize) {
        let node = self.get_node();
        let stack = node.get_stack();
        let n = if n < 0 {
            stack.varargs.len() as isize
        } else {
            n
        };
        self.stack_check(n as usize);
        self.stack_push_n(stack.varargs.clone(), n);
    }

    fn load_proto(&mut self, idx: isize) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        if let Some(proto) = &stack.closure.proto {
            let sub_proto = proto.protos[idx as usize].clone();
            let closure = Closure::new_lua_closure(sub_proto.clone());
            for (i, uv_info) in sub_proto.upvalues.iter().enumerate() {
                let uv_idx = uv_info.idx as isize;
                if uv_info.in_stack == 1 {
                    if let Some(openuv) = stack.openuvs.get(&uv_idx) {
                        closure.upvals.borrow_mut()[i] = openuv.clone();
                    } else {
                        let val = LuaUpval::new(self.stack_get(uv_idx));
                        closure.upvals.borrow_mut()[i] = val.clone();
                        stack.openuvs.insert(uv_idx, val);
                    }
                } else {
                    closure.upvals.borrow_mut()[i] =
                        stack.closure.upvals.borrow()[uv_idx as usize].clone();
                }
            }
            self.stack_push(LuaValue::Function(Rc::new(closure)));
        } else {
            panic!("proto is empty!!!")
        }
    }

    fn close_upvalue(&mut self, a: isize) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        let close_idxs: Vec<isize> = stack
            .openuvs
            .iter()
            .filter(|(idx, _)| **idx >= (a - 1))
            .map(|(idx, _)| idx.clone())
            .collect();
        for i in close_idxs.iter() {
            stack.openuvs.remove(i);
        }
    }
}
