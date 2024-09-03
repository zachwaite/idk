mod highlight;
use highlight::{highlight_all, HighlightMeta};
use nvim_oxi::{self as oxi};
use rpgle_lexer::{new_lexer, next_token, CompilerDirectiveType, FormType, Token, TokenKind};
use std::env;
use std::fmt;

struct SpecState {
    control: usize,
    file: usize,
    definition: usize,
    input: usize,
    calculation: usize,
    output: usize,
}

impl fmt::Display for SpecState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!(
            "SpecState(H: {}, F: {}, D: {}, C: {})",
            self.get_control(),
            self.get_file(),
            self.get_definition(),
            self.get_calculation()
        );
        write!(f, "{}", out)
    }
}

impl SpecState {
    fn new() -> Self {
        Self {
            control: 99999999,
            file: 99999999,
            definition: 99999999,
            input: 99999999,
            calculation: 99999999,
            output: 99999999,
        }
    }

    fn get_control(&self) -> usize {
        self.control + 1
    }

    fn get_file(&self) -> usize {
        self.file + 1
    }

    fn get_definition(&self) -> usize {
        self.definition + 1
    }

    fn get_input(&self) -> usize {
        self.input + 1
    }

    fn get_calculation(&self) -> usize {
        self.calculation + 1
    }

    fn get_output(&self) -> usize {
        self.output + 1
    }

    fn evolve(&mut self, token: &Token) {
        let kind = token.kind;
        let row = token.span.start.row;
        match kind {
            TokenKind::FormType(FormType::Control) => {
                if row < self.control {
                    self.control = row;
                }
            }
            TokenKind::FormType(FormType::File) => {
                if row < self.file {
                    self.file = row;
                }
            }
            TokenKind::FormType(FormType::Definition) => {
                if row < self.definition {
                    self.definition = row;
                }
            }
            TokenKind::FormType(FormType::Input) => {
                if row < self.input {
                    self.input = row;
                }
            }
            TokenKind::FormType(FormType::Calculation)
            | TokenKind::CompilerDirectiveType(CompilerDirectiveType::Free) => {
                if row < self.calculation {
                    self.calculation = row;
                }
            }
            TokenKind::FormType(FormType::Output) => {
                if row < self.output {
                    self.output = row;
                }
            }
            _ => {
                // pass
            }
        }
    }
}

fn get_hl_group(kind: &TokenKind) -> String {
    match kind {
        TokenKind::FullFree | TokenKind::Free | TokenKind::EndFree => {
            "@keyword.directive".to_string()
        }
        TokenKind::Comment(_) => "@comment".to_string(),
        TokenKind::Name => "Identifier".to_string(),
        TokenKind::FileType(_) => "@keyword.storage".to_string(),
        TokenKind::FileDesignation(_) => "@keyword.directive".to_string(),
        TokenKind::Option
        | TokenKind::Datedit
        | TokenKind::Datfmt
        | TokenKind::Timfmt
        | TokenKind::Dftactgrp
        | TokenKind::Debug
        | TokenKind::Rename
        | TokenKind::Ignore
        | TokenKind::Prefix
        | TokenKind::Extpgm
        | TokenKind::Dim => "@keyword.directive".to_string(),
        TokenKind::DefinitionType(_) => "@type.definition".to_string(),
        TokenKind::DefinitionDataType(_) => "@type.qualifier".to_string(),
        TokenKind::DefinitionDecimals => "@number".to_string(),
        TokenKind::CompilerDirectiveType(_) => "@keyword.directive.define".to_string(),
        TokenKind::Indicator => "@variable.builtin".to_string(),
        TokenKind::IndicatorValue => "@variable.parameter.builtin".to_string(),
        TokenKind::SetLL
        | TokenKind::SetGT
        | TokenKind::Chain
        | TokenKind::Read
        | TokenKind::ReadE
        | TokenKind::ReadPE
        | TokenKind::Write
        | TokenKind::Update
        | TokenKind::Delete
        | TokenKind::If
        | TokenKind::Else
        | TokenKind::Elseif
        | TokenKind::Endif
        | TokenKind::Dou
        | TokenKind::Dow
        | TokenKind::Iter
        | TokenKind::Leave
        | TokenKind::Reset
        | TokenKind::Eval
        | TokenKind::Clear
        | TokenKind::Enddo
        | TokenKind::Begsr
        | TokenKind::Endsr
        | TokenKind::Exsr => "@function.builtin".to_string(),
        TokenKind::Number => "@number".to_string(),
        TokenKind::Identifier => "Identifier".to_string(),
        TokenKind::BuiltinIdentifier => "@function.builtin".to_string(),
        TokenKind::StringLiteral => "String".to_string(),
        TokenKind::LessThan
        | TokenKind::LessThanOrEquals
        | TokenKind::GreaterThan
        | TokenKind::GreaterThanOrEquals
        | TokenKind::And
        | TokenKind::Or
        | TokenKind::NotEquals
        | TokenKind::Equals
        | TokenKind::Plus
        | TokenKind::PlusEqual
        | TokenKind::Minus
        | TokenKind::MinusEqual
        | TokenKind::Asterisk
        | TokenKind::AsteriskEqual
        | TokenKind::Slash
        | TokenKind::SlashEqual => "@operator".to_string(),
        _ => "Normal".to_string(),
    }
}

struct Highlighter {
    buf: oxi::api::Buffer,
    namespace_id: u32,
}

