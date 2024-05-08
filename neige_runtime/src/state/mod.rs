mod stack;

use std::rc::Rc;

use neige_infra::{
    state::LuaApi,
    value::{table::LuaTable, value::LuaValue},
    LUA_MINSTACK, LUA_REGISTRY_INDEX, LUA_RIDX_GLOBALS,
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
        let registry = LuaValue::new_table(0, 0);
        if let LuaValue::Table(tb) = &registry {
            tb.put(
                LuaValue::Integer(LUA_RIDX_GLOBALS as i64),
                LuaValue::new_table(0, 0),
            )
        }
        Self {
            registry,
            stacks: vec![LuaStack::new(LUA_MINSTACK)],
        }
    }

    pub fn set(&mut self, idx: isize, val: LuaValue) {
        if idx == LUA_REGISTRY_INDEX {
            if let LuaValue::Table(_) = val {
                self.registry = val
            }
        } else {
            self.get_stack_mut().set(idx, val)
        }
    }

    pub fn get(&self, idx: isize) -> &LuaValue {
        if idx == LUA_REGISTRY_INDEX {
            &self.registry
        } else {
            self.get_stack().get(idx)
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
