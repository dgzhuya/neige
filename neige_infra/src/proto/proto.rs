use std::rc::Rc;

use crate::code::inst::Instruction;

/// lua原型数据
#[derive(Debug)]
pub struct Prototype {
    pub source: Option<String>,     // 源文件名
    pub line_defined: u32,          // 定义行号
    pub last_line_defined: u32,     // 最后定义行号
    pub num_params: u8,             // 参数个数
    pub is_vararg: u8,              // 是否可变参数
    pub max_stack_size: u8,         // 最大栈大小
    pub code: Vec<Instruction>,     // 指令
    pub constants: Vec<Constant>,   // 常量
    pub upvalues: Vec<Upvalue>,     // upvalue
    pub protos: Vec<Rc<Prototype>>, // 子原型
    pub line_info: Vec<u32>,        // 行号信息
    pub loc_vars: Vec<LocVar>,      // 局部变量
    pub upvalue_names: Vec<String>, // upvalue名称
}

/// 局部变量
#[derive(Debug)]
pub struct LocVar {
    pub var_name: String, // 变量名
    pub start_pc: u32,    // 开始pc
    pub end_pc: u32,      // 结束pc
}

/// upvalue
#[derive(Debug)]
pub struct Upvalue {
    pub in_stack: u8, // 是否在栈中
    pub idx: u8,      // 索引
}

/// 函数原型
#[derive(Debug)]
pub enum Constant {
    Nil,           // nil
    Boolean(bool), // bool
    Number(f64),   // number
    Integer(i64),  // integer
    Str(String),   // string
}

impl Constant {
    /// 获取常量类型
    pub fn to_string(&self) -> String {
        match self {
            Constant::Nil => format!("nil"),
            Constant::Boolean(b) => format!("{}", b),
            Constant::Number(f) => format!("{}", f),
            Constant::Integer(i) => format!("{}", i),
            Constant::Str(s) => format!("{:?}", s),
        }
    }
}
