use std::rc::Rc;

use neige_infra::Prototype;

mod chunk;
mod reader;

pub fn undump(data: Vec<u8>) -> Rc<Prototype> {
    let mut r = reader::Reader::new(data);
    r.check_header();
    r.read_byte();
    r.read_proto(None)
}
