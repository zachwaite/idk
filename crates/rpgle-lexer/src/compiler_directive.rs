use crate::{
    core::{
        ch, is_letter, read_identifier, read_until_column, read_until_end_of_line, text_at,
        CompilerDirectiveType, FormType, IllegalLexerState, Lexer, LexerException, Span, Token,
        TokenKind,
    },
    read_char,
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
        (6, Some(' ')) => Ok(Token::new(TokenKind::FormType(FormType::Empty), &txt, span)),
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_directive_type(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let start = lexer.state.borrow().position;
    read_char(lexer)?;
    let kind = match ch(lexer) {
        Some(x) => {
            if is_letter(x) {
                let is = lexer.state.borrow().position.idx;
                read_identifier(lexer)?;
                let ie = lexer.state.borrow().position.idx;
                let literal = lexer.input[is..ie].iter().collect::<String>();
                match literal.to_uppercase().as_str() {
                    "FREE" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::Free),
                    "END-FREE" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::EndFree),
                    "TITLE" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::Title),
                    "EJECT" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::Eject),
                    "SPACE" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::Space),
                    "COPY" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::Copy),
                    "INCLUDE" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::Include),
                    "IF" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::If),
                    "ELSEIF" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::Elseif),
                    "ELSE" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::Else),
                    "ENDIF" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::Endif),
                    "EOF" => TokenKind::CompilerDirectiveType(CompilerDirectiveType::Eof),
                    _ => TokenKind::Idk(LexerException::NotImplemented),
                }
            } else {
                TokenKind::Idk(LexerException::NotImplemented)
            }
        }
        None => TokenKind::Idk(LexerException::NotImplemented),
    };
    read_until_end_of_line(lexer)?;
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
        6 => read_directive_type(lexer),
        _ => Err(IllegalLexerState::ImpossibleDestination),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{new_lexer, FormType, Position, TokenKind};

    use super::*;

    #[test]
    fn test_next_token_01() {
        let input = &r#"
      /free                                                                                         
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
                TokenKind::FormType(FormType::Empty),
                " ",
                Span {
                    start: Position::new(0, 5, 5),
                    end: Position::new(0, 6, 6),
                },
            ),
            Token::new(
                TokenKind::CompilerDirectiveType(CompilerDirectiveType::Free),
                "/free                                                                                         ",
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

    #[test]
    fn test_next_token_02() {
        let input = &r#"
      /End-Free                                                                                     
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
                TokenKind::FormType(FormType::Empty),
                " ",
                Span {
                    start: Position::new(0, 5, 5),
                    end: Position::new(0, 6, 6),
                },
            ),
            Token::new(
                TokenKind::CompilerDirectiveType(CompilerDirectiveType::EndFree),
                "/End-Free                                                                                     ",
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
