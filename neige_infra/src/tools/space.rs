use std::{fs::File, path::PathBuf};

use anyhow::{anyhow, Result};

pub fn read_file(path: &str) -> Result<File> {
    let path: PathBuf = path.into();
    if path.exists() {
        let file = File::open(&path)?;
        return Ok(file);
    }
    let path = PathBuf::from("..").join(&path);
    if path.exists() {
        let file = File::open(path)?;
        Ok(file)
    } else {
        Err(anyhow!("path errror"))
    }
}
