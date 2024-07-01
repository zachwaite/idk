mod compiler_directive;
mod core;
mod cspec;
mod dspec;
mod free;
mod fspec;
mod full_free;
mod hspec;
mod idk;
mod ispec;
mod line_comment;
mod ospec;
mod pspec;

pub use core::{
    ch, new_lexer, peek, peek_n, read_char, IllegalLexerState, Lexer, LexerMode, Position, Span,
    Token, TokenKind, TokenMeta,
};

pub use core::{CommentType, CompilerDirectiveType, FormType}; // This feels leaky. Is there a better way to identify the sections

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
        let tok = Token::new(TokenKind::Eof, "", Span::empty());
        return Ok(tok);
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
        (_, 0, _, _, Some('*')) => LexerMode::LineComment,
        (_, 0, _, Some('H'), _) => LexerMode::HSpec,
        (_, 0, _, Some('F'), _) => LexerMode::FSpec,
        (_, 0, _, Some('D'), _) => LexerMode::DSpec,
        (_, 0, _, Some('I'), _) => LexerMode::ISpec,
        (_, 0, _, Some('C'), _) => LexerMode::CSpec,
        (_, 0, _, Some('O'), _) => LexerMode::OSpec,
        (_, 0, _, Some('P'), _) => LexerMode::PSpec,
        (_, 0, _, Some(' '), Some('/')) => LexerMode::CompilerDirective,
        (_, 0, _, Some(' '), Some(' ')) => LexerMode::Free,
        (mode, 1.., _, _, _) => mode,
        (_, _, _, _, _) => LexerMode::Idk,
    };

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
        (_, LexerMode::CompilerDirective) => compiler_directive::next_token(lexer),
        (_, LexerMode::Idk) => idk::next_token(lexer),
    };
    lexer.state.borrow_mut().mode = new_mode;
    let tok = rs?;
    Ok(tok)
}

#[cfg(test)]
mod tests {
    use crate::core::{
        new_lexer, CommentType, CompilerDirectiveType, DefinitionDataType, DefinitionType,
        FileAdditionType, FileDesignation, FileFormatType, FileSequenceType, FileType,
        LexerException, Position, TokenKind,
    };

    use super::*;

