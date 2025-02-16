use super::{
    exp::{Exp, FuncCallExp, FuncDefExp},
    Block,
};

#[derive(Debug)]
struct WhileStat {
    exp: Box<Exp>,
    block: Box<Block>,
}

#[derive(Debug)]
struct RepeatStat {
    block: Box<Block>,
    exp: Box<Exp>,
}

#[derive(Debug)]
struct IfStat {
    exps: Vec<Box<Exp>>,
    blocks: Vec<Box<Block>>,
}

#[derive(Debug)]
struct ForNumStat {
    line_of_for: i32,
    line_of_do: i32,
    var_name: String,
    init_exp: Box<Exp>,
    limit_exp: Box<Exp>,
    step_exp: Box<Exp>,
    block: Box<Block>,
}

#[derive(Debug)]
struct ForInStat {
    line_of_do: i32,
    name_list: Vec<String>,
    exp_list: Vec<Box<Exp>>,
    block: Box<Block>,
}

#[derive(Debug)]
struct LocalVarDeclStat {
    last_line: i32,
    name_list: Vec<String>,
    exp_list: Vec<Box<Exp>>,
}

#[derive(Debug)]
struct AssignStat {
    last_line: i32,
    var_list: Vec<Box<Exp>>,
    exp_list: Vec<Box<Exp>>,
}

#[derive(Debug)]
struct LocalFuncDefStat {
    name: String,
    exp: Box<FuncDefExp>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Stat {
    Empty,
    Break(i32),
    Label(String),
    Goto(String),
    Do(Box<Block>),
    FuncCall(FuncCallExp),
    While(WhileStat),
    Repeat(RepeatStat),
    If(IfStat),
    ForNum(ForNumStat),
    ForIn(ForInStat),
    LocalVarDecl(LocalVarDeclStat),
    Assign(AssignStat),
    LocalFuncDef(LocalFuncDefStat),
}
