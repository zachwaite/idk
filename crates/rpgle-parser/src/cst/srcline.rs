// Adapts SpecLine to SrcLine
use crate::field::{
    has_extf2_optoken, CommentField, CompilerDirectiveField, ControlLevelField,
    DatastructureTypeField, DatatypeField, DecimalsField, DefinitionTypeField, DeviceField,
    EndfileField, ExternalDescriptionField, Factor1Field, FieldResult, FileAdditionField,
    FileDesignationField, FileFormatField, FileOrganizationField, FileSequenceField, FiletypeField,
    FormtypeField, IdkField, IndicatorsField, KeyLengthField, LimitsProcessingField, NameField,
    NothingField, OperationField, POSField, RawCodeField, RawFactor2Field, RawKeywordsField,
    RecordAddressTypeField, RecordLengthField, ReservedField, ResultField, ResultLengthField,
    SequenceField,
};
use crate::meta::pluck_array3 as pluck;
use crate::meta::Position;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    Unhandled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CSrcline {
    Traditional {
        nothing: FieldResult<NothingField>,
        form_type: FieldResult<FormtypeField>,
        control_level: FieldResult<ControlLevelField>,
        indicators: FieldResult<IndicatorsField>,
        factor1: FieldResult<Factor1Field>,
        operation: FieldResult<OperationField>,
        factor2: FieldResult<RawFactor2Field>,
        result: FieldResult<ResultField>,
        result_length: FieldResult<ResultLengthField>,
        decimals: FieldResult<DecimalsField>,
        resulting_indicators: FieldResult<IndicatorsField>,
        comments: FieldResult<CommentField>,
    },
    ExtF2 {
        nothing: FieldResult<NothingField>,
        form_type: FieldResult<FormtypeField>,
        control_level: FieldResult<ControlLevelField>,
        indicators: FieldResult<IndicatorsField>,
        factor1: FieldResult<Factor1Field>,
        operation: FieldResult<OperationField>,
        factor2: FieldResult<RawFactor2Field>,
    },
    Free {
        nothing: FieldResult<NothingField>,
        code: FieldResult<RawCodeField>,
    },
}

