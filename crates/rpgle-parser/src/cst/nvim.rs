use super::cst::CST;
use super::srcline::{CSrcline, Srcline};
use crate::field::FieldBehavior;
use crate::meta::Span;
use std::usize;

type SpanShape = ((usize, usize), (usize, usize));
fn highlight_csrcline(line: &CSrcline) -> Vec<(Span, String)> {
    let mut out = vec![];
    match line {
        CSrcline::Traditional {
            nothing,
            form_type,
            control_level,
            indicators,
            factor1,
            operation,
            factor2,
            result,
            result_length,
            decimals,
            resulting_indicators,
            comments,
        } => {
            out.append(&mut nothing.highlight());
            out.append(&mut form_type.highlight());
            out.append(&mut control_level.highlight());
            out.append(&mut indicators.highlight());
            out.append(&mut factor1.highlight());
            out.append(&mut operation.highlight());
            out.append(&mut factor2.highlight());
            out.append(&mut result.highlight());
            out.append(&mut result_length.highlight());
            out.append(&mut decimals.highlight());
            out.append(&mut resulting_indicators.highlight());
            out.append(&mut comments.highlight());
        }
        CSrcline::ExtF2 {
            nothing,
            form_type,
            control_level,
            indicators,
            factor1,
            operation,
            factor2,
        } => {
            out.append(&mut nothing.highlight());
            out.append(&mut form_type.highlight());
            out.append(&mut control_level.highlight());
            out.append(&mut indicators.highlight());
            out.append(&mut factor1.highlight());
            out.append(&mut operation.highlight());
            out.append(&mut factor2.highlight());
        }
        CSrcline::Free { nothing, code } => {
            out.append(&mut nothing.highlight());
            out.append(&mut code.highlight());
        }
    };
    out
}

pub fn highlight_cst(cst: &CST) -> Vec<(SpanShape, String)> {
    let mut out = vec![];
    for line in cst.lines.iter() {
        match line {
            Srcline::Idk { idk } => out.append(&mut idk.highlight()),
            Srcline::Comment {
                sequence,
                form_type,
                comment,
            } => {
                out.append(&mut sequence.highlight());
                out.append(&mut form_type.highlight());
                out.append(&mut comment.highlight());
            }
            Srcline::CompilerDirective {
                sequence,
                form_type,
                directive,
            } => {
                out.append(&mut sequence.highlight());
                out.append(&mut form_type.highlight());
                out.append(&mut directive.highlight());
            }
            Srcline::H {
                sequence,
                form_type,
                keywords,
            } => {
                out.append(&mut sequence.highlight());
                out.append(&mut form_type.highlight());
                out.append(&mut keywords.highlight());
            }
            Srcline::F {
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
            Srcline::FCont {
                sequence,
                form_type,
                nothing,
                keywords,
            } => {
                out.append(&mut sequence.highlight());
                out.append(&mut form_type.highlight());
                out.append(&mut nothing.highlight());
                out.append(&mut keywords.highlight());
            }
            Srcline::D {
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
            Srcline::DCont {
                sequence,
                form_type,
                nothing,
                keywords,
            } => {
                out.append(&mut sequence.highlight());
                out.append(&mut form_type.highlight());
                out.append(&mut nothing.highlight());
                out.append(&mut keywords.highlight());
            }
            Srcline::C(cline) => out.append(&mut highlight_csrcline(cline)),
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
