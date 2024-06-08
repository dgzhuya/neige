use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use neige_infra::value::value::LuaValue;

use super::{node::LuaNode, stack::LuaStack};

#[derive(Clone, Debug)]
pub struct LuaState {
    pub node: Rc<RefCell<LuaNode>>,
}

impl LuaState {
    pub fn new() -> Self {
        let node = LuaNode::new();
        Self { node }
    }

    pub(crate) fn pop_lua_stack(&self) {
        let mut node = self.get_node_mut();
        if let Some(stack) = &node.stack.clone() {
            node.stack = stack.borrow().prev.clone();
            stack.borrow_mut().prev = None;
        }
    }

    pub(crate) fn push_lua_stack(&self, lua_stack: Rc<RefCell<LuaStack>>) {
        let mut node = self.get_node_mut();
        if let Some(prev_stack) = &node.stack {
            lua_stack.borrow_mut().prev = Some(prev_stack.clone())
        }
        node.stack = Some(lua_stack);
    }

    pub(crate) fn get_node(&self) -> Ref<LuaNode> {
        self.node.borrow()
    }

    /// 获取node信息
    /// ### 返回值
    /// * `RefMut<LuaNode>` lua node的引用
    pub(crate) fn get_node_mut(&self) -> RefMut<LuaNode> {
        self.node.borrow_mut()
    }

    pub(crate) fn registry_get(&self, key: &LuaValue) -> LuaValue {
        let node = self.get_node();
        node.registry.get(key)
    }

    pub(crate) fn registry_set(&self, key: LuaValue, val: LuaValue) {
        let node = self.get_node();
        node.registry.put(key, val)
    }
}
