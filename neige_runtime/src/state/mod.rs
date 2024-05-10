mod meta;
mod node;
mod stack;
mod state;

use neige_infra::{state::LuaApi, value::value::LuaValue};

pub use state::LuaState;

impl LuaApi for LuaState {}

/// 用于实现栈相关函数
#[allow(dead_code)]
impl LuaState {
    pub(super) fn stack_get(&self, idx: isize) -> LuaValue {
        let node = self.get_node();
        let stack = node.get_stack();
        stack.get(idx)
    }

    pub(super) fn stack_check(&self, n: usize) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.check(n)
    }

    pub(super) fn stack_push(&self, val: LuaValue) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.push(val)
    }

    pub(super) fn stack_push_n(&self, vals: Vec<LuaValue>, n: isize) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.push_n(vals, n)
    }

    pub(super) fn stack_pop(&self) -> LuaValue {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.pop()
    }

    pub(super) fn stack_pop_n(&self, n: usize) -> Vec<LuaValue> {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.pop_n(n)
    }

    pub(super) fn stack_abs_index(&self, idx: isize) -> isize {
        let node = self.get_node();
        let stack = node.get_stack();
        stack.abs_index(idx)
    }

    pub(super) fn stack_is_valid(&self, idx: isize) -> bool {
        let node = self.get_node();
        let stack = node.get_stack();
        stack.is_valid(idx)
    }

    pub(super) fn stack_set(&self, idx: isize, val: LuaValue) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.set(idx, val)
    }

    pub(super) fn stack_reverse(&self, from: isize, to: isize) {
        let node = self.get_node();
        let mut stack = node.get_stack_mut();
        stack.reverse(from, to)
    }
}
