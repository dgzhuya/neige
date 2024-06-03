mod binary;
mod info;

pub use binary::undump;

#[cfg(test)]
#[cfg(not(feature = "wasm"))]
mod tests {
    use std::io::BufReader;

    use anyhow::{Ok, Result};
    use neige_infra::read_file;

    use crate::binary::undump;
    use crate::info::ProtoPrint;

    #[test]
    fn proto_undump_test() -> Result<()> {
        let file = read_file("example/test.out")?;
        let data = BufReader::new(file);
        let proto = undump(data, "test.out");
        proto.list_proto();
        Ok(())
    }
}
