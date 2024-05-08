mod stack;

use std::rc::Rc;

use neige_infra::{
    state::LuaApi,
    value::{table::LuaTable, value::LuaValue},
    LUA_MINSTACK, LUA_REGISTRY_INDEX,
};
use stack::LuaStack;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LuaState {
    registry: LuaValue,
    stacks: Vec<LuaStack>,
}

impl LuaState {
    pub fn new() -> Self {
        Self {
            registry: LuaValue::Table(Rc::new(LuaTable::new(0, 0))),
            stacks: vec![LuaStack::new(LUA_MINSTACK)],
        }
    }

    pub fn get(&self, idx: isize) -> &LuaValue {
        if idx == LUA_REGISTRY_INDEX {
            &self.registry
        } else {
            self.stacks.last().unwrap().get(idx)
        }
    }

    pub fn get_stack(&self) -> &LuaStack {
        self.stacks.last().unwrap()
    }

    pub fn get_stack_mut(&mut self) -> &mut LuaStack {
        self.stacks.last_mut().unwrap()
    }

    pub fn push_stack(&mut self, stack: LuaStack) {
        self.stacks.push(stack)
    }

    pub fn pop_stack(&mut self) -> LuaStack {
        self.stacks.pop().unwrap()
    }
}

impl LuaApi for LuaState {}
