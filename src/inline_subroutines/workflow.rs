use super::parser;

pub fn refactor(input: &str) -> Result<String, String> {
    let tokens = scan(input);
    dbg!("{}", tokens);
    Ok("".to_string())
}
