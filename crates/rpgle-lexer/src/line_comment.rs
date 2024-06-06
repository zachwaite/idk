use crate::core::{
    ch, read_until_column, read_until_end_of_line, text_at, CommentType, FormType,
    IllegalLexerState, Lexer, LexerException, Span, Token, TokenKind,
};

fn read_sequence(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 5;
    let start = lexer.state.borrow().position;
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    if end.col == c {
        Ok(Token::new(TokenKind::Sequence, &txt, span))
    } else {
        let ex = LexerException::IncompletePositionalEntry;
        let tok = Token::new(TokenKind::Idk(ex), &txt, span);
        Ok(tok)
    }
}

fn read_form_type(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 6;
    let cur = ch(&lexer);
    let start = lexer.state.borrow().position;
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    match end.col {
        6 => match cur {
            Some(' ') => Ok(Token::new(TokenKind::FormType(FormType::Empty), &txt, span)),
            Some('H') => Ok(Token::new(
                TokenKind::FormType(FormType::Control),
                &txt,
                span,
            )),
            Some('F') => Ok(Token::new(TokenKind::FormType(FormType::File), &txt, span)),
            Some('D') => Ok(Token::new(
                TokenKind::FormType(FormType::Definition),
                &txt,
                span,
            )),
            Some('C') => Ok(Token::new(
                TokenKind::FormType(FormType::Calculation),
                &txt,
                span,
            )),
            Some('I') => Ok(Token::new(TokenKind::FormType(FormType::Input), &txt, span)),
            Some('O') => Ok(Token::new(
                TokenKind::FormType(FormType::Output),
                &txt,
                span,
            )),
            Some('P') => Ok(Token::new(
                TokenKind::FormType(FormType::Procedure),
                &txt,
                span,
            )),
            _ => Ok(Token::new(TokenKind::FormType(FormType::Idk), &txt, span)),
        },
        _ => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_comment(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let start = lexer.state.borrow().position;
    read_until_end_of_line(lexer)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    let tok = Token::new(TokenKind::Comment(CommentType::LineComment), &txt, span);
    Ok(tok)
}

pub fn next_token(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let col = lexer.state.borrow().position.col;
    match col {
        0 => read_sequence(lexer),
        5 => read_form_type(lexer),
        6 => read_comment(lexer),
        _ => Err(IllegalLexerState::ImpossibleDestination),
    }
}
