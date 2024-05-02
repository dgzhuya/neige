use std::{
    fs::File,
    io::{Error, Read},
};
mod binary;
mod info;

use crate::binary::undump;

fn main() -> Result<(), Error> {
    let mut file = File::open("/Users/pinktu/Desktop/neige_lua/examples/test.out")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    let proto = undump(data);

    Ok(())
}
