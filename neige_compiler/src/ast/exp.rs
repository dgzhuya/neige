use super::Block;

#[derive(Debug, PartialEq)]
pub struct StringExp {
    line: usize,
    str: String,
}

#[derive(Debug, PartialEq)]
pub struct NameExp {
    line: usize,
    name: String,
}

#[derive(Debug, PartialEq)]
pub struct UnopExp {
    line: usize,
    op: usize,
    exp: Box<Exp>,
}

#[derive(Debug, PartialEq)]
pub struct BinopExp {
    line: usize,
    op: usize,
    exp1: Box<Exp>,
    exp2: Box<Exp>,
}

#[derive(Debug, PartialEq)]
pub struct ConcatExp {
    line: usize,
    exps: Vec<Box<Exp>>,
}

#[derive(Debug, PartialEq)]
pub struct TableConstructorExp {
    line: usize,
    last_line: usize,
    key_exps: Vec<Box<Exp>>,
    val_exps: Vec<Box<Exp>>,
}

#[derive(Debug, PartialEq)]
pub struct FuncCallExp {
    line: usize,
    last_line: usize,
    prefix_exp: Box<Exp>,
    name_exp: Option<Box<StringExp>>,
    args: Vec<Box<Exp>>,
}

#[derive(Debug, PartialEq)]
pub struct FuncDefExp {
    line: usize,
    last_line: usize,
    par_list: Vec<String>,
    is_vararg: bool,
    block: Box<Block>,
}

#[derive(Debug, PartialEq)]
pub struct TableAccessExp {
    last_line: usize,
    prefix_exp: Box<Exp>,
    key_exp: Box<Exp>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Exp {
    Nil(usize),
    True(usize),
    False(usize),
    Vararg(usize),
    Integer(usize, i64),
    Float(usize, f64),
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
