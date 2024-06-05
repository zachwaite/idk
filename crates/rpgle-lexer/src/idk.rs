use crate::core::{
    read_until_end_of_line, text_at, IllegalLexerState, Lexer, LexerException, Span, Token,
    TokenKind,
};

pub fn next_token(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let start = lexer.state.borrow().position;
    read_until_end_of_line(lexer)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    let ex = LexerException::NotImplemented;
    let tok = Token::new(TokenKind::Idk(ex), &txt, span);
    Ok(tok)
}
