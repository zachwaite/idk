use crate::{
    core::{
        ch, is_identifier_char, is_numeric, is_space_or_tab, read_identifier, read_number,
        read_spaces_or_tabs, read_string_literal, read_until_column, read_until_end_of_line,
        text_at, DefinitionDataType, DefinitionType, FileFormatType, FormType, IllegalLexerState,
        Lexer, LexerException, Span, Token, TokenKind,
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
    let maybe = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    match (end.col, maybe) {
        (6, Some('C')) => Ok(Token::new(
            TokenKind::FormType(FormType::Calculation),
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

fn read_control_level(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 8;
    let start = lexer.state.borrow().position;
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    let ex = LexerException::NotImplemented;
    let tok = Token::new(TokenKind::Idk(ex), &txt, span);
    Ok(tok)
}

fn read_indicators(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 11;
    let start = lexer.state.borrow().position;
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    let ex = LexerException::NotImplemented;
    let tok = Token::new(TokenKind::Idk(ex), &txt, span);
    Ok(tok)
}

fn read_factor1(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 25;
    let start = lexer.state.borrow().position;
    let (is_ident, is_space) = if let Some(x) = ch(lexer) {
        (is_identifier_char(x), is_space_or_tab(x))
    } else {
        (false, false)
    };
    match (is_ident, is_space) {
        (true, false) => {
            while ch(lexer).is_some()
                && lexer.state.borrow().position.col < c
                && is_identifier_char(ch(lexer).unwrap())
            {
                read_char(lexer)?;
            }
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(TokenKind::Identifier, &txt, span))
        }
        (false, true) => {
            while ch(lexer).is_some()
                && lexer.state.borrow().position.col < c
                && is_space_or_tab(ch(lexer).unwrap())
            {
                read_char(lexer)?;
            }
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(TokenKind::Whitespace, &txt, span))
        }
        (_, _) => {
            read_until_column(lexer, c)?;
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                &txt,
                span,
            ))
        }
    }
}

fn read_operation_and_extender(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 35;
    let start = lexer.state.borrow().position;
    let (is_ident, is_space) = if let Some(x) = ch(lexer) {
        (is_identifier_char(x), is_space_or_tab(x))
    } else {
        (false, false)
    };
    match (is_ident, is_space) {
        (true, false) => {
            while ch(lexer).is_some()
                && lexer.state.borrow().position.col < c
                && is_identifier_char(ch(lexer).unwrap())
            {
                read_char(lexer)?;
            }
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            let kind = match txt.to_uppercase().as_str() {
                "BEGSR" => TokenKind::Begsr,
                "ENDSR" => TokenKind::Endsr,
                "EXSR" => TokenKind::Exsr,
                "CHAIN" => TokenKind::Chain,
                "MOVE" => TokenKind::Move,
                "WRITE" => TokenKind::Write,
                "UPDATE" => TokenKind::Update,
                "IF" => TokenKind::If,
                "PLIST" => TokenKind::Plist,
                "PARM" => TokenKind::Parm,
                _ => TokenKind::Identifier,
            };
            Ok(Token::new(kind, &txt, span))
        }
        (false, true) => {
            while ch(lexer).is_some()
                && lexer.state.borrow().position.col < c
                && is_space_or_tab(ch(lexer).unwrap())
            {
                read_char(lexer)?;
            }
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(TokenKind::Whitespace, &txt, span))
        }
        (_, _) => {
            read_until_column(lexer, c)?;
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                &txt,
                span,
            ))
        }
    }
}

fn read_factor2(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 49;
    let start = lexer.state.borrow().position;
    let (is_ident, is_space, is_special) = if let Some(x) = ch(lexer) {
        (
            is_identifier_char(x),
            is_space_or_tab(x),
            *x == '<' || *x == '>' || *x == '=', // TODO: fix this hack. This probably needs the
                                                 // free lexer, but need to know how to
                                                 // differentiate factor2 from extended factor2
        )
    } else {
        (false, false, false)
    };
    match (is_ident, is_space, is_special) {
        (true, false, false) => {
            while ch(lexer).is_some()
                && lexer.state.borrow().position.col < c
                && is_identifier_char(ch(lexer).unwrap())
            {
                read_char(lexer)?;
            }
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(TokenKind::Identifier, &txt, span))
        }
        (false, true, false) => {
            while ch(lexer).is_some()
                && lexer.state.borrow().position.col < c
                && is_space_or_tab(ch(lexer).unwrap())
            {
                read_char(lexer)?;
            }
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(TokenKind::Whitespace, &txt, span))
        }
        (false, false, true) => {
            let kind: TokenKind = match ch(lexer) {
                Some('<') => {
                    read_char(lexer)?;
                    match ch(lexer) {
                        Some('=') => TokenKind::LessThanOrEquals,
                        Some('>') => TokenKind::NotEquals,
                        Some(_) => TokenKind::LessThan,
                        None => TokenKind::Idk(LexerException::IncompletePositionalEntry),
                    }
                }
                Some('>') => {
                    read_char(lexer)?;
                    match ch(lexer) {
                        Some('=') => TokenKind::GreaterThanOrEquals,
                        Some(_) => TokenKind::GreaterThan,
                        None => TokenKind::Idk(LexerException::IncompletePositionalEntry),
                    }
                }
                Some('=') => TokenKind::Equals,
                Some(_) => TokenKind::Idk(LexerException::NotImplemented),
                None => TokenKind::Idk(LexerException::IncompletePositionalEntry),
            };
            read_char(lexer)?;
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(kind, &txt, span))
        }
        (_, _, _) => {
            read_until_column(lexer, c)?;
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                &txt,
                span,
            ))
        }
    }
}