impl Display for CSrcline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        match self {
            Self::Traditional {
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
                msg.push_str(&nothing.to_string());
                msg.push_str(&form_type.to_string());
                msg.push_str(&control_level.to_string());
                msg.push_str(&indicators.to_string());
                msg.push_str(&factor1.to_string());
                msg.push_str(&operation.to_string());
                msg.push_str(&factor2.to_string());
                msg.push_str(&result.to_string());
                msg.push_str(&result_length.to_string());
                msg.push_str(&decimals.to_string());
                msg.push_str(&resulting_indicators.to_string());
                msg.push_str(&comments.to_string());
            }
            Self::ExtF2 {
                nothing,
                form_type,
                control_level,
                indicators,
                factor1,
                operation,
                factor2,
            } => {
                msg.push_str(&nothing.to_string());
                msg.push_str(&form_type.to_string());
                msg.push_str(&control_level.to_string());
                msg.push_str(&indicators.to_string());
                msg.push_str(&factor1.to_string());
                msg.push_str(&operation.to_string());
                msg.push_str(&factor2.to_string());
            }
            Self::Free { nothing, code } => {
                msg.push_str(&nothing.to_string());
                msg.push_str(&code.to_string());
            }
        }
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Srcline {
    Idk {
        idk: FieldResult<IdkField>,
    },
    Comment {
        sequence: FieldResult<SequenceField>,
        form_type: FieldResult<FormtypeField>,
        comment: FieldResult<CommentField>,
    },
    CompilerDirective {
        sequence: FieldResult<SequenceField>,
        form_type: FieldResult<FormtypeField>,
        directive: FieldResult<CompilerDirectiveField>,
    },
    H {
        sequence: FieldResult<SequenceField>,
        form_type: FieldResult<FormtypeField>,
        keywords: FieldResult<RawKeywordsField>,
    },
    F {
        sequence: FieldResult<SequenceField>,
        form_type: FieldResult<FormtypeField>,
        name: FieldResult<NameField>,
        filetype: FieldResult<FiletypeField>,
        file_designation: FieldResult<FileDesignationField>,
        endfile: FieldResult<EndfileField>,
        file_addition: FieldResult<FileAdditionField>,
        file_sequence: FieldResult<FileSequenceField>,
        file_format: FieldResult<FileFormatField>,
        record_length: FieldResult<RecordLengthField>,
        limits_processing: FieldResult<LimitsProcessingField>,
        keylength: FieldResult<KeyLengthField>,
        record_address_type: FieldResult<RecordAddressTypeField>,
        file_organization: FieldResult<FileOrganizationField>,
        device: FieldResult<DeviceField>,
        reserved: FieldResult<ReservedField>,
        keywords: FieldResult<RawKeywordsField>,
    },
    FCont {
        sequence: FieldResult<SequenceField>,
        form_type: FieldResult<FormtypeField>,
        nothing: FieldResult<NothingField>,
        keywords: FieldResult<RawKeywordsField>,
    },
    D {
        sequence: FieldResult<SequenceField>,
        form_type: FieldResult<FormtypeField>,
        name: FieldResult<NameField>,
        external_description: FieldResult<ExternalDescriptionField>,
        datastructure_type: FieldResult<DatastructureTypeField>,
        definition_type: FieldResult<DefinitionTypeField>,
        from_position: FieldResult<POSField>,
        to_length: FieldResult<POSField>,
        datatype: FieldResult<DatatypeField>,
        decimals: FieldResult<DecimalsField>,
        reserved: FieldResult<ReservedField>,
        keywords: FieldResult<RawKeywordsField>,
    },
    DCont {
        sequence: FieldResult<SequenceField>,
        form_type: FieldResult<FormtypeField>,
        nothing: FieldResult<NothingField>,
        keywords: FieldResult<RawKeywordsField>,
    },
    C(CSrcline),
}

impl Display for Srcline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        match self {
            Srcline::Idk { idk } => {
                msg.push_str(&idk.to_string());
            }
            Srcline::Comment {
                sequence,
                form_type,
                comment,
            } => {
                msg.push_str(&sequence.to_string());
                msg.push_str(&form_type.to_string());
                msg.push_str(&comment.to_string());
            }
            Srcline::CompilerDirective {
                sequence,
                form_type,
                directive,
            } => {
                msg.push_str(&sequence.to_string());
                msg.push_str(&form_type.to_string());
                msg.push_str(&directive.to_string());
            }
            Srcline::H {
                sequence,
                form_type,
                keywords,
            } => {
                msg.push_str(&sequence.to_string());
                msg.push_str(&form_type.to_string());
                msg.push_str(&keywords.to_string());
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
                msg.push_str(&sequence.to_string());
                msg.push_str(&form_type.to_string());
                msg.push_str(&name.to_string());
                msg.push_str(&filetype.to_string());
                msg.push_str(&file_designation.to_string());
                msg.push_str(&endfile.to_string());
                msg.push_str(&file_addition.to_string());
                msg.push_str(&file_sequence.to_string());
                msg.push_str(&file_format.to_string());
                msg.push_str(&record_length.to_string());
                msg.push_str(&limits_processing.to_string());
                msg.push_str(&keylength.to_string());
                msg.push_str(&record_address_type.to_string());
                msg.push_str(&file_organization.to_string());
                msg.push_str(&device.to_string());
                msg.push_str(&reserved.to_string());
                msg.push_str(&keywords.to_string());
            }
            Srcline::FCont {
                sequence,
                form_type,
                nothing,
                keywords,
            } => {
                msg.push_str(&sequence.to_string());
                msg.push_str(&form_type.to_string());
                msg.push_str(&nothing.to_string());
                msg.push_str(&keywords.to_string());
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
                msg.push_str(&sequence.to_string());
                msg.push_str(&form_type.to_string());
                msg.push_str(&name.to_string());
                msg.push_str(&external_description.to_string());
                msg.push_str(&datastructure_type.to_string());
                msg.push_str(&definition_type.to_string());
                msg.push_str(&from_position.to_string());
                msg.push_str(&to_length.to_string());
                msg.push_str(&datatype.to_string());
                msg.push_str(&decimals.to_string());
                msg.push_str(&reserved.to_string());
                msg.push_str(&keywords.to_string());
            }
            Srcline::DCont {
                sequence,
                form_type,
                nothing,
                keywords,
            } => {
                msg.push_str(&sequence.to_string());
                msg.push_str(&form_type.to_string());
                msg.push_str(&nothing.to_string());
                msg.push_str(&keywords.to_string());
            }
            Srcline::C(cline) => msg.push_str(&cline.to_string()),
        }
        write!(f, "{}", msg)
    }
}

