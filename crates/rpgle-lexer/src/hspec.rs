use crate::core::{
    ch, is_identifier_char, read_char, read_identifier, read_spaces_or_tabs, read_until_column,
    read_until_end_of_line, text_at, FormType, IllegalLexerState, Lexer, LexerException, Span,
    Token, TokenKind,
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
    let start = lexer.state.borrow().position;
    let maybe_h = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    match (end.col, maybe_h) {
        (6, Some('H')) => Ok(Token::new(
            TokenKind::FormType(FormType::Control),
            &txt,
            span,
        )),
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_keywords(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let start = lexer.state.borrow().position;
    let kind = match ch(lexer) {
        Some(' ') | Some('\t') => {
            read_spaces_or_tabs(lexer)?;
            TokenKind::Whitespace
        }
        Some('(') => {
            read_char(lexer)?;
            TokenKind::LParen
        }
        Some(')') => {
            read_char(lexer)?;
            TokenKind::RParen
        }
        Some(':') => {
            read_char(lexer)?;
            TokenKind::Colon
        }
        Some('*') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some(x) => match is_identifier_char(&x) {
                    true => {
                        read_identifier(lexer)?;
                        TokenKind::IndicatorValue
                    }
                    false => {
                        read_until_end_of_line(lexer)?;
                        TokenKind::Idk(LexerException::NotImplemented)
                    }
                },
                None => TokenKind::Eof,
            }
        }
        Some(x) => match is_identifier_char(&x) {
            true => {
                let is = lexer.state.borrow().position.idx;
                read_identifier(lexer)?;
                let ie = lexer.state.borrow().position.idx;
                let literal = lexer.input[is..ie].iter().collect::<String>();
                match literal.to_uppercase().as_str() {
                    "OPTION" => TokenKind::Option,
                    "DATEDIT" => TokenKind::Datedit,
                    "DATFMT" => TokenKind::Datfmt,
                    "TIMFMT" => TokenKind::Timfmt,
                    "DFTACTGRP" => TokenKind::Dftactgrp,
                    "DEBUG" => TokenKind::Debug,
                    _ => TokenKind::Identifier,
                }
            }
            false => {
                read_until_end_of_line(lexer)?;
                TokenKind::Idk(LexerException::NotImplemented)
            }
        },
        _ => TokenKind::Idk(LexerException::NotImplemented),
    };
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    let tok = Token::new(kind, &txt, span);
    Ok(tok)
}

pub fn next_token(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let col = lexer.state.borrow().position.col;
    match col {
        0 => read_sequence(lexer),
        5 => read_form_type(lexer),
        6.. => read_keywords(lexer),
        _ => Err(IllegalLexerState::ImpossibleDestination),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{new_lexer, FormType, Position, TokenKind};

    use super::*;

    // FCowEvt    UF A E           K DISK
    // FBornEvt   UF A E           K DISK
    // FCowEvtL2  IF   E           K DISK     Rename(EVTFMT:VEVTFMT) Prefix(V)
    // F**********************************************************************************************
    // D**********************************************************************************************
    // D LastId          S              8  0
    // C**********************************************************************************************

    #[test]
    fn test_next_token() {
        let input = &r#"
     H OPTION(*nodebugio:*srcstmt)                                                                  
"#[1..];
        let expected: Vec<Token> = vec![
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start: Position::new(0, 0, 0),
                    end: Position::new(0, 5, 5),
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::Control),
                "H",
                Span {
                    start: Position::new(0, 5, 5),
                    end: Position::new(0, 6, 6),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " OPTION(*nodebugio:*srcstmt)                                                                  ",
                Span {
                    start: Position::new(0, 6, 6),
                    end: Position::new(0, 100, 100),
                },
            ),
        ];
        let lexer = new_lexer(input);
        for pair in expected.into_iter().enumerate() {
            // println!("`{}` {}", pair.1.kind, lexer.state.borrow().position);
            let idx = pair.0.to_string();
            let expected_token = pair.1;
            let observed_token = next_token(&lexer).unwrap();
            assert_eq!(observed_token, expected_token, "test #{}", idx);
        }
    }
}
