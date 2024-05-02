use std::{cell::RefCell, collections::HashMap, hash::Hash};

use crate::math::{float_to_integer, random};

use super::value::{self, LuaValue};

#[derive(Clone)]
pub struct LuaTable {
    arr: RefCell<Vec<LuaValue>>,
    map: RefCell<HashMap<LuaValue, LuaValue>>,
    rdm: usize,
}

impl Hash for LuaTable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rdm.hash(state);
    }
}

impl LuaTable {
    pub fn new(n_arr: usize, n_rec: usize) -> Self {
        Self {
            arr: RefCell::new(Vec::with_capacity(n_arr)),
            map: RefCell::new(HashMap::with_capacity(n_rec)),
            rdm: random(),
        }
    }

    pub fn len(&self) -> usize {
        self.arr.borrow().len()
    }

    /// 获取table的值
    pub fn get(&self, key: &LuaValue) -> LuaValue {
        let arr = self.arr.borrow();
        if let Some(idx) = to_index(key) {
            if idx <= arr.len() {
                return arr[idx - 1].clone();
            }
        }
        let map = self.map.borrow();
        if let Some(val) = map.get(key) {
            val.clone()
        } else {
            LuaValue::Nil
        }
    }

    pub fn put(&self, key: LuaValue, value: LuaValue) {
        if key.is_nil() {}
    }
}

fn to_index(key: &LuaValue) -> Option<usize> {
    if let LuaValue::Integer(i) = key {
        if *i >= 1 {
            return Some(*i as usize);
        }
    }
    if let LuaValue::Number(f) = key {
        if let Some(i) = float_to_integer(*f) {
            if i >= 1 {
                return Some(i as usize);
            }
        }
    }
    None
}
