pub mod next;
mod tool;
mod value;

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
            line: 1,
        }
    }

    pub fn code_line(&self) -> usize {
        self.line
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use neige_infra::{read_file, LuaToken};

    use crate::lexer;

    #[test]
    fn lex_test_meta() -> Result<()> {
        let file = read_file("example/set_meta.lua")?;
        let mut lexer = lexer::Lex::new(file);
        while lexer.peek() != &LuaToken::Eos {
            println!("Token: {:?} -> {:?}", lexer.next(), lexer.code_line())
        }
        Ok(())
    }
}
