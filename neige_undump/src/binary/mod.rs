use std::rc::Rc;

use neige_infra::Prototype;

mod chunk;
mod reader;

pub(self) const LUA_SIGNATURE: [u8; 4] = [0x1b, 0x4c, 0x75, 0x61]; // "\x1bLua"
pub(self) const LUAC_VERSION: u8 = 0x53;
pub(self) const LUAC_FORMAT: u8 = 0;
pub(self) const LUAC_DATA: [u8; 6] = [0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a]; // "\x19\x93\r\n\x1a\n"
pub(self) const CINT_SIZE: u8 = 4;
pub(self) const CSIZET_SIZE: u8 = 8;
pub(self) const INSTRUCTION_SIZE: u8 = 4;
pub(self) const LUA_INTEGER_SIZE: u8 = 8;
pub(self) const LUA_NUMBER_SIZE: u8 = 8;
pub(self) const LUAC_INT: i64 = 0x5678;
pub(self) const LUAC_NUM: f64 = 370.5;

pub fn undump(data: Vec<u8>) -> Rc<Prototype> {
    let mut r = reader::Reader::new(data);
    r.check_header();
    r.read_byte();
    r.read_proto(None)
}
