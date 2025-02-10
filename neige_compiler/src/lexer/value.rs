use neige_infra::LuaToken;

use super::Lex;

impl Lex {
    pub(super) fn read_decimal(&mut self, ahead: char) -> LuaToken {
        // TODO heximal
        let mut is_float = ahead == '.';
        let mut buf = String::new();
        buf.push(ahead);
        loop {
            let byt = self.peek_byte();
            match byt {
                b'0'..=b'9' => buf.push(byt as char),
                b'.' | b'e' | b'E' | b'+' | b'-' => {
                    buf.push(byt as char);
                    is_float = true;
                }
                _ => break,
            }
            self.next_byte();
        }

        if is_float {
            LuaToken::Float(buf.parse::<f64>().unwrap())
        } else {
            LuaToken::Integer(buf.parse::<i64>().unwrap())
        }
    }

    pub(super) fn read_string(&mut self, quote: u8) -> LuaToken {
        let mut s = Vec::new();
        loop {
            match self.next_byte().expect("unfinished string") {
                b'\n' => panic!("unfinished string"),
                b'\\' => s.push(self.read_escape()),
                byt if byt == quote => break,
                byt => s.push(byt),
            }
        }
        LuaToken::Str(String::from_utf8(s).unwrap())
    }
    pub(super) fn read_escape(&mut self) -> u8 {
        match self.next_byte().expect("string escape") {
            b'a' => 0x07,
            b'b' => 0x08,
            b'f' => 0x0c,
            b'v' => 0x0b,
            b'n' => b'\n',
            b'r' => b'\r',
            b't' => b'\t',
            b'\\' => b'\\',
            b'"' => b'"',
            b'\'' => b'\'',
            b'x' => {
                // format: \xXX
                let n1 = char::to_digit(self.next_byte().unwrap() as char, 16).unwrap();
                let n2 = char::to_digit(self.next_byte().unwrap() as char, 16).unwrap();
                (n1 * 16 + n2) as u8
            }
            ch @ b'0'..=b'9' => {
                // format: \d[d[d]]
                let mut n = char::to_digit(ch as char, 10).unwrap(); // TODO no unwrap
                if let Some(d) = char::to_digit(self.peek_byte() as char, 10) {
                    self.next_byte();
                    n = n * 10 + d;
                    if let Some(d) = char::to_digit(self.peek_byte() as char, 10) {
                        self.next_byte();
                        n = n * 10 + d;
                    }
                }
                u8::try_from(n).expect("decimal escape too large")
            }
            _ => panic!("invalid string escape"),
        }
    }

    pub(super) fn read_name(&mut self, first: u8) -> LuaToken {
        let mut s = String::new();
        s.push(first as char);

        loop {
            let ch = self.peek_byte() as char;
            if ch.is_alphanumeric() || ch == '_' {
                self.next_byte();
                s.push(ch);
            } else {
                break;
            }
        }

        match &s as &str {
            // TODO optimize by hash
            "and" => LuaToken::And,
            "break" => LuaToken::Break,
            "do" => LuaToken::Do,
            "else" => LuaToken::Else,
            "elseif" => LuaToken::Elseif,
            "end" => LuaToken::End,
            "false" => LuaToken::False,
            "for" => LuaToken::For,
            "function" => LuaToken::Function,
            "goto" => LuaToken::Goto,
            "if" => LuaToken::If,
            "in" => LuaToken::In,
            "local" => LuaToken::Local,
            "nil" => LuaToken::Nil,
            "not" => LuaToken::Not,
            "or" => LuaToken::Or,
            "repeat" => LuaToken::Repeat,
            "return" => LuaToken::Return,
            "then" => LuaToken::Then,
            "true" => LuaToken::True,
            "until" => LuaToken::Until,
            "while" => LuaToken::While,
            _ => LuaToken::Name(s),
        }
    }
}
