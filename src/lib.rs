mod error;
mod parser;
mod token;
mod verifier;

pub fn verify(p: String) -> Result<(), error::Error> {
    let scanner = token::Scanner::new(&p);
    let mut parser = parser::Parser::new(scanner);
    let mut defs = Vec::new();
    while let Some(def) = parser.parse_def()? {
        defs.push(def);
    }
    let mut verifier = verifier::Verifier::new(defs);
    while verifier.verify_next()?.is_some() {}
    Ok(())
}