fn read_result_field(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 63;
    let start = lexer.state.borrow().position;
    let (is_ident, is_space) = if let Some(x) = ch(lexer) {
        (is_identifier_char(x), is_space_or_tab(x))
    } else {
        (false, false)
    };
    match (is_ident, is_space) {
        (true, false) => {
            while ch(lexer).is_some()
                && lexer.state.borrow().position.col < c
                && is_identifier_char(ch(lexer).unwrap())
            {
                read_char(lexer)?;
            }
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(TokenKind::Identifier, &txt, span))
        }
        (false, true) => {
            while ch(lexer).is_some()
                && lexer.state.borrow().position.col < c
                && is_space_or_tab(ch(lexer).unwrap())
            {
                read_char(lexer)?;
            }
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(TokenKind::Whitespace, &txt, span))
        }
        (_, _) => {
            read_until_column(lexer, c)?;
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                &txt,
                span,
            ))
        }
    }
}

fn read_field_length(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 68;
    let start = lexer.state.borrow().position;
    let (is_number, is_space) = if let Some(x) = ch(lexer) {
        (is_numeric(x), is_space_or_tab(x))
    } else {
        (false, false)
    };
    match (is_number, is_space) {
        (true, false) => {
            while ch(lexer).is_some()
                && lexer.state.borrow().position.col < c
                && is_numeric(ch(lexer).unwrap())
            {
                read_char(lexer)?;
            }
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(TokenKind::Number, &txt, span))
        }
        (false, true) => {
            while ch(lexer).is_some()
                && lexer.state.borrow().position.col < c
                && is_space_or_tab(ch(lexer).unwrap())
            {
                read_char(lexer)?;
            }
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(TokenKind::Whitespace, &txt, span))
        }
        (_, _) => {
            read_until_column(lexer, c)?;
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            Ok(Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                &txt,
                span,
            ))
        }
    }
}

fn read_decimal_positions(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 70;
    let start = lexer.state.borrow().position;
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    let ex = LexerException::NotImplemented;
    let tok = Token::new(TokenKind::Idk(ex), &txt, span);
    Ok(tok)
}

fn read_resulting_indicators(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 76;
    let start = lexer.state.borrow().position;
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    let ex = LexerException::NotImplemented;
    let tok = Token::new(TokenKind::Idk(ex), &txt, span);
    Ok(tok)
}

fn read_extended_factor2_todo(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 100;
    let start = lexer.state.borrow().position;
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    let ex = LexerException::NotImplemented;
    let tok = Token::new(TokenKind::Idk(ex), &txt, span);
    Ok(tok)
}

