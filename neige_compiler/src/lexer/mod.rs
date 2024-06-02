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