impl Highlighter {
    fn highlight(&mut self, meta: &HighlightMeta) -> oxi::Result<()> {
        // remove conflicting existing mark
        let opts = oxi::api::opts::GetExtmarksOpts::builder()
            .details(true)
            .build();
        let current_marks = self.buf.get_extmarks(
            self.namespace_id,
            oxi::api::types::ExtmarkPosition::ByTuple((meta.start_row, meta.start_col)),
            oxi::api::types::ExtmarkPosition::ByTuple((meta.end_row, meta.end_col)),
            &opts,
        );
        if let Ok(cm) = current_marks {
            for mark in cm {
                self.buf.del_extmark(self.namespace_id, mark.0)?;
            }
        }
        // add new mark
        let opts = oxi::api::opts::SetExtmarkOpts::builder()
            .end_row(meta.end_row)
            .end_col(meta.end_col)
            .hl_group(&meta.hl_group)
            .build();
        self.buf
            .set_extmark(self.namespace_id, meta.start_row, meta.start_col, &opts)?;
        Ok(())
    }

    fn apply_highlights(&mut self) -> oxi::Result<()> {
        let count = self.buf.line_count()?;
        let lines = self.buf.get_lines(0..count, true)?;
        let mut input = String::new();
        for line in lines {
            input.push_str(&line.to_string());
            input.push_str("\n");
        }

        // legacy rpgle-lexer based
        let lexer = new_lexer(&input);
        let mut front_kind = TokenKind::Eof;
        if let Ok(tok) = next_token(&lexer) {
            front_kind = tok.kind;
        }
        let mut counter = 0;
        while front_kind != TokenKind::Eof && counter < 1000000 {
            match next_token(&lexer) {
                Ok(tok) => {
                    front_kind = tok.kind;
                    let grp = get_hl_group(&tok.kind);
                    if grp != "Normal" {
                        let meta = HighlightMeta::new(
                            tok.span.start.row,
                            tok.span.start.col,
                            tok.span.end.row,
                            tok.span.end.col,
                            &grp,
                        );
                        if env::var("DEBUG").is_ok() {
                            oxi::print!("{}: {} {}...{}\n", counter, front_kind, &meta, tok.text);
                        }
                        self.highlight(&meta)?;
                    }
                    counter += 1;
                }
                Err(e) => {
                    oxi::print!("{}\n", e);
                    return Ok(());
                }
            }
        }
        if env::var("DEBUG").is_ok() {
            oxi::print!("{}: {}\n", counter, front_kind);
        }

        // rpgle-parser based
        let metas = highlight_all(&input);
        for meta in metas.iter() {
            self.highlight(meta)?;
        }
        Ok(())
    }
}

struct Marker {
    buf: oxi::api::Buffer,
    namespace_id: u32,
}

impl Marker {
    pub fn mark_specs(&mut self) -> oxi::Result<()> {
        let count = self.buf.line_count()?;
        let lines = self.buf.get_lines(0..count, true)?;
        let mut input = String::new();
        for line in lines {
            input.push_str(&line.to_string());
            input.push_str("\n");
        }
        let lexer = new_lexer(&input);
        let mut front_kind = TokenKind::Eof;
        if let Ok(tok) = next_token(&lexer) {
            front_kind = tok.kind;
        }
        let mut counter = 0;
        let mut state = SpecState::new();
        while front_kind != TokenKind::Eof && counter < 1000000 {
            match next_token(&lexer) {
                Ok(tok) => {
                    state.evolve(&tok);
                    counter += 1;
                }
                Err(e) => {
                    oxi::print!("{}\n", e);
                    return Ok(());
                }
            }
        }
        let opts = oxi::api::opts::SetMarkOpts::default();
        let _ = self.buf.set_mark('h', state.get_control(), 1, &opts);
        let _ = self.buf.set_mark('f', state.get_file(), 1, &opts);
        let _ = self.buf.set_mark('d', state.get_definition(), 1, &opts);
        let _ = self.buf.set_mark('c', state.get_calculation(), 1, &opts);
        let _ = self.buf.set_mark('i', state.get_input(), 1, &opts);
        let _ = self.buf.set_mark('o', state.get_output(), 1, &opts);
        if env::var("DEBUG").is_ok() {
            oxi::print!("{}: {}\n", counter, state);
        }
        Ok(())
    }
}

#[nvim_oxi::plugin]
fn idk() -> oxi::Result<oxi::Dictionary> {
    let highlight_rpgle = oxi::Function::from_fn(move |(): ()| {
        let mut highlighter = Highlighter {
            buf: oxi::api::Buffer::current(),
            namespace_id: oxi::api::create_namespace("RPGLENamespace"),
        };
        if let Err(e) = highlighter.apply_highlights() {
            oxi::print!("ERROR");
            oxi::print!("\n");
            oxi::print!("{}", e);
            oxi::print!("\n");
        }
    });

    let mark_rpgle_specs = oxi::Function::from_fn(move |(): ()| {
        let mut marker = Marker {
            buf: oxi::api::Buffer::current(),
            namespace_id: oxi::api::create_namespace("RPGLENamespace2"),
        };
        if let Err(e) = marker.mark_specs() {
            oxi::print!("ERROR");
            oxi::print!("\n");
            oxi::print!("{}", e);
            oxi::print!("\n");
        }
    });

    Ok(oxi::Dictionary::from_iter([
        ("highlight_rpgle", oxi::Object::from(highlight_rpgle)),
        ("mark_rpgle_specs", oxi::Object::from(mark_rpgle_specs)),
    ]))
}
