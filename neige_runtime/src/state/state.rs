use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use super::{node::LuaNode, stack::LuaStack};

#[derive(Clone, Debug)]
pub struct LuaState {
    pub node: Rc<RefCell<LuaNode>>,
}

#[allow(dead_code)]
impl LuaState {
    pub fn new() -> Self {
        let node = LuaNode::new();
        Self { node }
    }

    pub fn pop_lua_stack(&self) {
        let mut node = self.get_node_mut();
        if let Some(stack) = node.stack.clone() {
            node.stack = stack.borrow().prev.clone();
            stack.borrow_mut().prev = None;
        }
    }

    pub fn push_lua_stack(&self, lua_stack: Rc<RefCell<LuaStack>>) {
        let mut node = self.get_node_mut();
        lua_stack.borrow_mut().prev = node.stack.clone();
        node.stack = Some(lua_stack);
    }

    pub fn get_node(&self) -> Ref<LuaNode> {
        self.node.borrow()
    }

    /// 获取node信息
    /// ### 返回值
    /// * `RefMut<LuaNode>` lua node的引用
    pub fn get_node_mut(&self) -> RefMut<LuaNode> {
        self.node.borrow_mut()
    }
}
