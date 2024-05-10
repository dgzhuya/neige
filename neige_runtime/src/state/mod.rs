mod node;
mod stack;

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use neige_infra::{state::LuaApi, value::value::LuaValue};

use self::{node::LuaNode, stack::LuaStack};

#[derive(Clone, Debug)]
pub struct LuaState {
    pub node: Rc<RefCell<LuaNode>>,
}

impl LuaApi for LuaState {}

/// 用于实现栈相关函数
#[allow(dead_code)]
impl LuaState {
    pub fn stack_get(&self, idx: isize) -> LuaValue {
        let node = self.get_node();
        let stack = node.get_stack();
        stack.get(idx)
    }

    pub fn stack_top(&self) -> isize {
        let node = self.get_node();
        let stack = node.get_stack();
        stack.top()
    }

    pub fn stack_check(&self, n: usize) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.check(n)
    }

    pub fn stack_push(&self, val: LuaValue) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.push(val)
    }

    pub fn stack_push_n(&self, vals: Vec<LuaValue>, n: isize) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.push_n(vals, n)
    }

    pub fn stack_pop(&self) -> LuaValue {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.pop()
    }

    pub fn stack_pop_n(&self, n: usize) -> Vec<LuaValue> {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.pop_n(n)
    }

    pub fn stack_set_top(&self, idx: isize) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.set_top(idx)
    }

    pub fn stack_abs_index(&self, idx: isize) -> isize {
        let node = self.get_node();
        let stack = node.get_stack();
        stack.abs_index(idx)
    }

    pub fn stack_is_valid(&self, idx: isize) -> bool {
        let node = self.get_node();
        let stack = node.get_stack();
        stack.is_valid(idx)
    }

    pub fn stack_set(&self, idx: isize, val: LuaValue) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.set(idx, val)
    }

    pub fn stack_reverse(&self, from: usize, to: usize) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.reverse(from, to)
    }
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
