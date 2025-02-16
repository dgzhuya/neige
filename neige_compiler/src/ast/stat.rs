use super::{
    exp::{Exp, FuncCallExp, FuncDefExp},
    Block,
};

#[derive(Debug, PartialEq)]
pub struct WhileStat {
    pub(crate) exp: Box<Exp>,
    pub(crate) block: Box<Block>,
}

#[derive(Debug, PartialEq)]
pub struct RepeatStat {
    pub(crate) block: Box<Block>,
    pub(crate) exp: Box<Exp>,
}

#[derive(Debug, PartialEq)]
pub struct IfStat {
    pub(crate) exps: Vec<Box<Exp>>,
    pub(crate) blocks: Vec<Box<Block>>,
}

#[derive(Debug, PartialEq)]
pub struct ForNumStat {
    pub(crate) line_of_for: usize,
    pub(crate) line_of_do: usize,
    pub(crate) var_name: String,
    pub(crate) init_exp: Box<Exp>,
    pub(crate) limit_exp: Box<Exp>,
    pub(crate) step_exp: Box<Exp>,
    pub(crate) block: Box<Block>,
}

#[derive(Debug, PartialEq)]
pub struct ForInStat {
    pub(crate) line_of_do: usize,
    pub(crate) name_list: Vec<String>,
    pub(crate) exp_list: Vec<Box<Exp>>,
    pub(crate) block: Box<Block>,
}

#[derive(Debug, PartialEq)]
pub struct LocalVarDeclStat {
    pub(crate) last_line: usize,
    pub(crate) name_list: Vec<String>,
    pub(crate) exp_list: Vec<Box<Exp>>,
}

#[derive(Debug, PartialEq)]
pub struct AssignStat {
    pub(crate) last_line: usize,
    pub(crate) var_list: Vec<Box<Exp>>,
    pub(crate) exp_list: Vec<Box<Exp>>,
}

#[derive(Debug, PartialEq)]
pub struct LocalFuncDefStat {
    pub(crate) name: String,
    pub(crate) exp: Box<FuncDefExp>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Stat {
    Break(usize),
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
