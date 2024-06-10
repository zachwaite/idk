use crate::core::{
    ch, is_identifier_char, is_numeric, read_char, read_identifier, read_number,
    read_spaces_or_tabs, read_string_literal, read_until_column, read_until_end_of_line, text_at,
    CommentType, FormType, IllegalLexerState, Lexer, LexerException, Span, Token, TokenKind,
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

fn read_reserved(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let start = lexer.state.borrow().position;
    let maybe = ch(lexer);
    read_char(lexer)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    match (end.col, maybe) {
        (7, Some(' ')) => Ok(Token::new(TokenKind::Reserved, &txt, span)),
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_free(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let start = lexer.state.borrow().position;
    let kind = match ch(lexer) {
        Some(' ') | Some('\t') => {
            read_spaces_or_tabs(lexer)?;
            TokenKind::Whitespace
        }
        Some('=') => {
            read_char(lexer)?;
            TokenKind::Equals
        }
        Some('<') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some('>') => {
                    read_char(lexer)?;
                    TokenKind::NotEquals
                }
                Some('=') => {
                    read_char(lexer)?;
                    TokenKind::GreaterThanOrEquals
                }
                _ => TokenKind::LessThan,
            }
        }
        Some('>') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some('=') => {
                    read_char(lexer)?;
                    TokenKind::LessThanOrEquals
                }
                _ => TokenKind::GreaterThan,
            }
        }
        Some(';') => {
            read_char(lexer)?;
            TokenKind::Semicolon
        }
        Some(':') => {
            read_char(lexer)?;
            TokenKind::Colon
        }
        Some('/') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some('/') => {
                    read_until_end_of_line(lexer)?;
                    TokenKind::Comment(CommentType::InlineComment)
                }
                Some(' ') => TokenKind::Slash,
                Some('=') => {
                    read_char(lexer)?;
                    TokenKind::SlashEqual
                }
                _ => {
                    read_until_end_of_line(lexer)?;
                    TokenKind::Idk(LexerException::NotImplemented)
                }
            }
        }
        Some('\'') => {
            read_string_literal(lexer)?;
            match ch(lexer) {
                Some('\'') => {
                    read_char(lexer)?;
                    TokenKind::StringLiteral
                }
                _ => TokenKind::Idk(LexerException::NotImplemented),
            }
        }
        Some('+') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some(' ') => TokenKind::Plus,
                Some('=') => {
                    read_char(lexer)?;
                    TokenKind::PlusEqual
                }
                _ => TokenKind::Idk(LexerException::NotImplemented),
            }
        }
        Some('-') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some(' ') => TokenKind::Minus,
                Some('=') => {
                    read_char(lexer)?;
                    TokenKind::MinusEqual
                }
                _ => TokenKind::Idk(LexerException::NotImplemented),
            }
        }
        Some('*') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some(' ') => TokenKind::Asterisk,
                Some('=') => {
                    read_char(lexer)?;
                    TokenKind::AsteriskEqual
                }
                Some('i') => {
                    read_identifier(lexer)?;
                    TokenKind::Indicator
                }
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
        Some('%') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some(x) => match is_identifier_char(&x) {
                    true => {
                        read_identifier(lexer)?;
                        TokenKind::BuiltinIdentifier
                    }
                    false => {
                        read_until_end_of_line(lexer)?;
                        TokenKind::Idk(LexerException::NotImplemented)
                    }
                },
                None => TokenKind::Eof,
            }
        }
        Some('(') => {
            read_char(lexer)?;
            TokenKind::LParen
        }
        Some(')') => {
            read_char(lexer)?;
            TokenKind::RParen
        }
        Some(x) => match is_identifier_char(&x) {
            true => match is_numeric(&x) {
                true => {
                    read_number(lexer)?;
                    TokenKind::Number
                }
                false => {
                    let is = lexer.state.borrow().position.idx;
                    read_identifier(lexer)?;
                    let ie = lexer.state.borrow().position.idx;
                    let literal = lexer.input[is..ie].iter().collect::<String>();
                    match literal.to_uppercase().as_str() {
                        "SETLL" => TokenKind::SetLL,
                        "SETGT" => TokenKind::SetGT,
                        "CHAIN" => TokenKind::Chain,
                        "READ" => TokenKind::Read,
                        "READE" => TokenKind::ReadE,
                        "READPE" => TokenKind::ReadPE,
                        "WRITE" => TokenKind::Write,
                        "UPDATE" => TokenKind::Update,
                        "DELETE" => TokenKind::Delete,
                        "IF" => TokenKind::If,
                        "OR" => TokenKind::Or,
                        "AND" => TokenKind::And,
                        "ELSE" => TokenKind::Else,
                        "ELSEIF" => TokenKind::Elseif,
                        "ENDIF" => TokenKind::Endif,
                        "DOU" => TokenKind::Dou,
                        "DOW" => TokenKind::Dow,
                        "ENDDO" => TokenKind::Enddo,
                        "ITER" => TokenKind::Iter,
                        "LEAVE" => TokenKind::Leave,
                        "RESET" => TokenKind::Reset,
                        "EVAL" => TokenKind::Eval,
                        "CLEAR" => TokenKind::Clear,
                        "BEGSR" => TokenKind::Begsr,
                        "ENDSR" => TokenKind::Endsr,
                        "EXSR" => TokenKind::Exsr,
                        _ => TokenKind::Identifier,
                    }
                }
            },
            false => {
                read_until_end_of_line(lexer)?;
                TokenKind::Idk(LexerException::NotImplemented)
            }
        },
        None => TokenKind::Eof,
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
        6 => read_reserved(lexer),
        _ => read_free(lexer),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{new_lexer, FormType, Position, TokenKind};

    use super::*;
    #[test]
    fn test_next_token() {
        let input = &r#"
       // Look up LastId                                                                            
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
                TokenKind::Reserved,
                " ",
                Span {
                    start: Position::new(0, 6, 6),
                    end: Position::new(0, 7, 7),
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::InlineComment),
                "// Look up LastId                                                                            ",
                Span {
                    start: Position::new(0, 7, 7),
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