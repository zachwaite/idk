mod highlight;
use highlight::{highlight_all, HighlightMeta};
use nvim_oxi::{self as oxi};
use rpgle_parser::{query_definition, Span, AST, CST};
use std::env;

use nvim_oxi::conversion::{Error as ConversionError, FromObject, ToObject};
use nvim_oxi::serde::{Deserializer, Serializer};
use nvim_oxi::{api, lua, print, Dictionary, Function, Object};
use serde::{Deserialize, Serialize};

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
        // line length can be shorter than the end_col if formatting hasn't happened
        let maxlen = self
            .buf
            .get_lines(meta.start_row..=meta.end_row, true)?
            .map(|line| line.len())
            .max()
            .expect("This should be the max length of lines...");
        if meta.start_col <= maxlen {
            let endcol = if meta.end_col <= maxlen {
                meta.end_col
            } else {
                maxlen
            };
            let opts = oxi::api::opts::SetExtmarkOpts::builder()
                .end_row(meta.end_row)
                .end_col(endcol)
                .hl_group(&meta.hl_group)
                .build();
            self.buf
                .set_extmark(self.namespace_id, meta.start_row, meta.start_col, &opts)?;
        }
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
        let metas = highlight_all(&input);
        if env::var("DEBUG").is_ok() {
            let _ = std::fs::write(
                "/tmp/highlights.txt",
                metas
                    .iter()
                    .map(|m| {
                        let mut out = m.to_string();
                        out.push_str("\n");
                        out
                    })
                    .collect::<String>(),
            );
        }
        for meta in metas.iter() {
            self.highlight(meta)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct TagItem {
    name: String,
    start_line: usize,
    start_char: usize,
    end_line: usize,
    end_char: usize,
}
impl ToObject for TagItem {
    fn to_object(self) -> Result<Object, ConversionError> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}
impl lua::Pushable for TagItem {
    unsafe fn push(self, lstate: *mut lua::ffi::lua_State) -> Result<std::ffi::c_int, lua::Error> {
        self.to_object()
            .map_err(lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}

fn getdef(pattern: String) -> Option<TagItem> {
    let buf = oxi::api::Buffer::current();
    if let Ok(count) = buf.line_count() {
        if let Ok(lines) = buf.get_lines(0..count, true) {
            let mut input = String::new();
            for line in lines {
                input.push_str(&line.to_string());
                input.push_str("\n");
            }
            if let Ok(cst) = CST::try_from(input.as_str()) {
                let ast = AST::from(&cst);
                if let Some(def) = query_definition(&ast, &pattern) {
                    return Some(TagItem {
                        name: pattern.clone(),
                        start_line: def.start.row,
                        start_char: def.start.col,
                        end_line: def.end.row,
                        end_char: def.end.col,
                    });
                }
            }
        }
    }
    None
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

    let getdef = oxi::Function::from_fn(getdef);
    Ok(oxi::Dictionary::from_iter([
        ("highlight_rpgle", oxi::Object::from(highlight_rpgle)),
        ("getdef", oxi::Object::from(getdef)),
    ]))
}
