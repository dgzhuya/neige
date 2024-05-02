use std::{fs::File, io::Read};

use anyhow::{Ok, Result};

mod binary;
mod info;

use binary::undump;

fn main() -> Result<()> {
    let mut file = File::open("examples/test.out")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    undump(data);
    Ok(())
}