fn try_comment(row: usize, chars: &[char; 100]) -> Option<Srcline> {
    // guard: comment symbol
    if chars[6] != '*' {
        return None;
    }
    let line = Srcline::Comment {
        sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
        form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
        comment: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 94, 0>(chars))),
    };
    Some(line)
}

fn try_compiler_directive(row: usize, chars: &[char; 100]) -> Option<Srcline> {
    // guard: directive symbol
    if chars[6] != '/' {
        return None;
    }
    let line = Srcline::CompilerDirective {
        sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
        form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
        directive: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 94, 0>(chars))),
    };
    Some(line)
}

fn try_hline(row: usize, chars: &[char; 100]) -> Option<Srcline> {
    // guard: form type
    if chars[5] != 'H' {
        return None;
    }
    let line = Srcline::H {
        sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
        form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
        keywords: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 94, 0>(chars))),
    };
    Some(line)
}

fn try_fline(row: usize, chars: &[char; 100]) -> Option<Srcline> {
    // guard: form type
    if chars[5] != 'F' {
        return None;
    }
    // guard: continuations would be all blank here
    let unique_chars = chars[6..42].iter().collect::<HashSet<&char>>();
    if unique_chars.len() == 1 && unique_chars.contains(&' ') {
        return None;
    }
    let line = Srcline::F {
        sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
        form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
        name: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 10, 84>(chars))),
        filetype: FieldResult::from((Position::from((row, 16)), pluck::<100, 16, 1, 83>(chars))),
        file_designation: FieldResult::from((
            Position::from((row, 17)),
            pluck::<100, 17, 1, 82>(chars),
        )),
        endfile: FieldResult::from((Position::from((row, 18)), pluck::<100, 18, 1, 81>(chars))),
        file_addition: FieldResult::from((
            Position::from((row, 19)),
            pluck::<100, 19, 1, 80>(chars),
        )),
        file_sequence: FieldResult::from((
            Position::from((row, 20)),
            pluck::<100, 20, 1, 79>(chars),
        )),
        file_format: FieldResult::from((Position::from((row, 21)), pluck::<100, 21, 1, 78>(chars))),
        record_length: FieldResult::from((
            Position::from((row, 22)),
            pluck::<100, 22, 5, 73>(chars),
        )),
        limits_processing: FieldResult::from((
            Position::from((row, 27)),
            pluck::<100, 27, 1, 72>(chars),
        )),
        keylength: FieldResult::from((Position::from((row, 28)), pluck::<100, 28, 5, 67>(chars))),
        record_address_type: FieldResult::from((
            Position::from((row, 33)),
            pluck::<100, 33, 1, 66>(chars),
        )),
        file_organization: FieldResult::from((
            Position::from((row, 34)),
            pluck::<100, 34, 1, 65>(chars),
        )),
        device: FieldResult::from((Position::from((row, 35)), pluck::<100, 35, 7, 58>(chars))),
        reserved: FieldResult::from((Position::from((row, 42)), pluck::<100, 42, 1, 57>(chars))),
        keywords: FieldResult::from((Position::from((row, 43)), pluck::<100, 43, 57, 0>(chars))),
    };
    Some(line)
}

