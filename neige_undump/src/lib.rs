pub mod binary;
pub mod info;

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use anyhow::{Ok, Result};
    use neige_infra::read_file;

    use crate::binary::undump;

    #[test]
    fn proto_undump_test() -> Result<()> {
        let file = read_file("data/test.out")?;
        let data = BufReader::new(file);
        #[allow(unused_variables)]
        let proto = undump(data);
        Ok(())
    }
}
