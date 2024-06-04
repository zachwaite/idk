use crate::core::{
    ch, read_char, read_until_column, read_until_end_of_line, text_at, CommentType, FormType,
    IllegalLexerState, Lexer, LexerException, Position, Span, Token, TokenKind,
};

use crate::{cspec, dspec, fspec, hspec, ispec, ospec, pspec};

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

fn read_empty_form_type(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let c = 6;
    let start = lexer.state.borrow().position;
    read_until_column(lexer, c)?;
    let end = lexer.state.borrow().position;
    let span = Span { start, end };
    let txt = text_at(lexer, span);
    if end.col == c {
        Ok(Token::new(TokenKind::FormType(FormType::Empty), &txt, span))
    } else {
        let ex = LexerException::IncompletePositionalEntry;
        let tok = Token::new(TokenKind::Idk(ex), &txt, span);
        Ok(tok)
    }
}

fn read_line_comment(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let start = lexer.state.borrow().position;
    match ch(lexer) {
        Some(' ') => {
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            let tok = Token::new(TokenKind::Comment(CommentType::NoComment), &txt, span);
            return Ok(tok);
        }
        Some('*') => {
            read_until_end_of_line(lexer)?;
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            let tok = Token::new(TokenKind::Comment(CommentType::LineComment), &txt, span);
            return Ok(tok);
        }
        _ => {
            let ex = LexerException::UnknownCommentPrefix;
            let end = lexer.state.borrow().position;
            let span = Span { start, end };
            let txt = text_at(lexer, span);
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            return Ok(tok);
        }
    }
}

fn read_newline(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let start = lexer.state.borrow().position;
    let end = Position::new(start.row, 101, start.idx + 1);
    let span = Span { start, end };
    let txt = "\n";
    let tok = Token::new(TokenKind::Eol, &txt, span);
    read_char(lexer)?; // ok - we guard in top level next_token()
    return Ok(tok);
}

pub fn next_token(lexer: &Lexer) -> Result<Token, IllegalLexerState> {
    let col = lexer.state.borrow().position.col;
    match col {
        0 => read_sequence(lexer),
        5 => match ch(lexer) {
            Some('H') => hspec::next_token(lexer),
            Some('F') => fspec::next_token(lexer),
            Some('D') => dspec::next_token(lexer),
            Some('I') => ispec::next_token(lexer),
            Some('C') => cspec::next_token(lexer),
            Some('O') => ospec::next_token(lexer),
            Some('P') => pspec::next_token(lexer),
            Some(' ') => read_empty_form_type(lexer),
            None => Ok(Token::new(TokenKind::Eof, "", Span::empty())),
            _ => Err(IllegalLexerState::ImpossibleDestination),
        },
        6 => read_line_comment(lexer),
        100 => read_newline(lexer),
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
      * create cowevt record                                                                        
      * create bornevt record, using eid from cowevt                                                
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
                TokenKind::FormType(FormType::Empty),
                " ",
                Span {
                    start: Position::new(0, 5, 5),
                    end: Position::new(0, 6, 6),
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::LineComment),
                "* create cowevt record                                                                        ",
                Span {
                    start: Position::new(0, 6, 6),
                    end: Position::new(0, 100, 100),
                },
            ),
            Token::new(
                TokenKind::Eol,
                "\n",
                Span {
                    start: Position::new(0, 100, 100),
                    end: Position::new(0, 101, 101),
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start: Position::new(1, 0, 101),
                    end: Position::new(1, 5, 106),
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::Empty),
                " ",
                Span {
                    start: Position::new(1, 5, 106),
                    end: Position::new(1, 6, 107),
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::LineComment),
                "* create bornevt record, using eid from cowevt                                                ",
                Span {
                    start: Position::new(1, 6, 107),
                    end: Position::new(1, 100, 201),
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
