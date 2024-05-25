use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use neige_infra::{
    value::{closure::Closure, table::LuaTable},
    LUA_MINSTACK,
};

use super::stack::LuaStack;

#[derive(Debug, Clone)]
pub struct LuaNode {
    pub registry: Rc<LuaTable>,
    pub stack: Option<Rc<RefCell<LuaStack>>>,
}

impl LuaNode {
    pub(super) fn new() -> Rc<RefCell<Self>> {
        let table = LuaTable::new(0, 0);
        let node = Rc::new(RefCell::new(Self {
            stack: None,
            registry: Rc::new(table),
        }));
        let stack = LuaStack::new(LUA_MINSTACK, &node, Rc::new(Closure::new_fake_closure()));
        node.borrow_mut().stack = Some(stack);
        node
    }

    pub(crate) fn get_stack(&self) -> Ref<LuaStack> {
        if let Some(stack) = &self.stack {
            stack.borrow()
        } else {
            panic!("state has not stack")
        }
    }

    pub(crate) fn get_stack_mut(&self) -> RefMut<LuaStack> {
        if let Some(stack) = &self.stack {
            stack.borrow_mut()
        } else {
            panic!("state has not stack")
        }
    }
}
