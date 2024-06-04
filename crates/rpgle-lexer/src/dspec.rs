use crate::core::{
    peek, read_until_column, text_at, IllegalLexerState, Lexer, LexerException, Span, Token,
    TokenKind,
};

pub fn next_token(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    todo!()
}
