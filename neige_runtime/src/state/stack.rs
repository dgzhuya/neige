use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use neige_infra::{
    math::random,
    value::{closure::Closure, upval::LuaUpval, value::LuaValue},
    LUA_REGISTRY_INDEX,
};

use super::node::LuaNode;

#[derive(Clone, Debug)]
pub struct LuaStack {
    pub slots: Vec<LuaValue>,                // 值信息
    pub prev: Option<Rc<RefCell<LuaStack>>>, // 上级节点信息
    pub closure: Closure,                    // 函数信息
    pub varargs: Vec<LuaValue>,              // 传入参数信息
    pub pc: isize,                           // 函数指令执行位置
    pub node: Weak<RefCell<LuaNode>>,        // state 信息
    pub openuvs: HashMap<isize, LuaUpval>,   // 捕获的上值信息
    rdm: usize,
}

impl PartialEq for LuaStack {
    fn eq(&self, other: &Self) -> bool {
        self.rdm == other.rdm
    }
}

#[allow(dead_code)]
impl LuaStack {
    pub(super) fn new(size: usize, node: &Rc<RefCell<LuaNode>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            slots: Vec::with_capacity(size),
            closure: Closure::new_fake_closure(),
            varargs: Vec::new(),
            pc: 0,
            node: Rc::downgrade(node),
            prev: None,
            openuvs: HashMap::new(),
            rdm: random(),
        }))
    }

    pub(super) fn check(&mut self, n: usize) {
        self.slots.reserve(n)
    }

    pub(super) fn push(&mut self, val: LuaValue) {
        self.slots.push(val);
    }

    pub(super) fn push_n(&mut self, mut vals: Vec<LuaValue>, n: isize) {
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

    pub(super) fn pop(&mut self) -> LuaValue {
        self.slots.pop().unwrap()
    }

    pub(super) fn pop_n(&mut self, n: usize) -> Vec<LuaValue> {
        let mut vec = Vec::with_capacity(n);
        for _ in 0..n {
            vec.push(self.pop());
        }
        vec.reverse();
        vec
    }

    pub(super) fn abs_index(&self, idx: isize) -> isize {
        if idx <= LUA_REGISTRY_INDEX || idx > 0 {
            idx
        } else {
            idx + (self.slots.len() as isize) + 1
        }
    }

    pub(super) fn is_valid(&self, idx: isize) -> bool {
        if idx < LUA_REGISTRY_INDEX {
            let uv_idx = LUA_REGISTRY_INDEX - idx - 1;
            return self.has_closure() && uv_idx < (self.closure.upvals.len() as isize);
        }
        if idx == LUA_REGISTRY_INDEX {
            return true;
        }
        let abs_index = self.abs_index(idx);
        abs_index > 0 && abs_index <= (self.slots.len() as isize)
    }

    pub(super) fn get(&self, idx: isize) -> LuaValue {
        if idx < LUA_REGISTRY_INDEX {
            let uv_idx = (LUA_REGISTRY_INDEX - idx - 1) as usize;
            return if !self.has_closure() || uv_idx >= self.closure.upvals.len() {
                LuaValue::Nil
            } else {
                self.closure.upvals[uv_idx].val.clone()
            };
        }
        if idx == LUA_REGISTRY_INDEX {
            if let Some(node) = self.node.upgrade() {
                let n = node.borrow();
                return LuaValue::Table(n.registry.clone());
            }
        }
        let abs_idx = self.abs_index(idx);
        if abs_idx > 0 && abs_idx <= (self.slots.len() as isize) {
            self.slots[(abs_idx - 1) as usize].clone()
        } else {
            LuaValue::Nil
        }
    }

    pub(super) fn set(&mut self, idx: isize, val: LuaValue) {
        if idx < LUA_REGISTRY_INDEX {
            let uv_idx = (LUA_REGISTRY_INDEX - idx - 1) as usize;
            if self.has_closure() && uv_idx < self.closure.upvals.len() {
                self.closure.upvals[uv_idx].set_val(val);
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
        let abs_idx = self.abs_index(idx);
        if abs_idx > 0 {
            self.slots[(abs_idx - 1) as usize] = val;
            return;
        }
        panic!("invalid index!!!")
    }

    pub(super) fn reverse(&mut self, mut from: isize, mut to: isize) {
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
