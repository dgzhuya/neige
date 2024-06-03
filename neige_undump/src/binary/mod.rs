use std::{fs::File, io::BufReader, rc::Rc};

use neige_infra::Prototype;

mod chunk;
mod reader;

#[cfg(not(feature = "wasm"))]
pub fn undump(data: BufReader<File>, source: &str) -> Rc<Prototype> {
    let mut r = reader::Reader::new(data);
    r.check_header();
    r.read_byte();
    r.read_proto(Some(source.into()))
}

#[cfg(feature = "wasm")]
pub fn undump(data: Vec<u8>, source: &str) -> Rc<Prototype> {
    let mut r = reader::Reader::new(data);
    r.check_header();
    r.read_byte();
    r.read_proto(Some(source.into()))
}
