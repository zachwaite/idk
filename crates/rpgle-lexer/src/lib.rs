mod core;
mod cspec;
mod dspec;
mod fixed;
mod free;
mod fspec;
mod full_free;
mod hspec;
mod ispec;
mod ospec;
mod pspec;

use core::{peek, IllegalLexerState, Lexer, LexerMode, Span, Token, TokenKind};

pub fn next_token(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    // guard for eof
    if peek(lexer).is_none() {
        return Ok(Token::new(TokenKind::Eof, "", Span::empty()));
    }

    let mode = lexer.state.borrow().mode;
    match mode {
        LexerMode::FullFree => full_free::next_token(lexer),
        LexerMode::Fixed => fixed::next_token(lexer),
        LexerMode::Free => free::next_token(lexer),
        LexerMode::Init => Err(IllegalLexerState::ImpossibleDestination),
    }
}

#[cfg(test)]
mod tests {}
