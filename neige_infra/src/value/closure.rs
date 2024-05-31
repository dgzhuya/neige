use std::{cell::RefCell, hash::Hash, rc::Rc};

use crate::{math::random, proto::proto::Prototype, state::LuaApi};

use super::upval::LuaUpval;

pub type RustFn = fn(&mut dyn LuaApi) -> usize;

#[derive(Debug, Clone)]
pub struct Closure {
    pub proto: Option<Rc<Prototype>>,
    pub rust_fn: Option<RustFn>,
    pub upvals: RefCell<Vec<LuaUpval>>,
    rdm: usize,
}

#[allow(dead_code)]
impl Closure {
    pub fn new_fake_closure() -> Self {
        Self {
            proto: None,
            rust_fn: None,
            rdm: random(),
            upvals: RefCell::new(Vec::new()),
        }
    }

    pub fn new_lua_closure(proto: Rc<Prototype>) -> Self {
        let upvals = RefCell::new(if proto.upvalues.len() > 0 {
            Vec::with_capacity(proto.upvalues.len())
        } else {
            Vec::new()
        });
        Closure {
            proto: Some(proto),
            rust_fn: None,
            rdm: random(),
            upvals,
        }
    }

    pub fn new_rust_closure(f: RustFn, n_upvals: usize) -> Self {
        Self {
            proto: None,
            rust_fn: Some(f),
            upvals: RefCell::new(Vec::with_capacity(n_upvals)),
            rdm: random(),
        }
    }
}

impl Hash for Closure {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rdm.hash(state);
    }
}
