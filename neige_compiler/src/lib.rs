mod ast;
mod lexer;
mod parse;

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use neige_infra::read_file;

    use crate::lexer;

    #[test]
    fn lex_test() -> Result<()> {
        let file = read_file("example/set_meta.lua")?;
        lexer::Lex::new(file);
        Ok(())
    }
}
