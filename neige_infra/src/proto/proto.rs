use std::rc::Rc;

use crate::value::value::LuaValue;

#[derive(Debug)]
pub struct Prototype {
    pub source: Option<String>,
    pub line_defined: u32,
    pub last_line_defined: u32,
    pub num_params: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
    pub code: Vec<u32>,
    pub constants: Vec<Constant>,
    pub upvalues: Vec<Upvalue>,
    pub protos: Vec<Rc<Prototype>>,
    pub line_info: Vec<u32>,
    pub loc_vars: Vec<LocVar>,
    pub upvalue_names: Vec<String>,
}

#[derive(Debug)]
pub struct LocVar {
    pub var_name: String,
    pub start_pc: u32,
    pub end_pc: u32,
}

#[derive(Debug)]
pub struct Upvalue {
    pub in_stack: u8,
    pub idx: u8,
}

#[derive(Debug)]
pub enum Constant {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    Str(String),
}

impl Constant {
    pub fn to_string(&self) -> String {
        match self {
            Constant::Nil => format!("nil"),
            Constant::Boolean(b) => format!("{}", b),
            Constant::Number(f) => format!("{}", f),
            Constant::Integer(i) => format!("{}", i),
            Constant::Str(s) => format!("{:?}", s),
        }
    }

    pub fn to_value(&self) -> LuaValue {
        match self {
            Constant::Nil => LuaValue::Nil,
            Constant::Boolean(b) => LuaValue::Boolean(*b),
            Constant::Number(n) => LuaValue::Number(*n),
            Constant::Integer(i) => LuaValue::Integer(*i),
            Constant::Str(s) => LuaValue::Str(s.clone()),
        }
    }
}
