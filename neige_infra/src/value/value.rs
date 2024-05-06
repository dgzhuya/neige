use std::{fmt::Debug, hash::Hash, rc::Rc};

use crate::{math::float_to_integer, proto::proto::Prototype, LuaType};

use super::{
    closure::{Closure, RustFn},
    table::LuaTable,
};

#[allow(dead_code)]
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

#[allow(dead_code)]
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
            LuaValue::Integer(_) => LuaType::Integer,
            LuaValue::Number(_) => LuaType::Number,
            LuaValue::Str(_) => LuaType::String,
            LuaValue::Table(_) => LuaType::Table,
            LuaValue::Function(_) => LuaType::Function,
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
