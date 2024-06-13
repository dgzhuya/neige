use std::rc::Rc;

use neige_infra::code::inst::Instruction;

use crate::{
    state::LuaState,
    value::{closure::Closure, upval::LuaUpval, value::LuaValue},
};

use super::{LuaApi, StackApi};

pub trait LuaVm: LuaApi {
    fn add_pc(&mut self, n: isize);
    fn fetch(&self) -> Instruction;
    fn get_const(&mut self, idx: isize);
    fn get_rk(&mut self, rk: isize);
    fn register_count(&self) -> u8;
    fn load_vararg(&mut self, n: isize);
    fn load_proto(&mut self, idx: isize);
    fn close_upvalue(&mut self, a: isize);
}

impl LuaVm for LuaState {
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
            i
        } else {
            panic!("state overflow")
        }
    }

    fn get_const(&mut self, idx: isize) {
        let proto = {
            let node = self.get_node();
            let stack = node.get_stack();
            &stack.closure.proto.clone()
        };
        if let Some(proto) = proto {
            let val = LuaValue::from_const(&proto.constants[idx as usize]);
            self.stack_push(val)
        } else {
            panic!("state overflow")
        }
    }

    fn get_rk(&mut self, rk: isize) {
        if rk > 0xff {
            self.get_const(rk & 0xff)
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
        let mut stack = node.get_stack_mut();
        let varagrs = stack.varargs.clone();
        let n = if n < 0 {
            stack.varargs.len() as isize
        } else {
            n
        };
        stack.check(n as usize);
        stack.push_n(varagrs, n);
    }

    /// ### 加载函数信息
    /// - idx 加载到的位置
    fn load_proto(&mut self, idx: isize) {
        // 获取栈信息
        let node = self.get_node();
        let mut stack = node.get_stack_mut();

        // 获取当前栈中的字节码
        if let Some(proto) = &stack.closure.proto {
            let sub_proto = proto.protos[idx as usize].clone();
            let closure = Closure::new_lua_closure(sub_proto.clone());
            for (i, uv_info) in sub_proto.upvalues.iter().enumerate() {
                let uv_idx = uv_info.idx as isize;
                if uv_info.in_stack == 1 {
                    if let Some(openuv) = stack.openuvs.get(&uv_idx) {
                        closure.upvals.borrow_mut()[i] = openuv.clone();
                    } else {
                        let val = stack.get(uv_idx + 1);
                        closure.upvals.borrow_mut()[i].set_val(val.clone());
                        stack.openuvs.insert(uv_idx, LuaUpval::new(val));
                    }
                } else {
                    let upval = stack.closure.upvals.borrow()[uv_idx as usize].clone();
                    closure.upvals.borrow_mut()[i] = upval;
                }
            }
            stack.push(LuaValue::Function(Rc::new(closure)));
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