pub fn next_token(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let col = lexer.state.borrow().position.col;
    match col {
        0 => read_sequence(lexer),
        5 => read_form_type(lexer),
        6 => read_control_level(lexer),
        8 => read_indicators(lexer),
        11..=24 => read_factor1(lexer),
        25..=34 => read_operation_and_extender(lexer),
        35..=48 => read_factor2(lexer),
        49..=62 => read_result_field(lexer),
        63..=67 => read_field_length(lexer),
        68 => read_decimal_positions(lexer),
        70 => read_resulting_indicators(lexer),
        76 => read_extended_factor2_todo(lexer),
        _ => Err(IllegalLexerState::ImpossibleDestination),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{new_lexer, Position, TokenKind};

    use super::*;

    #[test]
    fn test_next_token_01() {
        let input = &r#"
     C     $DoStuff      BegSr                                                                     
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
                TokenKind::FormType(FormType::Calculation),
                "C",
                Span {
                    start: Position::new(0, 5, 5),
                    end: Position::new(0, 6, 6),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "  ",
                Span {
                    start: Position::new(0, 6, 6),
                    end: Position::new(0, 8, 8),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "   ",
                Span {
                    start: Position::new(0, 8, 8),
                    end: Position::new(0, 11, 11),
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "$DoStuff",
                Span {
                    start: Position::new(0, 11, 11),
                    end: Position::new(0, 19, 19),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "      ",
                Span {
                    start: Position::new(0, 19, 19),
                    end: Position::new(0, 25, 25),
                },
            ),
            Token::new(
                TokenKind::Begsr,
                "BegSr",
                Span {
                    start: Position::new(0, 25, 25),
                    end: Position::new(0, 30, 30),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "     ",
                Span {
                    start: Position::new(0, 30, 30),
                    end: Position::new(0, 35, 35),
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
     C     somefld       chain     somefile                                                        
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
                TokenKind::FormType(FormType::Calculation),
                "C",
                Span {
                    start: Position::new(0, 5, 5),
                    end: Position::new(0, 6, 6),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "  ",
                Span {
                    start: Position::new(0, 6, 6),
                    end: Position::new(0, 8, 8),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "   ",
                Span {
                    start: Position::new(0, 8, 8),
                    end: Position::new(0, 11, 11),
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "somefld",
                Span {
                    start: Position::new(0, 11, 11),
                    end: Position::new(0, 18, 18),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "       ",
                Span {
                    start: Position::new(0, 18, 18),
                    end: Position::new(0, 25, 25),
                },
            ),
            Token::new(
                TokenKind::Chain,
                "chain",
                Span {
                    start: Position::new(0, 25, 25),
                    end: Position::new(0, 30, 30),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "     ",
                Span {
                    start: Position::new(0, 30, 30),
                    end: Position::new(0, 35, 35),
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "somefile",
                Span {
                    start: Position::new(0, 35, 35),
                    end: Position::new(0, 43, 43),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "      ",
                Span {
                    start: Position::new(0, 43, 43),
                    end: Position::new(0, 49, 49),
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
    fn test_next_token_03() {
        let input = &r#"
     C                   move      *blanks       SomeResult                                         
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
                TokenKind::FormType(FormType::Calculation),
                "C",
                Span {
                    start: Position::new(0, 5, 5),
                    end: Position::new(0, 6, 6),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "  ",
                Span {
                    start: Position::new(0, 6, 6),
                    end: Position::new(0, 8, 8),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "   ",
                Span {
                    start: Position::new(0, 8, 8),
                    end: Position::new(0, 11, 11),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "              ",
                Span {
                    start: Position::new(0, 11, 11),
                    end: Position::new(0, 25, 25),
                },
            ),
            Token::new(
                TokenKind::Move,
                "move",
                Span {
                    start: Position::new(0, 25, 25),
                    end: Position::new(0, 29, 29),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "      ",
                Span {
                    start: Position::new(0, 29, 29),
                    end: Position::new(0, 35, 35),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "*blanks       ",
                Span {
                    start: Position::new(0, 35, 35),
                    end: Position::new(0, 49, 49),
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "SomeResult",
                Span {
                    start: Position::new(0, 49, 49),
                    end: Position::new(0, 59, 59),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "    ",
                Span {
                    start: Position::new(0, 59, 59),
                    end: Position::new(0, 63, 63),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "     ",
                Span {
                    start: Position::new(0, 63, 63),
                    end: Position::new(0, 68, 68),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "  ",
                Span {
                    start: Position::new(0, 68, 68),
                    end: Position::new(0, 70, 70),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "      ",
                Span {
                    start: Position::new(0, 70, 70),
                    end: Position::new(0, 76, 76),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "                        ",
                Span {
                    start: Position::new(0, 76, 76),
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
    fn test_next_token_04() {
        let input = &r#"
     C                   if        foo <> bar                                                       
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
                TokenKind::FormType(FormType::Calculation),
                "C",
                Span {
                    start: Position::new(0, 5, 5),
                    end: Position::new(0, 6, 6),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "  ",
                Span {
                    start: Position::new(0, 6, 6),
                    end: Position::new(0, 8, 8),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "   ",
                Span {
                    start: Position::new(0, 8, 8),
                    end: Position::new(0, 11, 11),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "              ",
                Span {
                    start: Position::new(0, 11, 11),
                    end: Position::new(0, 25, 25),
                },
            ),
            Token::new(
                TokenKind::If,
                "if",
                Span {
                    start: Position::new(0, 25, 25),
                    end: Position::new(0, 27, 27),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "        ",
                Span {
                    start: Position::new(0, 27, 27),
                    end: Position::new(0, 35, 35),
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "foo",
                Span {
                    start: Position::new(0, 35, 35),
                    end: Position::new(0, 38, 38),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                " ",
                Span {
                    start: Position::new(0, 38, 38),
                    end: Position::new(0, 39, 39),
                },
            ),
            Token::new(
                TokenKind::NotEquals,
                "<>",
                Span {
                    start: Position::new(0, 39, 39),
                    end: Position::new(0, 41, 41),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                " ",
                Span {
                    start: Position::new(0, 41, 41),
                    end: Position::new(0, 42, 42),
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "bar",
                Span {
                    start: Position::new(0, 42, 42),
                    end: Position::new(0, 45, 45),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "    ",
                Span {
                    start: Position::new(0, 45, 45),
                    end: Position::new(0, 49, 49),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "              ",
                Span {
                    start: Position::new(0, 49, 49),
                    end: Position::new(0, 63, 63),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "     ",
                Span {
                    start: Position::new(0, 63, 63),
                    end: Position::new(0, 68, 68),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "  ",
                Span {
                    start: Position::new(0, 68, 68),
                    end: Position::new(0, 70, 70),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "      ",
                Span {
                    start: Position::new(0, 70, 70),
                    end: Position::new(0, 76, 76),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "                        ",
                Span {
                    start: Position::new(0, 76, 76),
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
