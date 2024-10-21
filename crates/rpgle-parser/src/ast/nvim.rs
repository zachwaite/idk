use super::core::AST;
use crate::field::FieldResult;
use crate::free::Op;
use crate::meta::{PMixin, Span};
use crate::spec::Spec;

pub fn highlight_ast(ast: AST) -> Vec<((usize, usize), (usize, usize), String)> {
    let mut out = vec![];
    for spec in ast.specs.iter() {
        match spec {
            Spec::H(spec) => {
                out.append(&mut spec.highlight());
            }
            Spec::D(spec) => {
                out.append(&mut spec.highlight());
            }
            Spec::F(spec) => {
                out.append(&mut spec.highlight());
            }
            Spec::C(spec) => {
                out.append(&mut spec.highlight());
            }
            _ => continue,
        }
    }
    out.into_iter()
        .map(|tup| {
            (
                (tup.0.start.row, tup.0.start.col),
                (tup.0.end.row, tup.0.end.col),
                tup.1,
            )
        })
        .collect::<Vec<_>>()
}

pub fn query_definition(ast: &AST, pattern: &str) -> Option<Span> {
    for spec in ast.specs.iter() {
        if let Spec::D(dspec) = spec {
            if let FieldResult::Ok(namefield) = &dspec.name {
                if namefield.value.to_uppercase() == pattern.to_uppercase() {
                    return Some(namefield.meta.span);
                }
            }

            if let FieldResult::Ok(kwfield) = &dspec.keywords {
                for t in kwfield.tokens.iter() {
                    for m in t.metas.iter() {
                        if m.text.to_uppercase().contains(&pattern.to_uppercase()) {
                            return Some(dspec.sequence.span());
                        }
                    }
                }
            }
        }
        if let Spec::C(cspec) = spec {
            if let FieldResult::Ok(codefield) = &cspec.code {
                if let Op::Begsr { name, .. } = &codefield.op {
                    if name.trim().to_uppercase() == pattern.trim().to_uppercase() {
                        return Some(codefield.op.span());
                    }
                }
            }
        }
    }
    None
}
