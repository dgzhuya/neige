use crate::{ast, ast::Stat, lexer};
use neige_infra::LuaToken;

use super::{block::parse_block, exp::parse_exp};

pub(super) fn parse_stat(lexer: &mut lexer::Lex) -> Option<ast::Stat> {
    match lexer.peek() {
        LuaToken::SemiColon => {
            lexer.expect(LuaToken::SemiColon);
            None
        }
        LuaToken::Break => {
            lexer.expect(LuaToken::Break);
            Some(ast::Stat::Break(lexer.code_line()))
        }
        LuaToken::DoubColon => {
            lexer.expect(LuaToken::DoubColon);
            let name = lexer.next_identifer();
            lexer.expect(LuaToken::DoubColon);
            Some(Stat::Label(name))
        }
        LuaToken::Goto => {
            lexer.expect(LuaToken::Goto);
            let name = lexer.next_identifer();
            Some(Stat::Goto(name))
        }
        LuaToken::Do => {
            lexer.expect(LuaToken::Do);
            let block = parse_block(lexer);
            lexer.expect(LuaToken::End);
            Some(Stat::Do(Box::new(block)))
        }
        LuaToken::While => {
            lexer.expect(LuaToken::While);
            let cond = parse_exp(lexer);
            lexer.expect(LuaToken::Do);
            let block = parse_block(lexer);
            lexer.expect(LuaToken::End);
            Some(Stat::While(ast::WhileStat {
                exp: Box::new(cond),
                block: Box::new(block),
            }))
        }
        _ => {
            todo!("解析函数调用和赋值语句");
        }
    }
}
