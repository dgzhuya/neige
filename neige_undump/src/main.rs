use std::{
    fs::File,
    io::{Error, Read},
};
mod binary;
mod info;

use crate::binary::undump;

fn main() -> Result<(), Error> {
    let mut file = File::open("examples/test.out")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    #[allow(unused_variables)]
    let proto = undump(data);

    Ok(())
}
