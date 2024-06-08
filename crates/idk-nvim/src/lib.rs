use nvim_oxi::{self as oxi};
use rpgle_lexer::{new_lexer, next_token, TokenKind};
use std::env;
use std::fmt;

fn get_hl_group(kind: &TokenKind) -> String {
    match kind {
        TokenKind::FullFree | TokenKind::Free | TokenKind::EndFree => {
            "@keyword.directive".to_string()
        }
        TokenKind::Comment(_) => "@comment".to_string(),
        TokenKind::FormType(_) => "Function".to_string(),
        TokenKind::Name => "Identifier".to_string(),
        TokenKind::FileType(_) => "@keyword.storage".to_string(),
        TokenKind::FileDesignation(_) => "@keyword.directive".to_string(),
        TokenKind::DefinitionType(_) => "@type.definition".to_string(),
        TokenKind::DefinitionDataType(_) => "@type.qualifier".to_string(),
        TokenKind::DefinitionDecimals => "@number".to_string(),
        TokenKind::CompilerDirectiveType(_) => "@keyword.directive.define".to_string(),
        _ => "Normal".to_string(),
    }
}

struct HighlightMeta {
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
    hl_group: String,
}

impl fmt::Display for HighlightMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "({}, {}) -> ({}, {}): {}",
            self.start_row, self.start_col, self.end_row, self.end_col, self.hl_group
        );
        write!(f, "{}", s)
    }
}

impl HighlightMeta {
    fn new(sr: usize, sc: usize, er: usize, ec: usize, hl_group: &str) -> Self {
        Self {
            start_row: sr,
            start_col: sc,
            end_row: er,
            end_col: ec,
            hl_group: hl_group.to_string(),
        }
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

    fn highlight_all(&mut self) -> oxi::Result<()> {
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
        while front_kind != TokenKind::Eof && counter < 200 {
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
                            oxi::print!("{} {}\n", front_kind, &meta);
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
        if let Err(e) = highlighter.highlight_all() {
            oxi::print!("ERROR");
            oxi::print!("\n");
            oxi::print!("{}", e);
            oxi::print!("\n");
        }
    });

    Ok(oxi::Dictionary::from_iter([(
        "highlight_rpgle",
        oxi::Object::from(highlight_rpgle),
    )]))
}
