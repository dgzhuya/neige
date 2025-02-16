use super::Block;

#[derive(Debug, PartialEq)]
pub struct StringExp {
    line: i32,
    str: String,
}

#[derive(Debug, PartialEq)]
pub struct NameExp {
    line: i32,
    name: String,
}

#[derive(Debug, PartialEq)]
pub struct UnopExp {
    line: i32,
    op: i32,
    exp: Box<Exp>,
}

#[derive(Debug, PartialEq)]
pub struct BinopExp {
    line: i32,
    op: i32,
    exp1: Box<Exp>,
    exp2: Box<Exp>,
}

#[derive(Debug, PartialEq)]
pub struct ConcatExp {
    line: i32,
    exps: Vec<Box<Exp>>,
}

#[derive(Debug, PartialEq)]
pub struct TableConstructorExp {
    line: i32,
    last_line: i32,
    key_exps: Vec<Box<Exp>>,
    val_exps: Vec<Box<Exp>>,
}

#[derive(Debug, PartialEq)]
pub struct FuncCallExp {
    line: i32,
    last_line: i32,
    prefix_exp: Box<Exp>,
    name_exp: Option<Box<StringExp>>,
    args: Vec<Box<Exp>>,
}

#[derive(Debug, PartialEq)]
pub struct FuncDefExp {
    line: i32,
    last_line: i32,
    par_list: Vec<String>,
    is_vararg: bool,
    block: Box<Block>,
}

#[derive(Debug, PartialEq)]
pub struct TableAccessExp {
    last_line: i32,
    prefix_exp: Box<Exp>,
    key_exp: Box<Exp>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Exp {
    Nil(i32),
    True(i32),
    False(i32),
    Vararg(i32),
    Integer(i32, i64),
    Float(i32, f64),
    String(StringExp),
    Name(NameExp),
    Unop(UnopExp),
    Binop(BinopExp),
    Concat(ConcatExp),
    TableConstructor(TableConstructorExp),
    FuncCall(FuncCallExp),
    FuncDef(FuncDefExp),
    Parens(Box<Exp>),
    TableAccess(TableAccessExp),
}
