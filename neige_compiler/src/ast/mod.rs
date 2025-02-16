// 代码块
mod exp;
mod stat;

use exp::Exp;
use stat::Stat;

#[derive(Debug)]
pub struct Block {
    last_line: i32,     // 行号
    stats: Vec<Stat>,   // 语句
    ret_exps: Vec<Exp>, // 返回表达式
}
