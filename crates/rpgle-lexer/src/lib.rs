mod core;
mod cspec;
mod dspec;
mod free;
mod fspec;
mod full_free;
mod hspec;
mod ispec;
mod line_comment;
mod ospec;
mod pspec;

pub use core::{
    ch, new_lexer, peek, peek_n, read_char, IllegalLexerState, Lexer, LexerMode, Position, Span,
    Token, TokenKind,
};

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
    // guard for eof
    if peek(lexer).is_none() {
        return Ok(Token::new(TokenKind::Eof, "", Span::empty()));
    }

    // manage mode
    let mode = lexer.state.borrow().mode;
    let col = lexer.state.borrow().position.col;
    let cur = ch(&lexer);
    let peek5 = peek_n(&lexer, 5);
    let peek6 = peek_n(&lexer, 6);
    let table = (mode, col, cur, peek5, peek6);
    let new_mode = match table {
        (LexerMode::Init, 0, Some('*'), _, _) => LexerMode::FullFree,
        (_, 0, _, Some('H'), _) => LexerMode::HSpec,
        (_, 0, _, Some('F'), _) => LexerMode::FSpec,
        (_, 0, _, Some('D'), _) => LexerMode::DSpec,
        (_, 0, _, Some('I'), _) => LexerMode::ISpec,
        (_, 0, _, Some('C'), _) => LexerMode::CSpec,
        (_, 0, _, Some('O'), _) => LexerMode::OSpec,
        (_, 0, _, Some('P'), _) => LexerMode::PSpec,
        (_, 0, _, Some(' '), Some('*')) => LexerMode::LineComment,
        (mode, 1.., _, _, _) => mode,
        (a, b, c, d, e) => {
            let msg = format!(
                "{}, {}, {}, {}, {}",
                a,
                b,
                c.unwrap(),
                d.unwrap(),
                e.unwrap()
            );
            println!("{}", msg);
            LexerMode::Init
        }
    };
    println!("{}", new_mode);

    // dispatch lexer
    let rs = match (col, new_mode) {
        (100, _) => read_newline(lexer),
        (_, LexerMode::Init) => Err(IllegalLexerState::ImpossibleDestination),
        (_, LexerMode::LineComment) => line_comment::next_token(lexer),
        (_, LexerMode::FullFree) => full_free::next_token(lexer),
        (_, LexerMode::Free) => free::next_token(lexer),
        (_, LexerMode::HSpec) => hspec::next_token(lexer),
        (_, LexerMode::FSpec) => fspec::next_token(lexer),
        (_, LexerMode::DSpec) => dspec::next_token(lexer),
        (_, LexerMode::ISpec) => ispec::next_token(lexer),
        (_, LexerMode::CSpec) => cspec::next_token(lexer),
        (_, LexerMode::OSpec) => ospec::next_token(lexer),
        (_, LexerMode::PSpec) => pspec::next_token(lexer),
    };
    lexer.state.borrow_mut().mode = new_mode;
    rs
}

#[cfg(test)]
mod tests {
    use crate::core::{
        new_lexer, CommentType, FileAdditionType, FileDesignation, FileFormatType,
        FileSequenceType, FileType, FormType, LexerException, Position, TokenKind,
    };

    use super::*;

