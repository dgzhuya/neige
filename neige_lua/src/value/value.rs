use std::{fmt::Debug, hash::Hash, rc::Rc};

use neige_infra::{math::float_to_integer, Constant, LuaType, Prototype};
use serde::{ser::SerializeMap, Serialize};

use super::{
    closure::{Closure, RustFn},
    table::LuaTable,
};

#[derive(Clone)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    Str(String),
    Table(Rc<LuaTable>),
    Function(Rc<Closure>),
}

impl LuaValue {
    pub fn is_nil(&self) -> bool {
        match self {
            Self::Nil => true,
            _ => false,
        }
    }

    pub fn new_table(n_arr: usize, n_rec: usize) -> LuaValue {
        LuaValue::Table(Rc::new(LuaTable::new(n_arr, n_rec)))
    }

    pub fn new_lua_closure(proto: Rc<Prototype>) -> LuaValue {
        LuaValue::Function(Rc::new(Closure::new_lua_closure(proto)))
    }

    pub fn new_rust_closure(f: RustFn, n_upvals: usize) -> LuaValue {
        LuaValue::Function(Rc::new(Closure::new_rust_closure(f, n_upvals)))
    }

    pub fn type_of(&self) -> LuaType {
        match self {
            LuaValue::Nil => LuaType::Nil,
            LuaValue::Boolean(_) => LuaType::Boolean,
            LuaValue::Integer(_) => LuaType::Number,
            LuaValue::Number(_) => LuaType::Number,
            LuaValue::Str(_) => LuaType::String,
            LuaValue::Table(_) => LuaType::Table,
            LuaValue::Function(_) => LuaType::Function,
        }
    }
}

impl LuaValue {
    pub fn from_const(val: &Constant) -> Self {
        match val {
            Constant::Nil => LuaValue::Nil,
            Constant::Boolean(b) => LuaValue::Boolean(*b),
            Constant::Number(n) => LuaValue::Number(*n),
            Constant::Integer(i) => LuaValue::Integer(*i),
            Constant::Str(s) => LuaValue::Str(s.clone()),
        }
    }

    pub fn convert_to_boolean(&self) -> bool {
        match self {
            LuaValue::Nil => false,
            LuaValue::Boolean(b) => *b,
            _ => true,
        }
    }

    pub fn convert_to_integer(&self) -> Option<i64> {
        match self {
            LuaValue::Integer(i) => Some(*i),
            LuaValue::Number(n) => float_to_integer(*n),
            LuaValue::Str(s) => {
                if let Ok(i) = s.parse() {
                    Some(i)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn convert_to_float(&self) -> Option<f64> {
        match self {
            LuaValue::Integer(i) => Some(*i as f64),
            LuaValue::Number(f) => Some(*f),
            LuaValue::Str(s) => {
                if let Ok(f) = s.parse() {
                    Some(f)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Debug for LuaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "(nil)"),
            Self::Boolean(b) => write!(f, "({})", b),
            Self::Integer(i) => write!(f, "({})", i),
            Self::Number(n) => write!(f, "({})", n),
            Self::Str(s) => write!(f, "({})", s),
            Self::Table(_) => write!(f, "(table)"),
            Self::Function(_) => write!(f, "(function)"),
        }
    }
}

impl PartialEq for LuaValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (&Self::Boolean(x), &Self::Boolean(y)) => x == y,
            (&Self::Integer(x), &Self::Integer(y)) => x == y,
            (&Self::Integer(i), &Self::Number(f)) | (&Self::Number(f), &Self::Integer(i)) => {
                i as f64 == f && f as i64 == i
            }
            (&Self::Number(x), &Self::Number(y)) => x == y,
            (Self::Str(x), Self::Str(y)) => x == y,
            (Self::Table(x), Self::Table(y)) => Rc::ptr_eq(x, y),
            (Self::Function(x), Self::Function(y)) => Rc::ptr_eq(x, y),
            _ => false,
        }
    }
}
impl Eq for LuaValue {}

impl Hash for LuaValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LuaValue::Nil => (),
            LuaValue::Boolean(b) => b.hash(state),
            LuaValue::Integer(i) => i.hash(state),
            &LuaValue::Number(f) => {
                if let Some(i) = float_to_integer(f) {
                    i.hash(state)
                } else {
                    (f.to_bits() as i64).hash(state)
                }
            }
            LuaValue::Str(s) => s.hash(state),
            LuaValue::Table(t) => t.as_ref().hash(state),
            LuaValue::Function(f) => f.as_ref().hash(state),
        }
    }
}

impl Serialize for LuaValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            LuaValue::Nil => serializer.serialize_unit(),
            LuaValue::Boolean(b) => serializer.serialize_bool(*b),
            LuaValue::Integer(i) => serializer.serialize_i64(*i),
            LuaValue::Number(f) => serializer.serialize_f64(*f),
            LuaValue::Str(s) => serializer.serialize_str(s),
            LuaValue::Function(_) => panic!("function can not serialize"),
            LuaValue::Table(table) => {
                let map = table.map.borrow();
                if map.len() == 0 {
                    table.arr.borrow().serialize(serializer)
                } else {
                    let arr = table.arr.borrow();
                    let mut obj = serializer.serialize_map(Some(map.len() + arr.len()))?;
                    for (i, val) in arr.iter().enumerate() {
                        obj.serialize_entry(&(i + 1).to_string(), val)?;
                    }
                    for (k, val) in map.iter() {
                        obj.serialize_entry(k, val)?;
                    }
                    obj.end()
                }
            }
        }
    }
}
