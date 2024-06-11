use crate::{
    core::{
        ch, is_identifier_char, is_numeric, read_identifier, read_number, read_spaces_or_tabs,
        read_string_literal, read_until_column, read_until_end_of_line, text_at,
        DefinitionDataType, DefinitionType, FileFormatType, FormType, IllegalLexerState, Lexer,
        LexerException, Span, Token, TokenKind,
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
        (6, Some('D')) => Ok(Token::new(
            TokenKind::FormType(FormType::Definition),
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

fn read_name(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 21;
    let start = lexer.state.borrow().position;
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    if end.col == c {
        Ok(Token::new(TokenKind::Name, &txt, span))
    } else {
        let ex = LexerException::IncompletePositionalEntry;
        let tok = Token::new(TokenKind::Idk(ex), &txt, span);
        Ok(tok)
    }
}

fn read_file_format(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 22;
    let start = lexer.state.borrow().position;
    let maybe = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    match (end.col, maybe) {
        (22, Some(' ')) => Ok(Token::new(
            TokenKind::FileFormat(FileFormatType::ProgramDescribed),
            &txt,
            span,
        )),
        (22, Some('E') | Some('e')) => Ok(Token::new(
            TokenKind::FileFormat(FileFormatType::ExternallyDescribed),
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

fn read_data_structure_type(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 23;
    let start = lexer.state.borrow().position;
    let maybe = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match (end.col, maybe) {
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_definition_type(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let start = lexer.state.borrow().position;
    let kind = match ch(lexer) {
        Some(' ') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some(' ') => TokenKind::DefinitionType(DefinitionType::Blank),
                _ => TokenKind::DefinitionType(DefinitionType::Blank),
            }
        }
        Some('C') | Some('c') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some(' ') => TokenKind::DefinitionType(DefinitionType::Constant),
                _ => TokenKind::Idk(LexerException::NotImplemented),
            }
        }
        Some('D') | Some('d') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some('D') => TokenKind::DefinitionType(DefinitionType::DataStructure),
                _ => TokenKind::Idk(LexerException::NotImplemented),
            }
        }
        Some('P') | Some('p') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some('R') | Some('r') => TokenKind::DefinitionType(DefinitionType::Prototype),
                Some('I') | Some('i') => {
                    TokenKind::DefinitionType(DefinitionType::ProcedureInterface)
                }
                _ => TokenKind::Idk(LexerException::NotImplemented),
            }
        }
        Some('S') | Some('s') => {
            read_char(lexer)?;
            match ch(lexer) {
                Some(' ') => TokenKind::DefinitionType(DefinitionType::Standalone),
                _ => TokenKind::Idk(LexerException::NotImplemented),
            }
        }
        _ => TokenKind::Idk(LexerException::NotImplemented),
    };
    // take one more to stop on 25
    read_char(lexer)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    let tok = Token::new(kind, &txt, span);
    Ok(tok)
}

fn read_from_position(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 32;
    let start = lexer.state.borrow().position;
    let maybe = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match (end.col, maybe) {
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_to_position_length(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 39;
    let start = lexer.state.borrow().position;
    let kind = match (start.col, ch(lexer)) {
        (32..=38, Some(' ')) => {
            read_spaces_or_tabs(lexer)?;
            TokenKind::Whitespace
        }
        (32..=38, Some(x)) => match is_numeric(&x) {
            true => {
                read_number(lexer)?;
                TokenKind::Number
            }
            false => {
                read_until_column(lexer, c)?;
                TokenKind::Idk(LexerException::NotImplemented)
            }
        },
        (_, None) => TokenKind::Eof,
        (_, _) => {
            read_until_column(lexer, c)?;
            TokenKind::Idk(LexerException::NotImplemented)
        }
    };
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    let tok = Token::new(kind, &txt, span);
    Ok(tok)
}

fn read_data_type(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let start = lexer.state.borrow().position;
    let maybe = ch(lexer);
    let kind = match maybe {
        Some(' ') => TokenKind::DefinitionDataType(DefinitionDataType::Blank),
        Some('A') | Some('a') => TokenKind::DefinitionDataType(DefinitionDataType::Character),
        Some('B') | Some('b') => TokenKind::DefinitionDataType(DefinitionDataType::Binary),
        Some('C') | Some('c') => TokenKind::DefinitionDataType(DefinitionDataType::UCS2),
        Some('D') | Some('d') => TokenKind::DefinitionDataType(DefinitionDataType::Date),
        Some('F') | Some('f') => TokenKind::DefinitionDataType(DefinitionDataType::Float),
        Some('G') | Some('g') => TokenKind::DefinitionDataType(DefinitionDataType::Graphic),
        Some('I') | Some('i') => TokenKind::DefinitionDataType(DefinitionDataType::Integer),
        Some('N') | Some('n') => TokenKind::DefinitionDataType(DefinitionDataType::Indicator),
        Some('O') | Some('o') => TokenKind::DefinitionDataType(DefinitionDataType::Object),
        Some('P') | Some('p') => TokenKind::DefinitionDataType(DefinitionDataType::Packed),
        Some('S') | Some('s') => TokenKind::DefinitionDataType(DefinitionDataType::Zoned),
        Some('T') | Some('t') => TokenKind::DefinitionDataType(DefinitionDataType::Time),
        Some('U') | Some('u') => TokenKind::DefinitionDataType(DefinitionDataType::Unsigned),
        Some('Z') | Some('z') => TokenKind::DefinitionDataType(DefinitionDataType::Timestamp),
        Some('*') => TokenKind::DefinitionDataType(DefinitionDataType::Pointer),
        _ => TokenKind::Idk(LexerException::NotImplemented),
    };
    read_char(lexer)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    let tok = Token::new(kind, &txt, span);
    Ok(tok)
}

fn read_decimals(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 42;
    let start = lexer.state.borrow().position;
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match end.col {
        42 => {
            let tok = Token::new(TokenKind::DefinitionDecimals, &txt, span);
            Ok(tok)
        }
        _ => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_reserved(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 43;
    let start = lexer.state.borrow().position;
    let maybe = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match (end.col, maybe) {
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
                        "EXTPGM" => TokenKind::Extpgm,
                        "DIM" => TokenKind::Dim,
                        _ => TokenKind::Identifier,
                    }
                }
            },
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
        6 => read_name(lexer),
        21 => read_file_format(lexer),
        22 => read_data_structure_type(lexer),
        23 => read_definition_type(lexer),
        25 => read_from_position(lexer),
        32..=38 => read_to_position_length(lexer),
        39 => read_data_type(lexer),
        40 => read_decimals(lexer),
        42 => read_reserved(lexer),
        43.. => read_keywords(lexer),
        _ => Err(IllegalLexerState::ImpossibleDestination),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{new_lexer, Position, TokenKind};

    use super::*;

    #[test]
    fn test_next_token() {
        let input = &r#"
     D LastId          S              8  0                                                          
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
                TokenKind::FormType(FormType::Definition),
                "D",
                Span {
                    start: Position::new(0, 5, 5),
                    end: Position::new(0, 6, 6),
                },
            ),
            Token::new(
                TokenKind::Name,
                " LastId        ",
                Span {
                    start: Position::new(0, 6, 6),
                    end: Position::new(0, 21, 21),
                },
            ),
            Token::new(
                TokenKind::FileFormat(FileFormatType::ProgramDescribed),
                " ",
                Span {
                    start: Position::new(0, 21, 21),
                    end: Position::new(0, 22, 22),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " ",
                Span {
                    start: Position::new(0, 22, 22),
                    end: Position::new(0, 23, 23),
                },
            ),
            Token::new(
                TokenKind::DefinitionType(DefinitionType::Standalone),
                "S ",
                Span {
                    start: Position::new(0, 23, 23),
                    end: Position::new(0, 25, 25),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "       ",
                Span {
                    start: Position::new(0, 25, 25),
                    end: Position::new(0, 32, 32),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "      ",
                Span {
                    start: Position::new(0, 32, 32),
                    end: Position::new(0, 38, 38),
                },
            ),
            Token::new(
                TokenKind::Number,
                "8",
                Span {
                    start: Position::new(0, 38, 38),
                    end: Position::new(0, 39, 39),
                },
            ),
            Token::new(
                TokenKind::DefinitionDataType(DefinitionDataType::Blank),
                " ",
                Span {
                    start: Position::new(0, 39, 39),
                    end: Position::new(0, 40, 40),
                },
            ),
            Token::new(
                TokenKind::DefinitionDecimals,
                " 0",
                Span {
                    start: Position::new(0, 40, 40),
                    end: Position::new(0, 42, 42),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " ",
                Span {
                    start: Position::new(0, 42, 42),
                    end: Position::new(0, 43, 43),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "                                                         ",
                Span {
                    start: Position::new(0, 43, 43),
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
