use super::ast::AST;
use super::spec::Spec;
use crate::free::Op;
use crate::meta::{PMixin, Span};

type SpanShape = ((usize, usize), (usize, usize));
pub fn highlight_ast(ast: &AST) -> Vec<(SpanShape, String)> {
    let mut out = vec![];
    for spec in ast.specs.iter() {
        match spec {
            Spec::H {
                sequence,
                form_type,
                keywords,
            } => {
                out.append(&mut sequence.highlight());
                out.append(&mut form_type.highlight());
                out.append(&mut keywords.highlight());
            }
            Spec::D {
                sequence,
                form_type,
                name,
                external_description,
                datastructure_type,
                definition_type,
                from_position,
                to_length,
                datatype,
                decimals,
                reserved,
                keywords,
            } => {
                out.append(&mut sequence.highlight());
                out.append(&mut form_type.highlight());
                out.append(&mut name.highlight());
                out.append(&mut external_description.highlight());
                out.append(&mut datastructure_type.highlight());
                out.append(&mut definition_type.highlight());
                out.append(&mut from_position.highlight());
                out.append(&mut to_length.highlight());
                out.append(&mut datatype.highlight());
                out.append(&mut decimals.highlight());
                out.append(&mut reserved.highlight());
                out.append(&mut keywords.highlight());
            }
            Spec::F {
                sequence,
                form_type,
                name,
                filetype,
                file_designation,
                endfile,
                file_addition,
                file_sequence,
                file_format,
                record_length,
                limits_processing,
                keylength,
                record_address_type,
                file_organization,
                device,
                reserved,
                keywords,
            } => {
                out.append(&mut sequence.highlight());
                out.append(&mut form_type.highlight());
                out.append(&mut name.highlight());
                out.append(&mut filetype.highlight());
                out.append(&mut file_designation.highlight());
                out.append(&mut endfile.highlight());
                out.append(&mut file_addition.highlight());
                out.append(&mut file_sequence.highlight());
                out.append(&mut file_format.highlight());
                out.append(&mut record_length.highlight());
                out.append(&mut limits_processing.highlight());
                out.append(&mut keylength.highlight());
                out.append(&mut record_address_type.highlight());
                out.append(&mut file_organization.highlight());
                out.append(&mut device.highlight());
                out.append(&mut reserved.highlight());
                out.append(&mut keywords.highlight());
            }
            Spec::C { code } => {
                out.append(&mut code.highlight());
            }
        }
    }
    out.into_iter()
        .map(|tup| {
            (
                (
                    (tup.0.start.row, tup.0.start.col),
                    (tup.0.end.row, tup.0.end.col),
                ),
                tup.1,
            )
        })
        .collect::<Vec<_>>()
}

pub fn query_definition(ast: &AST, pattern: &str) -> Option<Span> {
    for spec in ast.specs.iter() {
        if let Spec::D {
            sequence,
            name,
            keywords,
            ..
        } = spec
        {
            if let Some(namefield) = name.try_as() {
                if namefield.value.to_uppercase() == pattern.to_uppercase() {
                    return Some(namefield.meta.span);
                }
            }
            if let Some(kwfield) = keywords.try_as() {
                for t in kwfield.tokens.iter() {
                    for m in t.metas.iter() {
                        if m.text.to_uppercase().contains(&pattern.to_uppercase()) {
                            return Some(sequence.span());
                        }
                    }
                }
            }
        }

        if let Spec::C { code } = spec {
            if let Some(codefield) = code.try_as() {
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