    #[test]
    fn test_next_token_e2e() {
        let input = &r#"
      * create cowevt record                                                                        
      * create bornevt record, using eid from cowevt                                                
     H OPTION(*nodebugio:*srcstmt)                                                                  
     FCowEvt    UF A E           K DISK                                                             
                                                                                                    
       *inlr = *on;                                                                                 
                                                                                                    
     F**********************************************************************************************
     D LastId          S              8  0                                                          
      /free                                                                                         
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
                TokenKind::Whitespace,
                " ",
                Span {
                    start: Position::new(2, 6, 208),
                    end: Position::new(2, 7, 209),
                },
            ),
            Token::new(
                TokenKind::Option,
                "OPTION",
                Span {
                    start: Position::new(2, 7, 209),
                    end: Position::new(2, 13, 215),
                },
            ),
            Token::new(
                TokenKind::LParen,
                "(",
                Span {
                    start: Position::new(2, 13, 215),
                    end: Position::new(2, 14, 216),
                },
            ),
            Token::new(
                TokenKind::IndicatorValue,
                "*nodebugio",
                Span {
                    start: Position::new(2, 14, 216),
                    end: Position::new(2, 24, 226),
                },
            ),
            Token::new(
                TokenKind::Colon,
                ":",
                Span {
                    start: Position::new(2, 24, 226),
                    end: Position::new(2, 25, 227),
                },
            ),
            Token::new(
                TokenKind::IndicatorValue,
                "*srcstmt",
                Span {
                    start: Position::new(2, 25, 227),
                    end: Position::new(2, 33, 235),
                },
            ),
            Token::new(
                TokenKind::RParen,
                ")",
                Span {
                    start: Position::new(2, 33, 235),
                    end: Position::new(2, 34, 236),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "                                                                  ",
                Span {
                    start: Position::new(2, 34, 236),
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
            Token::new(
                TokenKind::FormType(FormType::File),
                "F",
                Span {
                    start: Position::new(3, 5, 308),
                    end: Position::new(3, 6, 309),
                },
            ),
            Token::new(
                TokenKind::Name,
                "CowEvt    ",
                Span {
                    start: Position::new(3, 6, 309),
                    end: Position::new(3, 16, 319),
                },
            ),
            Token::new(
                TokenKind::FileType(FileType::Update),
                "U",
                Span {
                    start: Position::new(3, 16, 319),
                    end: Position::new(3, 17, 320),
                },
            ),
            Token::new(
                TokenKind::FileDesignation(FileDesignation::FullProcedural),
                "F",
                Span {
                    start: Position::new(3, 17, 320),
                    end: Position::new(3, 18, 321),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " ",
                Span {
                    start: Position::new(3, 18, 321),
                    end: Position::new(3, 19, 322),
                },
            ),
            Token::new(
                TokenKind::FileAddition(FileAdditionType::Add),
                "A",
                Span {
                    start: Position::new(3, 19, 322),
                    end: Position::new(3, 20, 323),
                },
            ),
            Token::new(
                TokenKind::FileSequence(FileSequenceType::Ascending),
                " ",
                Span {
                    start: Position::new(3, 20, 323),
                    end: Position::new(3, 21, 324),
                },
            ),
            Token::new(
                TokenKind::FileFormat(FileFormatType::ExternallyDescribed),
                "E",
                Span {
                    start: Position::new(3, 21, 324),
                    end: Position::new(3, 22, 325),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "     ",
                Span {
                    start: Position::new(3, 22, 325),
                    end: Position::new(3, 27, 330),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " ",
                Span {
                    start: Position::new(3, 27, 330),
                    end: Position::new(3, 28, 331),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "     ",
                Span {
                    start: Position::new(3, 28, 331),
                    end: Position::new(3, 33, 336),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "K",
                Span {
                    start: Position::new(3, 33, 336),
                    end: Position::new(3, 34, 337),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " ",
                Span {
                    start: Position::new(3, 34, 337),
                    end: Position::new(3, 35, 338),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "DISK   ",
                Span {
                    start: Position::new(3, 35, 338),
                    end: Position::new(3, 42, 345),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " ",
                Span {
                    start: Position::new(3, 42, 345),
                    end: Position::new(3, 43, 346),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "                                                         ",
                Span {
                    start: Position::new(3, 43, 346),
                    end: Position::new(3, 100, 403),
                },
            ),
            Token::new(
                TokenKind::Eol,
                "\n",
                Span {
                    start: Position::new(3, 100, 403),
                    end: Position::new(3, 101, 404),
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start: Position::new(4, 0, 404),
                    end: Position::new(4, 5, 409),
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::Empty),
                " ",
                Span {
                    start: Position::new(4, 5, 409),
                    end: Position::new(4, 6, 410),
                },
            ),
            Token::new(
                TokenKind::Reserved,
                " ",
                Span {
                    start: Position::new(4, 6, 410),
                    end: Position::new(4, 7, 411),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "                                                                                             ",
                Span {
                    start: Position::new(4, 7, 411),
                    end: Position::new(4, 100, 504),
                },
            ),
            Token::new(
                TokenKind::Eol,
                "\n",
                Span {
                    start: Position::new(4, 100, 504),
                    end: Position::new(4, 101, 505),
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start: Position::new(5, 0, 505),
                    end: Position::new(5, 5, 510),
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::Empty),
                " ",
                Span {
                    start: Position::new(5, 5, 510),
                    end: Position::new(5, 6, 511),
                },
            ),
            Token::new(
                TokenKind::Reserved,
                " ",
                Span {
                    start: Position::new(5, 6, 511),
                    end: Position::new(5, 7, 512),
                },
            ),
            Token::new(
                TokenKind::Indicator,
                "*inlr",
                Span {
                    start: Position::new(5, 7, 512),
                    end: Position::new(5, 12, 517),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                " ",
                Span {
                    start: Position::new(5, 12, 517),
                    end: Position::new(5, 13, 518),
                },
            ),
            Token::new(
                TokenKind::Equals,
                "=",
                Span {
                    start: Position::new(5, 13, 518),
                    end: Position::new(5, 14, 519),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                " ",
                Span {
                    start: Position::new(5, 14, 519),
                    end: Position::new(5, 15, 520),
                },
            ),
            Token::new(
                TokenKind::IndicatorValue,
                "*on",
                Span {
                    start: Position::new(5, 15, 520),
                    end: Position::new(5, 18, 523),
                },
            ),
            Token::new(
                TokenKind::Semicolon,
                ";",
                Span {
                    start: Position::new(5, 18, 523),
                    end: Position::new(5, 19, 524),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "                                                                                 ",
                Span {
                    start: Position::new(5, 19, 524),
                    end: Position::new(5, 100, 605),
                },
            ),
            Token::new(
                TokenKind::Eol,
                "\n",
                Span {
                    start: Position::new(5, 100, 605),
                    end: Position::new(5, 101, 606),
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start: Position::new(6, 0, 606),
                    end: Position::new(6, 5, 611),
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::Empty),
                " ",
                Span {
                    start: Position::new(6, 5, 611),
                    end: Position::new(6, 6, 612),
                },
            ),
            Token::new(
                TokenKind::Reserved,
                " ",
                Span {
                    start: Position::new(6, 6, 612),
                    end: Position::new(6, 7, 613),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "                                                                                             ",
                Span {
                    start: Position::new(6, 7, 613),
                    end: Position::new(6, 100, 706),
                },
            ),
            Token::new(
                TokenKind::Eol,
                "\n",
                Span {
                    start: Position::new(6, 100, 706),
                    end: Position::new(6, 101, 707),
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start: Position::new(7, 0, 707),
                    end: Position::new(7, 5, 712),
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::File),
                "F",
                Span {
                    start: Position::new(7, 5, 712),
                    end: Position::new(7, 6, 713),
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::LineComment),
                "**********************************************************************************************",
                Span {
                    start: Position::new(7, 6, 713),
                    end: Position::new(7, 100, 807),
                },
            ),
            Token::new(
                TokenKind::Eol,
                "\n",
                Span {
                    start: Position::new(7, 100, 807),
                    end: Position::new(7, 101, 808),
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start: Position::new(8, 0, 808),
                    end: Position::new(8, 5, 813),
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::Definition),
                "D",
                Span {
                    start: Position::new(8, 5, 813),
                    end: Position::new(8, 6, 814),
                },
            ),
            Token::new(
                TokenKind::Name,
                " LastId        ",
                Span {
                    start: Position::new(8, 6, 814),
                    end: Position::new(8, 21, 829),
                },
            ),
            Token::new(
                TokenKind::FileFormat(FileFormatType::ProgramDescribed),
                " ",
                Span {
                    start: Position::new(8, 21, 829),
                    end: Position::new(8, 22, 830),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " ",
                Span {
                    start: Position::new(8, 22, 830),
                    end: Position::new(8, 23, 831),
                },
            ),
            Token::new(
                TokenKind::DefinitionType(DefinitionType::Standalone),
                "S ",
                Span {
                    start: Position::new(8, 23, 831),
                    end: Position::new(8, 25, 833),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "       ",
                Span {
                    start: Position::new(8, 25, 833),
                    end: Position::new(8, 32, 840),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "      ",
                Span {
                    start: Position::new(8, 32, 840),
                    end: Position::new(8, 38, 846),
                },
            ),
            Token::new(
                TokenKind::Number,
                "8",
                Span {
                    start: Position::new(8, 38, 846),
                    end: Position::new(8, 39, 847),
                },
            ),
            Token::new(
                TokenKind::DefinitionDataType(DefinitionDataType::Blank),
                " ",
                Span {
                    start: Position::new(8, 39, 847),
                    end: Position::new(8, 40, 848),
                },
            ),
            Token::new(
                TokenKind::DefinitionDecimals,
                " 0",
                Span {
                    start: Position::new(8, 40, 848),
                    end: Position::new(8, 42, 850),
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                " ",
                Span {
                    start: Position::new(8, 42, 850),
                    end: Position::new(8, 43, 851),
                },
            ),
            Token::new(
                TokenKind::Whitespace,
                "                                                         ",
                Span {
                    start: Position::new(8, 43, 851),
                    end: Position::new(8, 100, 908),
                },
            ),
            Token::new(
                TokenKind::Eol,
                "\n",
                Span {
                    start: Position::new(8, 100, 908),
                    end: Position::new(8, 101, 909),
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start: Position::new(9, 0, 909),
                    end: Position::new(9, 5, 914),
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::Empty),
                " ",
                Span {
                    start: Position::new(9, 5, 914),
                    end: Position::new(9, 6, 915),
                },
            ),
            Token::new(
                TokenKind::CompilerDirectiveType(CompilerDirectiveType::Free),
                "/free                                                                                         ",
                Span {
                    start: Position::new(9, 6, 915),
                    end: Position::new(9, 100, 1009),
                },
            ),
            Token::new(
                TokenKind::Eol,
                "\n",
                Span {
                    start: Position::new(9, 100, 1009),
                    end: Position::new(9, 101, 1010),
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start: Position::new(10, 0, 1010),
                    end: Position::new(10, 5, 1015),
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::Empty),
                " ",
                Span {
                    start: Position::new(10, 5, 1015),
                    end: Position::new(10, 6, 1016),
                },
            ),
            Token::new(
                TokenKind::Reserved,
                " ",
                Span {
                    start: Position::new(10, 6, 1016),
                    end: Position::new(10, 7, 1017),
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::InlineComment),
                "// Look up LastId                                                                            ",
                Span {
                    start: Position::new(10, 7, 1017),
                    end: Position::new(10, 100, 1110),
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
