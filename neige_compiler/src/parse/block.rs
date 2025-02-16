use neige_infra::LuaToken;

use crate::{ast, lexer};

use super::{exp::parse_exp_list, stat::parse_stat};

pub(super) fn parse_block(lx: &mut lexer::Lex) -> ast::Block {
    ast::Block {
        stats: parse_stats(lx),
        ret_exps: parse_ret_exps(lx),
        last_line: lx.code_line(),
    }
}

fn parse_stats(lexer: &mut lexer::Lex) -> Vec<ast::Stat> {
    let mut stats = Vec::new();
    loop {
        if is_ret_or_block_end(lexer.peek()) {
            break;
        }
        if let Some(stat) = parse_stat(lexer) {
            stats.push(stat);
        }
    }
    return stats;
}

fn is_ret_or_block_end(token: &LuaToken) -> bool {
    match token {
        LuaToken::Return
        | LuaToken::Eos
        | LuaToken::End
        | LuaToken::Else
        | LuaToken::Elseif
        | LuaToken::Until => true,
        _ => false,
    }
}

fn parse_ret_exps(lexer: &mut lexer::Lex) -> Option<Vec<ast::Exp>> {
    if lexer.peek() == &LuaToken::Return {
        lexer.next(); // skip return
        match lexer.peek() {
            LuaToken::Eos | LuaToken::End | LuaToken::Else | LuaToken::Elseif | LuaToken::Until => {
                Some(Vec::new())
            }
            LuaToken::SemiColon => {
                lexer.next();
                Some(Vec::new())
            }
            _ => {
                let exps = parse_exp_list(lexer);
                if lexer.peek() == &LuaToken::SemiColon {
                    lexer.next();
                }
                Some(exps)
            }
        }
    } else {
        None
    }
}
