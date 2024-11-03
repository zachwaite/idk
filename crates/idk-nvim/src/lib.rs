mod graph;
mod highlight;
use dds_parser;
use highlight::{highlight_pfdds, highlight_rpgle, HighlightMeta};
use nvim_oxi::{self as oxi};
use rpgle_parser;
use std::path::PathBuf;
use std::{env, fs};

use graph::{IRenderable, IdkGraph};
use nvim_oxi::conversion::{Error as ConversionError, ToObject};
use nvim_oxi::serde::Serializer;
use nvim_oxi::{lua, Object};
use serde::{Deserialize, Serialize};
use serde_json;

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
                .hl_group(meta.hl_group.as_str())
                .build();
            self.buf
                .set_extmark(self.namespace_id, meta.start_row, meta.start_col, &opts)?;
        }
        Ok(())
    }

    fn apply_rpgle_highlights(&mut self) -> oxi::Result<()> {
        let count = self.buf.line_count()?;
        let lines = self.buf.get_lines(0..count, true)?;
        let mut input = String::new();
        for line in lines {
            input.push_str(&line.to_string());
            input.push_str("\n");
        }
        let metas = highlight_rpgle(&input);
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

    fn apply_pfdds_highlights(&mut self) -> oxi::Result<()> {
        let count = self.buf.line_count()?;
        let lines = self.buf.get_lines(0..count, true)?;
        let mut input = String::new();
        for line in lines {
            input.push_str(&line.to_string());
            input.push_str("\n");
        }
        let metas = highlight_pfdds(&input);
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

#[derive(Debug, Serialize, Deserialize)]
struct TagItem {
    name: String,
    start_line: usize,
    start_char: usize,
    end_line: usize,
    end_char: usize,
    uri: Option<String>,
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

#[derive(Debug)]
struct Manifest {
    uri: String,
}

impl Manifest {
    fn uri_filepath(&self) -> String {
        self.uri.replace("file://", "").to_string()
    }

    fn get_source_files(&self) -> Option<Vec<String>> {
        if let Ok(raw_manifest) = fs::read_to_string(self.uri_filepath()) {
            if let Some(maybebp) = PathBuf::from(&self.uri_filepath()).parent() {
                let mut relpaths = raw_manifest
                    .replace("\n", "")
                    .replace("[", "")
                    .replace("]", "")
                    .replace('"', "")
                    .replace('\\', "")
                    .replace(' ', "")
                    .split(",")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                relpaths.reverse();
                if let Ok(bp) = maybebp.canonicalize() {
                    if let Some(basepath) = bp.to_str() {
                        let out = relpaths
                            .iter()
                            .map(|rp| {
                                let mut out = basepath.to_string();
                                out.push_str("/");
                                out.push_str(rp);
                                format!("{}", out)
                            })
                            .collect::<Vec<String>>();
                        return Some(out);
                    }
                }
            }
        }
        None
    }
}

fn get_manifest() -> Option<Manifest> {
    let buf = oxi::api::Buffer::current();
    if let Ok(bufname) = buf.get_name() {
        if let Some(parent) = bufname.parent() {
            if let Ok(entries) = parent.read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.file_name().to_ascii_lowercase() == "manifest.json" {
                            if let Ok(fp) = entry.path().canonicalize() {
                                if let Some(s) = fp.to_str() {
                                    return Some(Manifest {
                                        uri: format!("file://{}", s),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            if let Some(grandparent) = parent.parent() {
                if let Ok(entries) = grandparent.read_dir() {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if entry.file_name().to_ascii_lowercase() == "manifest.json" {
                                if let Ok(fp) = entry.path().canonicalize() {
                                    if let Some(s) = fp.to_str() {
                                        return Some(Manifest {
                                            uri: format!("file://{}", s),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

#[derive(Debug, Serialize, Deserialize)]
struct DumpOutcome {
    ok: bool,
    msg: Option<String>,
}
impl ToObject for DumpOutcome {
    fn to_object(self) -> Result<Object, ConversionError> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}
impl lua::Pushable for DumpOutcome {
    unsafe fn push(self, lstate: *mut lua::ffi::lua_State) -> Result<std::ffi::c_int, lua::Error> {
        self.to_object()
            .map_err(lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}

fn dot_dump_current_buffer(path: String) -> DumpOutcome {
    let buf = oxi::api::Buffer::current();
    if let Ok(count) = buf.line_count() {
        if let Ok(lines) = buf.get_lines(0..count, true) {
            let mut input = String::new();
            for line in lines {
                input.push_str(&line.to_string());
                input.push_str("\n");
            }
            if let Ok(cst) = rpgle_parser::CST::try_from(input.as_str()) {
                let ast_rs = rpgle_parser::parse_ast(&cst);
                if let Ok(ast) = ast_rs {
                    let graph = IdkGraph::from(&ast);
                    let _ = std::fs::write("/tmp/graph.txt", format!("{:#?}", &graph));
                    let dot = graph.render();
                    let _ = std::fs::write(path, dot);
                    return DumpOutcome {
                        ok: true,
                        msg: None,
                    };
                }
            } else {
                return DumpOutcome {
                    ok: false,
                    msg: Some("Unable to parse AST from current buffer!".to_string()),
                };
            }
        }
    }
    DumpOutcome {
        ok: false,
        msg: Some("Unable to parse CST from current buffer!".to_string()),
    }
}

fn json_dump_current_buffer(path: String) -> DumpOutcome {
    let buf = oxi::api::Buffer::current();
    if let Ok(count) = buf.line_count() {
        if let Ok(lines) = buf.get_lines(0..count, true) {
            let mut input = String::new();
            for line in lines {
                input.push_str(&line.to_string());
                input.push_str("\n");
            }
            if let Ok(cst) = rpgle_parser::CST::try_from(input.as_str()) {
                match serde_json::to_string(&cst) {
                    Ok(jsons) => {
                        let _ = std::fs::write(path, jsons);
                        DumpOutcome {
                            ok: true,
                            msg: None,
                        }
                    }
                    Err(e) => DumpOutcome {
                        ok: false,
                        msg: Some(e.to_string()),
                    },
                };
            }
        }
    }
    DumpOutcome {
        ok: false,
        msg: Some("Unable to parse CST from current buffer!".to_string()),
    }
}

fn getdef(pattern: String) -> Option<TagItem> {
    let buf = oxi::api::Buffer::current();
    let current_file: String = match buf.get_name() {
        Ok(pb) => pb
            .as_path()
            .file_name()
            .expect("path from pathbuf from ok result should always unwrap")
            .to_str()
            .expect("should always be able to convert osstr to str in this context")
            .to_string()
            .to_uppercase(),
        Err(_) => "".to_string(),
    };
    let current_row = match oxi::api::get_current_win().get_cursor() {
        Ok((row1, _)) => row1 - 1,
        Err(_) => 0,
    };
    if let Ok(count) = buf.line_count() {
        if let Ok(lines) = buf.get_lines(0..count, true) {
            let mut input = String::new();
            for line in lines {
                input.push_str(&line.to_string());
                input.push_str("\n");
            }
            if let Ok(cst) = rpgle_parser::CST::try_from(input.as_str()) {
                let ast_rs = rpgle_parser::parse_ast(&cst);
                if let Ok(ast) = ast_rs {
                    if let Some(def) = rpgle_parser::query_definition(&ast, &pattern) {
                        if def.start.row != current_row {
                            let ti = TagItem {
                                name: pattern.clone(),
                                uri: None,
                                start_line: def.start.row,
                                start_char: def.start.col,
                                end_line: def.end.row,
                                end_char: def.end.col,
                            };
                            if env::var("DEBUG").is_ok() {
                                let _ = std::fs::write("/tmp/getdef.txt", format!("{:#?}", ti));
                            }
                            return Some(ti);
                        }
                    }
                    // else
                    if let Some(man) = get_manifest() {
                        if let Some(srcs) = man.get_source_files() {
                            let mut sources = srcs
                                .into_iter()
                                .filter(|x| !x.to_uppercase().ends_with(&current_file))
                                .collect::<Vec<String>>();
                            sources.sort_by_key(|x| {
                                match x.to_uppercase().contains(&pattern.to_uppercase()) {
                                    true => 0,
                                    false => 1,
                                }
                            });
                            for source in sources {
                                if source.ends_with("rpgle") {
                                    if let Ok(input) = fs::read_to_string(source.clone()) {
                                        if let Ok(cst) = rpgle_parser::CST::try_from(input.as_str())
                                        {
                                            let ast_rs = rpgle_parser::parse_ast(&cst);
                                            if let Ok(ast) = ast_rs {
                                                if let Some(def) =
                                                    rpgle_parser::query_definition(&ast, &pattern)
                                                {
                                                    let uri = format!("file://{}", source);
                                                    let ti = TagItem {
                                                        name: pattern.clone(),
                                                        uri: Some(uri),
                                                        start_line: def.start.row,
                                                        start_char: def.start.col,
                                                        end_line: def.end.row,
                                                        end_char: def.end.col,
                                                    };
                                                    if env::var("DEBUG").is_ok() {
                                                        let _ = std::fs::write(
                                                            "/tmp/getdef.txt",
                                                            format!("{:#?}", ti),
                                                        );
                                                    }
                                                    return Some(ti);
                                                }
                                            }
                                        }
                                    }
                                }
                                if source.ends_with("pfdds") {
                                    if let Ok(input) = fs::read_to_string(&source) {
                                        if let Ok(cst) =
                                            dds_parser::pfdds::CST::try_from(input.as_str())
                                        {
                                            let ast = dds_parser::pfdds::AST::from(&cst);
                                            if let Some(def) =
                                                dds_parser::pfdds::query_definition(&ast, &pattern)
                                            {
                                                let uri = format!("file://{}", source);
                                                let ti = TagItem {
                                                    name: pattern.clone(),
                                                    uri: Some(uri),
                                                    start_line: def.start.row,
                                                    start_char: def.start.col,
                                                    end_line: def.end.row,
                                                    end_char: def.end.col,
                                                };
                                                if env::var("DEBUG").is_ok() {
                                                    let _ = std::fs::write(
                                                        "/tmp/getdef.txt",
                                                        format!("{:#?}", ti),
                                                    );
                                                }
                                                return Some(ti);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

#[nvim_oxi::plugin]
fn libidk() -> oxi::Result<oxi::Dictionary> {
    let highlight_rpgle = oxi::Function::from_fn(move |(): ()| {
        let mut highlighter = Highlighter {
            buf: oxi::api::Buffer::current(),
            namespace_id: oxi::api::create_namespace("RPGLENamespace"),
        };
        if let Err(e) = highlighter.apply_rpgle_highlights() {
            oxi::print!("ERROR");
            oxi::print!("\n");
            oxi::print!("{}", e);
            oxi::print!("\n");
        }
    });

    let highlight_pfdds = oxi::Function::from_fn(move |(): ()| {
        let mut highlighter = Highlighter {
            buf: oxi::api::Buffer::current(),
            namespace_id: oxi::api::create_namespace("PFDDSNamespace"),
        };
        if let Err(e) = highlighter.apply_pfdds_highlights() {
            oxi::print!("ERROR");
            oxi::print!("\n");
            oxi::print!("{}", e);
            oxi::print!("\n");
        }
    });

    let getdef = oxi::Function::from_fn(getdef);

    let json_dump_current_buffer = oxi::Function::from_fn(json_dump_current_buffer);
    let dot_dump_current_buffer = oxi::Function::from_fn(dot_dump_current_buffer);

    Ok(oxi::Dictionary::from_iter([
        ("highlight_rpgle", oxi::Object::from(highlight_rpgle)),
        ("highlight_pfdds", oxi::Object::from(highlight_pfdds)),
        ("getdef", oxi::Object::from(getdef)),
        (
            "json_dump_current_buffer",
            oxi::Object::from(json_dump_current_buffer),
        ),
        (
            "dot_dump_current_buffer",
            oxi::Object::from(dot_dump_current_buffer),
        ),
    ]))
}
