mod block;
mod exp;
mod stat;

use std::io::Cursor;

use crate::{ast::Block, lexer};

#[allow(dead_code)]
pub fn parse(chunk: Vec<u8>) -> Block {
    let read = Cursor::new(chunk);
    let mut lx = lexer::Lex::new(read);
    let block = block::parse_block(&mut lx);
    lx.expect(neige_infra::LuaToken::Eos);
    block
}
