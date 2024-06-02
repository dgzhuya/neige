use std::{
    io::{Bytes, Read},
    iter::Peekable,
};

use neige_infra::LuaToken;

pub struct Lex {
    input: Peekable<Bytes<Box<dyn Read>>>,
    ahead: LuaToken,
    line: usize,
}

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
