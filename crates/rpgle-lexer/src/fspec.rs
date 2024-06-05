use crate::{
    core::{
        ch, read_until_column, read_until_end_of_line, text_at, FileAdditionType, FileDesignation,
        FileFormatType, FileSequenceType, FileType, FormType, IllegalLexerState, Lexer,
        LexerException, Span, Token, TokenKind,
    },
    LexerMode,
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
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    match (end.col, maybe_f) {
        (6, Some('F')) => Ok(Token::new(
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

fn read_name(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 16;
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

fn read_file_type(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 17;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    match (end.col, maybe_f) {
        (17, Some('I')) => Ok(Token::new(TokenKind::FileType(FileType::Input), &txt, span)),
        (17, Some('O')) => Ok(Token::new(
            TokenKind::FileType(FileType::Output),
            &txt,
            span,
        )),
        (17, Some('U')) => Ok(Token::new(
            TokenKind::FileType(FileType::Update),
            &txt,
            span,
        )),
        (17, Some('C')) => Ok(Token::new(
            TokenKind::FileType(FileType::Combined),
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

fn read_file_designation(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 18;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    match (end.col, maybe_f) {
        (18, Some(' ')) => Ok(Token::new(
            TokenKind::FileDesignation(FileDesignation::Output),
            &txt,
            span,
        )),
        (18, Some('P')) => Ok(Token::new(
            TokenKind::FileDesignation(FileDesignation::Primary),
            &txt,
            span,
        )),
        (18, Some('S')) => Ok(Token::new(
            TokenKind::FileDesignation(FileDesignation::Secondary),
            &txt,
            span,
        )),
        (18, Some('R')) => Ok(Token::new(
            TokenKind::FileDesignation(FileDesignation::RecordAddress),
            &txt,
            span,
        )),
        (18, Some('T')) => Ok(Token::new(
            TokenKind::FileDesignation(FileDesignation::Table),
            &txt,
            span,
        )),
        (18, Some('F')) => Ok(Token::new(
            TokenKind::FileDesignation(FileDesignation::FullProcedural),
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

fn read_end_of_file(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 19;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match (end.col, maybe_f) {
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_file_addition(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 20;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    match (end.col, maybe_f) {
        (20, Some('A')) => Ok(Token::new(
            TokenKind::FileAddition(FileAdditionType::Add),
            &txt,
            span,
        )),
        (20, Some(' ')) => Ok(Token::new(
            TokenKind::FileAddition(FileAdditionType::NoAdd),
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

fn read_file_sequence(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 21;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    match (end.col, maybe_f) {
        (21, Some(' ')) => Ok(Token::new(
            TokenKind::FileSequence(FileSequenceType::Ascending),
            &txt,
            span,
        )),
        (21, Some('A')) => Ok(Token::new(
            TokenKind::FileSequence(FileSequenceType::Ascending),
            &txt,
            span,
        )),
        (21, Some('D')) => Ok(Token::new(
            TokenKind::FileSequence(FileSequenceType::Descending),
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

fn read_file_format(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 22;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    match (end.col, maybe_f) {
        (22, Some('F')) => Ok(Token::new(
            TokenKind::FileFormat(FileFormatType::ProgramDescribed),
            &txt,
            span,
        )),
        (22, Some('E')) => Ok(Token::new(
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

fn read_record_length(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 27;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match (end.col, maybe_f) {
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_limits_processing(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 28;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match (end.col, maybe_f) {
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_record_address_length(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 33;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match (end.col, maybe_f) {
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_record_address_type(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 34;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match (end.col, maybe_f) {
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_file_organization(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 35;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match (end.col, maybe_f) {
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_device(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 42;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match (end.col, maybe_f) {
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_reserved(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 43;
    let start = lexer.state.borrow().position;
    let maybe_f = ch(lexer);
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    // TDE
    match (end.col, maybe_f) {
        (_, _) => {
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }
}

fn read_keywords(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let start = lexer.state.borrow().position;
    read_until_end_of_line(lexer)?;
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
        6 => read_name(lexer),
        16 => read_file_type(lexer),
        17 => read_file_designation(lexer),
        18 => read_end_of_file(lexer),
        19 => read_file_addition(lexer),
        20 => read_file_sequence(lexer),
        21 => read_file_format(lexer),
        22 => read_record_length(lexer),
        27 => read_limits_processing(lexer),
        28 => read_record_address_length(lexer),
        33 => read_record_address_type(lexer),
        34 => read_file_organization(lexer),
        35 => read_device(lexer),
        42 => read_reserved(lexer),
        43 => read_keywords(lexer),
        _ => Err(IllegalLexerState::ImpossibleDestination),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{new_lexer, Position, TokenKind};

    use super::*;

    // FBornEvt   UF A E           K DISK
    // FCowEvtL2  IF   E           K DISK     Rename(EVTFMT:VEVTFMT) Prefix(V)
    // F**********************************************************************************************
    // D**********************************************************************************************
    // D LastId          S              8  0
    // C**********************************************************************************************

    #[test]
    fn test_next_token() {
        let input = &r#"
     FCowEvt    UF A E           K DISK                                                             
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
                "F",
                Span {
                    start: Position::new(0, 5, 5),
                    end: Position::new(0, 6, 6),
                },
            ),
            Token::new(
                TokenKind::Name,
                "CowEvt    ",
                Span {
                    start: Position::new(0, 6, 6),
                    end: Position::new(0, 16, 16),
                },
            ),
            Token::new(
                TokenKind::FileType(FileType::Update),
                "U",
                Span {
                    start: Position::new(0, 16, 16),
                    end: Position::new(0, 17, 17),
                },
            ),
            Token::new(
                TokenKind::FileDesignation(FileDesignation::FullProcedural),
                "F",
                Span {
                    start: Position::new(0, 17, 17),
                    end: Position::new(0, 18, 18),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " ",
                Span {
                    start: Position::new(0, 18, 18),
                    end: Position::new(0, 19, 19),
                },
            ),
            Token::new(
                TokenKind::FileAddition(FileAdditionType::Add),
                "A",
                Span {
                    start: Position::new(0, 19, 19),
                    end: Position::new(0, 20, 20),
                },
            ),
            Token::new(
                TokenKind::FileSequence(FileSequenceType::Ascending),
                " ",
                Span {
                    start: Position::new(0, 20, 20),
                    end: Position::new(0, 21, 21),
                },
            ),
            Token::new(
                TokenKind::FileFormat(FileFormatType::ExternallyDescribed),
                "E",
                Span {
                    start: Position::new(0, 21, 21),
                    end: Position::new(0, 22, 22),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "     ",
                Span {
                    start: Position::new(0, 22, 22),
                    end: Position::new(0, 27, 27),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " ",
                Span {
                    start: Position::new(0, 27, 27),
                    end: Position::new(0, 28, 28),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "     ",
                Span {
                    start: Position::new(0, 28, 28),
                    end: Position::new(0, 33, 33),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "K",
                Span {
                    start: Position::new(0, 33, 33),
                    end: Position::new(0, 34, 34),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " ",
                Span {
                    start: Position::new(0, 34, 34),
                    end: Position::new(0, 35, 35),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "DISK   ",
                Span {
                    start: Position::new(0, 35, 35),
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