fn try_fline_continuation(row: usize, chars: &[char; 100]) -> Option<Srcline> {
    // guard: form type
    if chars[5] != 'F' {
        return None;
    }
    // guard: continuations will be all blank here
    let unique_chars = chars[6..42].iter().collect::<HashSet<&char>>();
    let guard = || unique_chars.len() == 1 && unique_chars.contains(&' ');
    if !guard() {
        return None;
    }
    let line = Srcline::FCont {
        sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
        form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
        nothing: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 37, 57>(chars))),
        keywords: FieldResult::from((Position::from((row, 43)), pluck::<100, 43, 57, 0>(chars))),
    };
    Some(line)
}

fn try_dline(row: usize, chars: &[char; 100]) -> Option<Srcline> {
    // guard: form type
    if chars[5] != 'D' {
        return None;
    }
    // guard: continuations will be all blank here
    let unique_chars = chars[6..42].iter().collect::<HashSet<&char>>();
    let guard = || unique_chars.len() == 1 && unique_chars.contains(&' ');
    if guard() {
        return None;
    }
    let line = Srcline::D {
        sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
        form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
        name: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 15, 79>(chars))),
        external_description: FieldResult::from((
            Position::from((row, 21)),
            pluck::<100, 21, 1, 78>(chars),
        )),
        datastructure_type: FieldResult::from((
            Position::from((row, 22)),
            pluck::<100, 22, 1, 77>(chars),
        )),
        definition_type: FieldResult::from((
            Position::from((row, 23)),
            pluck::<100, 23, 2, 75>(chars),
        )),
        from_position: FieldResult::from((
            Position::from((row, 25)),
            pluck::<100, 25, 7, 68>(chars),
        )),
        to_length: FieldResult::from((Position::from((row, 32)), pluck::<100, 32, 7, 61>(chars))),
        datatype: FieldResult::from((Position::from((row, 39)), pluck::<100, 39, 1, 60>(chars))),
        decimals: FieldResult::from((Position::from((row, 40)), pluck::<100, 40, 2, 58>(chars))),
        reserved: FieldResult::from((Position::from((row, 42)), pluck::<100, 42, 1, 57>(chars))),
        keywords: FieldResult::from((Position::from((row, 43)), pluck::<100, 43, 57, 0>(chars))),
    };
    Some(line)
}

fn try_dline_continuation(row: usize, chars: &[char; 100]) -> Option<Srcline> {
    // guard: form type
    if chars[5] != 'D' {
        return None;
    }
    // guard: continuations will be all blank here
    for c in &chars[6..42] {
        if *c != ' ' {
            return None;
        }
    }
    let line = Srcline::DCont {
        sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
        form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
        nothing: FieldResult::from((Position::from((row, 37)), pluck::<100, 6, 37, 57>(chars))),
        keywords: FieldResult::from((Position::from((row, 43)), pluck::<100, 43, 57, 0>(chars))),
    };
    Some(line)
}

fn try_cline_extf2(row: usize, chars: &[char; 100]) -> Option<Srcline> {
    // guard: form type
    if chars[5] != 'C' {
        return None;
    }
    // guard: extf2 op token = extf2
    if !has_extf2_optoken(chars) {
        return None;
    }
    let line = Srcline::C(CSrcline::ExtF2 {
        nothing: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
        form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
        control_level: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 2, 92>(chars))),
        indicators: FieldResult::from((Position::from((row, 8)), pluck::<100, 8, 3, 89>(chars))),
        factor1: FieldResult::from((Position::from((row, 11)), pluck::<100, 11, 14, 75>(chars))),
        operation: FieldResult::from((Position::from((row, 25)), pluck::<100, 25, 10, 65>(chars))),
        factor2: FieldResult::from((Position::from((row, 35)), pluck::<100, 35, 65, 0>(chars))),
    });
    Some(line)
}

