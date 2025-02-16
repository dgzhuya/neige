use std::mem;

use neige_infra::LuaToken;

use super::Lex;

#[allow(dead_code)]
impl Lex {
    pub fn next(&mut self) -> LuaToken {
        if self.ahead == LuaToken::Eos {
            self.do_next()
        } else {
            mem::replace(&mut self.ahead, LuaToken::Eos)
        }
    }

    pub fn peek(&mut self) -> &LuaToken {
        if self.ahead == LuaToken::Eos {
            self.ahead = self.do_next();
        }
        &self.ahead
    }

    pub fn expect(&mut self, token: LuaToken) {
        assert_eq!(self.next(), token);
    }
}

impl Lex {
    pub(super) fn next_byte(&mut self) -> Option<u8> {
        self.input.next().map(|r| r.unwrap())
    }

    pub(super) fn peek_byte(&mut self) -> u8 {
        match self.input.peek() {
            Some(Ok(byt)) => *byt,
            Some(_) => panic!("Unexpected EOF"),
            None => b'\0',
        }
    }
}

impl Lex {
    pub(super) fn do_next(&mut self) -> LuaToken {
        if let Some(byt) = self.next_byte() {
            match byt {
                b'\n' => {
                    self.line += 1;
                    if self.peek_byte() == b'\r' {
                        self.next_byte();
                    }
                    self.do_next()
                }
                b'\r' => {
                    self.line += 1;
                    if self.peek_byte() == b'\n' {
                        self.next_byte();
                    }
                    self.do_next()
                }
                b'\t' | b' ' => self.do_next(),
                b'+' => LuaToken::Add,
                b'*' => LuaToken::Mul,
                b'%' => LuaToken::Mod,
                b'^' => LuaToken::Pow,
                b'#' => LuaToken::Len,
                b'&' => LuaToken::BitAnd,
                b'|' => LuaToken::BitOr,
                b'(' => LuaToken::ParL,
                b')' => LuaToken::ParR,
                b'{' => LuaToken::CurlyL,
                b'}' => LuaToken::CurlyR,
                b'[' => LuaToken::SqurL,
                b']' => LuaToken::SqurR,
                b';' => LuaToken::SemiColon,
                b',' => LuaToken::Comma,
                b'/' => self.check_ahead(b'/', LuaToken::IDiv, LuaToken::Div),
                b'=' => self.check_ahead(b'=', LuaToken::Equal, LuaToken::Assign),
                b'~' => self.check_ahead(b'=', LuaToken::NotEq, LuaToken::BitNot),
                b':' => self.check_ahead(b':', LuaToken::DoubColon, LuaToken::Colon),
                b'<' => self.check_ahead2(
                    b'=',
                    LuaToken::LesEq,
                    b'<',
                    LuaToken::ShiftL,
                    LuaToken::Less,
                ),
                b'>' => self.check_ahead2(
                    b'=',
                    LuaToken::GreEq,
                    b'>',
                    LuaToken::ShiftR,
                    LuaToken::Greater,
                ),
                b'\'' | b'"' => self.read_string(byt),
                b'.' => match self.peek_byte() {
                    b'.' => {
                        self.next_byte();
                        if self.peek_byte() == b'.' {
                            self.next_byte();
                            LuaToken::Dots
                        } else {
                            LuaToken::Concat
                        }
                    }
                    b'0'..=b'9' => self.read_decimal('.'),
                    _ => LuaToken::Dot,
                },
                b'-' => {
                    if self.peek_byte() == b'-' {
                        self.next_byte();
                        self.read_comment();
                        self.do_next()
                    } else {
                        LuaToken::Sub
                    }
                }
                ch @ b'0'..=b'9' => self.read_decimal(ch as char),
                b'A'..=b'Z' | b'a'..=b'z' | b'_' => self.read_name(byt),
                _ => panic!("invalid char {byt}"),
            }
        } else {
            LuaToken::Eos
        }
    }
}
