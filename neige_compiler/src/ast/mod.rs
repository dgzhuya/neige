// 代码块
mod exp;
mod stat;

pub use {exp::Exp, stat::Stat};

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stats: Vec<Stat>,           // 语句
    pub ret_exps: Option<Vec<Exp>>, // 返回表达式
    pub last_line: usize,           // 行号
}