    #[test]
    fn test_next_token_e2e() {
        let input = &r#"
      * create cowevt record                                                                        
      * create bornevt record, using eid from cowevt                                                
     H OPTION(*nodebugio:*srcstmt)                                                                  
     FCowEvt    UF A E           K DISK                                                             
     FBornEvt   UF A E           K DISK                                                             
     FCowEvtL2  IF   E           K DISK     Rename(EVTFMT:VEVTFMT) Prefix(V)                        
     F**********************************************************************************************
     D**********************************************************************************************
     D LastId          S              8  0                                                          
     C**********************************************************************************************
      /free                                                                                         
       // Look up LastId                                                                            
       // Since CowEvtL1 is sorted by id descending,                                                
       // the `id` of the first row would be the LastId                                             
       SetLL *Loval CowEvtL2;                                                                       
       If Not %Eof;                                                                                 
         Read CowEvtL2;                                                                             
         LastId = Vid;                                                                              
       Else;                                                                                        
        LastId = 1;                                                                                 
       Endif;                                                                                       
                                                                                                    
       // create the new cowevt                                                                     
       Id = LastId + 1;                                                                             
       Edat = 20240101;                                                                             
       Etim = 125959;                                                                               
       Etyp = 'BORN';                                                                               
       Write EVTFMT;                                                                                
                                                                                                    
       // create the related bornevt                                                                
       EID = Id;                                                                                    
       BNAME = 'BESSE';                                                                             
       BDAT = 20240101;                                                                             
       Write BORNFMT;                                                                               
                                                                                                    
       *inlr = *on;                                                                                 
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
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
            Token::new(
                TokenKind::Eol,
                "\n",
                Span {
                    start: Position::new(1, 100, 201),
                    end: Position::new(1, 101, 202),
                },
            ),
            // hspec
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start: Position::new(2, 0, 202),
                    end: Position::new(2, 5, 207),
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::Control),
                "H",
                Span {
                    start: Position::new(2, 5, 207),
                    end: Position::new(2, 6, 208),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " OPTION(*nodebugio:*srcstmt)                                                                  ",
                Span {
                    start: Position::new(2, 6, 208),
                    end: Position::new(2, 100, 302),
                },
            ),
            Token::new(
                TokenKind::Eol,
                "\n",
                Span {
                    start: Position::new(2, 100, 302),
                    end: Position::new(2, 101, 303),
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start: Position::new(3, 0, 303),
                    end: Position::new(3, 5, 308),
                },
            ),
            // Token::new(
            //     TokenKind::FormType(FormType::Control),
            //     "F",
            //     Span {
            //         start: Position::new(0, 5, 5),
            //         end: Position::new(0, 6, 6),
            //     },
            // ),
            // Token::new(
            //     TokenKind::Name,
            //     "CowEvt    ",
            //     Span {
            //         start: Position::new(0, 6, 6),
            //         end: Position::new(0, 16, 16),
            //     },
            // ),
            // Token::new(
            //     TokenKind::FileType(FileType::Update),
            //     "U",
            //     Span {
            //         start: Position::new(0, 16, 16),
            //         end: Position::new(0, 17, 17),
            //     },
            // ),
            // Token::new(
            //     TokenKind::FileDesignation(FileDesignation::FullProcedural),
            //     "F",
            //     Span {
            //         start: Position::new(0, 17, 17),
            //         end: Position::new(0, 18, 18),
            //     },
            // ),
            // Token::new(
            //     TokenKind::Idk(LexerException::NotImplemented),
            //     " ",
            //     Span {
            //         start: Position::new(0, 18, 18),
            //         end: Position::new(0, 19, 19),
            //     },
            // ),
            // Token::new(
            //     TokenKind::FileAddition(FileAdditionType::Add),
            //     "A",
            //     Span {
            //         start: Position::new(0, 19, 19),
            //         end: Position::new(0, 20, 20),
            //     },
            // ),
            // Token::new(
            //     TokenKind::FileSequence(FileSequenceType::Ascending),
            //     " ",
            //     Span {
            //         start: Position::new(0, 20, 20),
            //         end: Position::new(0, 21, 21),
            //     },
            // ),
            // Token::new(
            //     TokenKind::FileFormat(FileFormatType::ExternallyDescribed),
            //     "E",
            //     Span {
            //         start: Position::new(0, 21, 21),
            //         end: Position::new(0, 22, 22),
            //     },
            // ),
            // Token::new(
            //     TokenKind::Idk(LexerException::NotImplemented),
            //     "     ",
            //     Span {
            //         start: Position::new(0, 22, 22),
            //         end: Position::new(0, 27, 27),
            //     },
            // ),
            // Token::new(
            //     TokenKind::Idk(LexerException::NotImplemented),
            //     " ",
            //     Span {
            //         start: Position::new(0, 27, 27),
            //         end: Position::new(0, 28, 28),
            //     },
            // ),
            // Token::new(
            //     TokenKind::Idk(LexerException::NotImplemented),
            //     "     ",
            //     Span {
            //         start: Position::new(0, 28, 28),
            //         end: Position::new(0, 33, 33),
            //     },
            // ),
            // Token::new(
            //     TokenKind::Idk(LexerException::NotImplemented),
            //     "K",
            //     Span {
            //         start: Position::new(0, 33, 33),
            //         end: Position::new(0, 34, 34),
            //     },
            // ),
            // Token::new(
            //     TokenKind::Idk(LexerException::NotImplemented),
            //     " ",
            //     Span {
            //         start: Position::new(0, 34, 34),
            //         end: Position::new(0, 35, 35),
            //     },
            // ),
            // Token::new(
            //     TokenKind::Idk(LexerException::NotImplemented),
            //     "DISK   ",
            //     Span {
            //         start: Position::new(0, 35, 35),
            //         end: Position::new(0, 42, 42),
            //     },
            // ),
            // Token::new(
            //     TokenKind::Idk(LexerException::NotImplemented),
            //     " ",
            //     Span {
            //         start: Position::new(0, 42, 42),
            //         end: Position::new(0, 43, 43),
            //     },
            // ),
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
