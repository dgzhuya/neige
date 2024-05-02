use std::{hash::Hash, rc::Rc};

use crate::{math::random, proto::proto::Prototype, state::state::LuaApi};

pub type RustFn = fn(&dyn LuaApi) -> usize;

pub struct Closure {
    pub proto: Option<Rc<Prototype>>,
    pub rust_fn: Option<Rc<RustFn>>,
    rdm: usize,
}

impl Closure {
    pub fn new_fake_closure() -> Self {
        Self {
            proto: None,
            rust_fn: None,
            rdm: random(),
        }
    }

    pub fn new_lua_closure(proto: Rc<Prototype>) -> Self {
        Closure {
            proto: Some(proto),
            rust_fn: None,
            rdm: random(),
        }
    }

    pub fn new_rust_closure(f: RustFn) -> Self {
        Self {
            proto: None,
            rust_fn: Some(Rc::new(f)),
            rdm: random(),
        }
    }
}

impl Hash for Closure {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rdm.hash(state);
    }
}
