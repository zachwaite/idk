mod scanner;
mod parser;
pub use scanner::{scan, Token};
pub use parser::parse;

pub fn zzz(input: &str) -> Result<String, String>{
    let tokens = scan(input);
    let pgm = parse(&tokens).unwrap();
    dbg!("{}", pgm);
    Ok("".to_string())
}
