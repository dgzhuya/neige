use std::{collections::HashMap, rc::Rc};

use neige_infra::{
    value::{closure::Closure, upval::LuaUpval, value::LuaValue},
    LUA_REGISTRY_INDEX,
};

#[derive(Clone, Debug)]
pub struct LuaStack {
    slots: Vec<LuaValue>,              // 值信息
    closure: Rc<Closure>,              // 函数信息
    varargs: Vec<LuaValue>,            // 传入参数信息
    pc: usize,                         // 函数指令执行位置
    openuvs: HashMap<isize, LuaUpval>, // 捕获的上值信息
}

impl LuaStack {
    pub fn new(size: usize) -> Self {
        Self {
            slots: Vec::with_capacity(size),
            closure: Rc::new(Closure::new_fake_closure()),
            varargs: Vec::new(),
            pc: 0,
            openuvs: HashMap::new(),
        }
    }

    pub fn top(&self) -> isize {
        self.slots.len() as isize
    }

    pub fn check(&mut self, n: usize) {
        self.slots.reserve(n)
    }

    pub fn push(&mut self, val: LuaValue) {
        self.slots.push(val);
    }

    pub fn push_n(&mut self, mut vals: Vec<LuaValue>, n: isize) {
        vals.reverse();
        let n_vals = vals.len();
        let un = if n < 0 { n_vals } else { n as usize };
        for i in 0..un {
            if i < n_vals {
                self.push(vals.pop().unwrap())
            } else {
                self.push(LuaValue::Nil)
            }
        }
    }

    pub fn pop(&mut self) -> LuaValue {
        self.slots.pop().unwrap()
    }

    pub fn pop_n(&mut self, n: usize) -> Vec<LuaValue> {
        let mut vec = Vec::with_capacity(n);
        for _ in 0..n {
            vec.push(self.pop());
        }
        vec.reverse();
        vec
    }

    pub fn set_top(&mut self, idx: isize) {
        let new_top = self.abs_index(idx);
        if new_top < 0 {
            panic!("stack overflow!")
        }

        let n = self.top() - new_top;
        if n > 0 {
            for _ in 0..n {
                self.pop();
            }
        } else if n < 0 {
            for _ in n..0 {
                self.push(LuaValue::Nil);
            }
        }
    }

    pub fn abs_index(&self, idx: isize) -> isize {
        if idx <= LUA_REGISTRY_INDEX || idx > 0 {
            idx
        } else {
            idx + (self.slots.len() as isize) + 1
        }
    }

    pub fn is_valid(&self, idx: isize) -> bool {
        if idx < LUA_REGISTRY_INDEX {
            let uv_idx = LUA_REGISTRY_INDEX - idx - 1;
            let is_closure = if let Some(_) = self.closure.rust_fn {
                true
            } else if let Some(_) = self.closure.proto {
                true
            } else {
                false
            };
            return is_closure && uv_idx < (self.closure.upvals.len() as isize);
        }
        if idx == LUA_REGISTRY_INDEX {
            return true;
        }
        let abs_index = self.abs_index(idx);
        abs_index > 0 && abs_index <= self.top()
    }

    pub fn get(&self, idx: isize) -> &LuaValue {
        if idx < LUA_REGISTRY_INDEX {
            let uv_idx = (LUA_REGISTRY_INDEX - idx - 1) as usize;
            let is_closure = if let Some(_) = self.closure.rust_fn {
                true
            } else if let Some(_) = self.closure.proto {
                true
            } else {
                false
            };
            return if !is_closure || uv_idx >= self.closure.upvals.len() {
                &LuaValue::Nil
            } else {
                &self.closure.upvals[uv_idx].val
            };
        }
        let abs_idx = self.abs_index(idx);
        if abs_idx > 0 && abs_idx <= self.top() {
            &self.slots[(abs_idx - 1) as usize]
        } else {
            &LuaValue::Nil
        }
    }

    pub fn reverse(&mut self, mut from: usize, mut to: usize) {
        while from < to {
            self.slots.swap(from, to);
            from += 1;
            to -= 1;
        }
    }
}