fn try_cline_traditional(row: usize, chars: &[char; 100]) -> Option<Srcline> {
    // guard: form type
    if chars[5] != 'C' {
        return None;
    }
    // guard: no extf2 op token = traditional
    if has_extf2_optoken(chars) {
        return None;
    }
    let line = Srcline::C(CSrcline::Traditional {
        nothing: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
        form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
        control_level: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 2, 92>(chars))),
        indicators: FieldResult::from((Position::from((row, 8)), pluck::<100, 8, 3, 89>(chars))),
        factor1: FieldResult::from((Position::from((row, 11)), pluck::<100, 11, 14, 75>(chars))),
        operation: FieldResult::from((Position::from((row, 25)), pluck::<100, 25, 10, 65>(chars))),
        factor2: FieldResult::from((
            // this doesn't match the documentation, but matches every program I can find
            // ...wtf...wtf...wtf...wtf....
            Position::from((row, 35)),
            pluck::<100, 35, 13, 52>(chars),
        )),
        result: FieldResult::from((Position::from((row, 49)), pluck::<100, 49, 14, 37>(chars))),
        result_length: FieldResult::from((
            Position::from((row, 63)),
            pluck::<100, 63, 5, 32>(chars),
        )),
        decimals: FieldResult::from((Position::from((row, 67)), pluck::<100, 67, 2, 31>(chars))),
        resulting_indicators: FieldResult::from((
            Position::from((row, 69)),
            pluck::<100, 69, 5, 26>(chars),
        )),
        comments: FieldResult::from((Position::from((row, 74)), pluck::<100, 74, 26, 0>(chars))),
    });
    Some(line)
}

fn try_cline_free(row: usize, chars: &[char; 100]) -> Option<Srcline> {
    // guard: form type + comment
    if chars[5..=6] != [' ', ' '] {
        return None;
    }
    // guard: not blank line
    let unique_chars = chars.iter().collect::<HashSet<&char>>();
    if unique_chars.len() == 1 && unique_chars.contains(&' ') {
        return None;
    }
    let line = Srcline::C(CSrcline::Free {
        nothing: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 7, 93>(chars))),
        code: FieldResult::from((Position::from((row, 7)), pluck::<100, 7, 93, 0>(chars))),
    });
    Some(line)
}

fn try_idk(row: usize, chars: &[char; 100]) -> Option<Srcline> {
    let start = Position::from((row, 0));
    let line = Srcline::Idk {
        idk: FieldResult::from((start, chars.as_slice())),
    };
    Some(line)
}

pub fn srcline(row: usize, chars: &[char; 100]) -> Result<Srcline, ParseError> {
    let parse_comment = || try_comment(row, chars);
    let parse_compiler_directive = || try_compiler_directive(row, chars);
    let parse_hline = || try_hline(row, chars);
    let parse_fline = || try_fline(row, chars);
    let parse_fline_cont = || try_fline_continuation(row, chars);
    let parse_dline = || try_dline(row, chars);
    let parse_dline_cont = || try_dline_continuation(row, chars);
    let parse_cline_traditional = || try_cline_traditional(row, chars);
    let parse_cline_extf2 = || try_cline_extf2(row, chars);
    let parse_cline_free = || try_cline_free(row, chars);
    let parse_idk = || try_idk(row, chars);
    parse_comment()
        .or_else(parse_compiler_directive)
        .or_else(parse_hline)
        .or_else(parse_fline)
        .or_else(parse_fline_cont)
        .or_else(parse_dline)
        .or_else(parse_dline_cont)
        .or_else(parse_cline_traditional)
        .or_else(parse_cline_extf2)
        .or_else(parse_cline_free)
        .or_else(parse_idk)
        .ok_or(ParseError::Unhandled)
}
