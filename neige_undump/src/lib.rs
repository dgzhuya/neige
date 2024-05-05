pub mod binary;
pub mod info;

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use anyhow::{Ok, Result};
    use neige_infra::read_file;

    use crate::binary::undump;
    use crate::info::ProtoPrint;

    #[test]
    fn proto_undump_test() -> Result<()> {
        let file = read_file("data/test.out")?;
        let data = BufReader::new(file);
        let proto = undump(data);
        proto.list_proto();
        Ok(())
    }
}
