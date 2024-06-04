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
