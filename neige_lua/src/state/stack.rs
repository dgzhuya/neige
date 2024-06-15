use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use neige_infra::{math::random, LUA_REGISTRY_INDEX};

use crate::value::{closure::Closure, upval::LuaUpval, value::LuaValue};

use super::node::LuaNode;

#[derive(Clone, Debug)]
pub struct LuaStack {
    pub slots: Vec<LuaValue>,                // 值信息
    pub prev: Option<Rc<RefCell<LuaStack>>>, // 上级节点信息
    pub closure: Rc<Closure>,                // 函数信息
    pub varargs: Vec<LuaValue>,              // 传入参数信息
    pub pc: isize,                           // 函数指令执行位置
    pub top: usize,                          // 栈顶位置
    pub node: Weak<RefCell<LuaNode>>,        // state 信息
    pub openuvs: HashMap<isize, LuaUpval>,   // 捕获的上值信息
    rdm: usize,
}

impl PartialEq for LuaStack {
    fn eq(&self, other: &Self) -> bool {
        self.rdm == other.rdm
    }
}

impl LuaStack {
    pub fn new(
        size: usize,
        node: &Rc<RefCell<LuaNode>>,
        closure: Rc<Closure>,
    ) -> Rc<RefCell<Self>> {
        let mut slots = Vec::with_capacity(size);
        for _ in 0..size {
            slots.push(LuaValue::Nil)
        }
        let stack = Self {
            slots,
            closure,
            varargs: Vec::new(),
            pc: 0,
            top: 0,
            node: Rc::downgrade(node),
            prev: None,
            openuvs: HashMap::new(),
            rdm: random(),
        };
        Rc::new(RefCell::new(stack))
    }
}

impl LuaStack {
    pub(crate) fn check(&mut self, n: usize) {
        let free = self.slots.len() - self.top;
        for _ in free..n {
            self.slots.push(LuaValue::Nil)
        }
    }

    pub(crate) fn push(&mut self, val: LuaValue) {
        if self.slots.len() == self.top {
            panic!("stack overflow!")
        }
        if !val.is_nil() {
            self.slots[self.top] = val;
        }
        self.top += 1;
    }

    pub(crate) fn push_n(&mut self, mut vals: Vec<LuaValue>, n: isize) {
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

    pub(crate) fn pop(&mut self) -> LuaValue {
        if self.top < 1 {
            panic!("stack overflow!")
        }
        self.top -= 1;
        std::mem::replace(&mut self.slots[self.top], LuaValue::Nil)
    }

    pub(crate) fn pop_n(&mut self, n: usize) -> Vec<LuaValue> {
        let mut vec = Vec::with_capacity(n);
        for _ in 0..n {
            vec.push(self.pop());
        }
        vec.reverse();
        vec
    }

    pub(crate) fn abs_index(&self, idx: isize) -> isize {
        if idx <= LUA_REGISTRY_INDEX || idx > 0 {
            idx
        } else {
            idx + (self.top as isize) + 1
        }
    }

    pub(crate) fn is_valid(&self, idx: isize) -> bool {
        if idx < LUA_REGISTRY_INDEX {
            let uv_idx = LUA_REGISTRY_INDEX - idx - 1;
            return self.has_closure() && uv_idx < (self.closure.upvals.borrow().len() as isize);
        }
        if idx == LUA_REGISTRY_INDEX {
            return true;
        }
        let abs_index = self.abs_index(idx) as usize;
        abs_index > 0 && abs_index <= self.top
    }

    pub(crate) fn get(&self, idx: isize) -> LuaValue {
        if idx < LUA_REGISTRY_INDEX {
            let uv_idx = (LUA_REGISTRY_INDEX - idx - 1) as usize;
            return if !self.has_closure() || uv_idx >= self.closure.upvals.borrow().len() {
                LuaValue::Nil
            } else {
                self.closure.upvals.borrow()[uv_idx].val.clone()
            };
        }
        if idx == LUA_REGISTRY_INDEX {
            if let Some(node) = self.node.upgrade() {
                let n = node.borrow();
                return LuaValue::Table(n.registry.clone());
            }
        }
        let abs_idx = self.abs_index(idx) as usize;
        if abs_idx > 0 && abs_idx <= self.top {
            self.slots[abs_idx - 1].clone()
        } else {
            LuaValue::Nil
        }
    }

    pub(crate) fn set(&mut self, idx: isize, val: LuaValue) {
        if idx < LUA_REGISTRY_INDEX {
            let uv_idx = (LUA_REGISTRY_INDEX - idx - 1) as usize;
            if self.has_closure() && uv_idx < self.closure.upvals.borrow().len() {
                self.closure.upvals.borrow_mut()[uv_idx].set_val(val);
                return;
            }
        }
        if idx == LUA_REGISTRY_INDEX {
            if let Some(node) = self.node.upgrade() {
                if let LuaValue::Table(tb) = val {
                    node.borrow_mut().registry = tb;
                    return;
                } else {
                    panic!("set value is not table!!!")
                }
            }
        }
        let abs_idx = self.abs_index(idx) as usize;
        if abs_idx > 0 {
            self.slots[abs_idx - 1] = val;
            return;
        }
        panic!("invalid index!!!")
    }

    pub(crate) fn reverse(&mut self, mut from: isize, mut to: isize) {
        while from < to {
            self.slots.swap(from as usize, to as usize);
            from += 1;
            to -= 1;
        }
    }

    fn has_closure(&self) -> bool {
        if let Some(_) = self.closure.rust_fn {
            true
        } else if let Some(_) = self.closure.proto {
            true
        } else {
            false
        }
    }
}
