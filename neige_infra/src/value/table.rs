use std::{cell::RefCell, collections::HashMap, hash::Hash};

use crate::math::{float_to_integer, random};

use super::value::LuaValue;

#[derive(Clone, Debug)]
pub struct LuaTable {
    pub arr: RefCell<Vec<LuaValue>>,
    pub map: RefCell<HashMap<LuaValue, LuaValue>>,
    rdm: usize,
}

impl Hash for LuaTable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rdm.hash(state);
    }
}

#[allow(dead_code)]
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

    pub fn put(&self, key: LuaValue, val: LuaValue) {
        if key.is_nil() {
            panic!("table index is not nil")
        }
        if let LuaValue::Number(n) = key {
            if n.is_nan() {
                panic!("table index is not NaN")
            }
        }

        let mut arr = self.arr.borrow_mut();
        let mut map = self.map.borrow_mut();
        if let Some(idx) = to_index(&key) {
            let arr_n = arr.len();
            if idx <= arr_n {
                let val_is_nil = val.is_nil();
                arr[idx - 1] = val;
                if idx == arr_n && val_is_nil {
                    shrink_array(&mut arr);
                }
                return;
            }
            if idx == arr_n + 1 {
                map.remove(&key);
                if !val.is_nil() {
                    arr.push(val);
                    expand_arr(&mut arr, &mut map)
                }
                return;
            }
        }

        if !val.is_nil() {
            map.insert(key, val);
        } else {
            map.remove(&key);
        }
    }
}

fn expand_arr(arr: &mut Vec<LuaValue>, map: &mut HashMap<LuaValue, LuaValue>) {
    let mut idx = arr.len() + 1;
    loop {
        let key = LuaValue::Integer(idx as i64);
        if map.contains_key(&key) {
            let val = map.remove(&key).unwrap();
            arr.push(val);
            idx += 1;
        } else {
            break;
        }
    }
}

fn shrink_array(arr: &mut Vec<LuaValue>) {
    while !arr.is_empty() {
        if arr.last().unwrap().is_nil() {
            arr.pop();
        } else {
            break;
        }
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
