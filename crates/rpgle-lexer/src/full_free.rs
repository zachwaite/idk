use crate::core::{
    read_until_end_of_file, text_at, IllegalLexerState, Lexer, LexerException, Position, Span,
    Token, TokenKind,
};

pub fn next_token(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    read_until_end_of_file(lexer)?;
    let current_position = lexer.state.borrow().position;
    let span = Span {
        start: Position::empty(),
        end: current_position,
    };
    let txt = text_at(lexer, span);
    let ex = LexerException::NotImplemented;
    let tok = Token::new(TokenKind::Idk(ex), &txt, span);
    Ok(tok)
}

#[cfg(test)]
mod tests {
    use crate::core::new_lexer;

    use super::*;

    #[test]
    fn test_next_token() {
        let input = &r#"
**free
// foo bar baz
"#[1..];
        let expected: Vec<Token> = vec![Token::new(
            TokenKind::Idk(LexerException::NotImplemented),
            "**free\n// foo bar baz\n",
            Span {
                start: Position::new(0, 0, 0),
                end: Position::new(2, 0, 22),
            },
        )];
        let lexer = new_lexer(input);
        for pair in expected.into_iter().enumerate() {
            println!("{}", lexer.state.borrow().position);
            let idx = pair.0.to_string();
            let expected_token = pair.1;
            let observed_token = next_token(&lexer).unwrap();
            assert_eq!(observed_token, expected_token, "test #{}", idx);
        }
    }
}
