use std::{
    io::{Bytes, Read},
    iter::Peekable,
};

use neige_infra::LuaToken;

#[allow(dead_code)]
pub struct Lex {
    input: Peekable<Bytes<Box<dyn Read>>>,
    ahead: LuaToken,
    line: usize,
}
#[allow(dead_code)]
impl Lex {
    pub fn new<R: Read + 'static>(input: R) -> Self {
        let input: Box<dyn Read> = Box::new(input);
        Self {
            input: input.bytes().peekable(),
            ahead: LuaToken::Eos,
            line: 0,
        }
    }
}
