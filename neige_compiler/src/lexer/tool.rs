use neige_infra::LuaToken;

use super::Lex;

impl Lex {
    pub(super) fn check_ahead(&mut self, ahead: u8, long: LuaToken, short: LuaToken) -> LuaToken {
        if self.peek_byte() == ahead {
            self.next_byte();
            long
        } else {
            short
        }
    }

    // '--' has been read
    pub(super) fn read_comment(&mut self) {
        match self.next_byte() {
            None => (),
            Some(b'[') => {
                if self.peek_byte() == b'[' {
                    while let Some(byt) = self.next_byte() {
                        match byt {
                            b'\n' => {
                                self.line += 1;
                                if self.peek_byte() == b'\r' {
                                    self.next_byte();
                                }
                            }
                            b'\r' => {
                                self.line += 1;
                                if self.peek_byte() == b'\n' {
                                    self.next_byte();
                                }
                            }
                            b']' => {
                                if self.peek_byte() == b']' {
                                    self.next_byte();
                                    break;
                                }
                            }
                            _ => {}
                        };
                    }
                }
            }
            Some(_) => {
                while let Some(byt) = self.next_byte() {
                    if byt == b'\n' {
                        self.line += 1;
                        break;
                    }
                }
            }
        }
    }

    pub(super) fn check_ahead2(
        &mut self,
        ahead1: u8,
        long1: LuaToken,
        ahead2: u8,
        long2: LuaToken,
        short: LuaToken,
    ) -> LuaToken {
        let byt = self.peek_byte();
        if byt == ahead1 {
            self.next_byte();
            long1
        } else if byt == ahead2 {
            self.next_byte();
            long2
        } else {
            short
        }
    }
}
