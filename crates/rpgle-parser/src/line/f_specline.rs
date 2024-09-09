use crate::field::{
    DeviceField, EndfileField, FKeywordsField, FieldResult, FileAdditionField,
    FileDesignationField, FileFormatField, FileOrganizationField, FileSequenceField, FiletypeField,
    FormtypeField, KeyLengthField, LimitsProcessingField, NameField, NothingField,
    RecordAddressTypeField, RecordLengthField, ReservedField, SequenceField,
};
use crate::meta::pluck_array3 as pluck;
use crate::meta::{PMixin, Position, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSpecLine {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub name: FieldResult<NameField>,
    pub filetype: FieldResult<FiletypeField>,
    pub file_designation: FieldResult<FileDesignationField>,
    pub endfile: FieldResult<EndfileField>,
    pub file_addition: FieldResult<FileAdditionField>,
    pub file_sequence: FieldResult<FileSequenceField>,
    pub file_format: FieldResult<FileFormatField>,
    pub record_length: FieldResult<RecordLengthField>,
    pub limits_processing: FieldResult<LimitsProcessingField>,
    pub keylength: FieldResult<KeyLengthField>,
    pub record_address_type: FieldResult<RecordAddressTypeField>,
    pub file_organization: FieldResult<FileOrganizationField>,
    pub device: FieldResult<DeviceField>,
    pub reserved: FieldResult<ReservedField>,
    pub keywords: FieldResult<FKeywordsField>,
}

impl Display for FSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.sequence.to_string());
        msg.push_str(&self.form_type.to_string());
        msg.push_str(&self.name.to_string());
        msg.push_str(&self.filetype.to_string());
        msg.push_str(&self.file_designation.to_string());
        msg.push_str(&self.endfile.to_string());
        msg.push_str(&self.file_addition.to_string());
        msg.push_str(&self.file_sequence.to_string());
        msg.push_str(&self.file_format.to_string());
        msg.push_str(&self.record_length.to_string());
        msg.push_str(&self.limits_processing.to_string());
        msg.push_str(&self.keylength.to_string());
        msg.push_str(&self.record_address_type.to_string());
        msg.push_str(&self.file_organization.to_string());
        msg.push_str(&self.device.to_string());
        msg.push_str(&self.reserved.to_string());
        msg.push_str(&self.keywords.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for FSpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let chars = value.1;
        Self {
            sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
            name: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 10, 84>(chars))),
            filetype: FieldResult::from((
                Position::from((row, 16)),
                pluck::<100, 16, 1, 83>(chars),
            )),
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
            file_format: FieldResult::from((
                Position::from((row, 21)),
                pluck::<100, 21, 1, 78>(chars),
            )),
            record_length: FieldResult::from((
                Position::from((row, 22)),
                pluck::<100, 22, 5, 73>(chars),
            )),
            limits_processing: FieldResult::from((
                Position::from((row, 27)),
                pluck::<100, 27, 1, 72>(chars),
            )),
            keylength: FieldResult::from((
                Position::from((row, 28)),
                pluck::<100, 28, 5, 67>(chars),
            )),
            record_address_type: FieldResult::from((
                Position::from((row, 33)),
                pluck::<100, 33, 1, 66>(chars),
            )),
            file_organization: FieldResult::from((
                Position::from((row, 34)),
                pluck::<100, 34, 1, 65>(chars),
            )),
            device: FieldResult::from((Position::from((row, 35)), pluck::<100, 35, 7, 58>(chars))),
            reserved: FieldResult::from((
                Position::from((row, 42)),
                pluck::<100, 42, 1, 57>(chars),
            )),
            keywords: FieldResult::from((
                Position::from((row, 43)),
                pluck::<100, 43, 57, 0>(chars),
            )),
        }
    }
}

impl PMixin for FSpecLine {
    fn span(&self) -> Span {
        let start = self.sequence.span();
        let end = self.keywords.span();
        Span::from((start, end))
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = vec![];
        out.append(&mut self.sequence.highlight());
        out.append(&mut self.form_type.highlight());
        out.append(&mut self.name.highlight());
        out.append(&mut self.filetype.highlight());
        out.append(&mut self.file_designation.highlight());
        out.append(&mut self.endfile.highlight());
        out.append(&mut self.file_addition.highlight());
        out.append(&mut self.file_sequence.highlight());
        out.append(&mut self.file_format.highlight());
        out.append(&mut self.record_length.highlight());
        out.append(&mut self.limits_processing.highlight());
        out.append(&mut self.keylength.highlight());
        out.append(&mut self.record_address_type.highlight());
        out.append(&mut self.file_organization.highlight());
        out.append(&mut self.device.highlight());
        out.append(&mut self.reserved.highlight());
        out.append(&mut self.keywords.highlight());
        out
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSpecLineContinuation {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub nothing: FieldResult<NothingField>,
    pub keywords: FieldResult<FKeywordsField>,
}

impl Display for FSpecLineContinuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.sequence.to_string());
        msg.push_str(&self.form_type.to_string());
        msg.push_str(&self.nothing.to_string());
        msg.push_str(&self.keywords.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for FSpecLineContinuation {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let chars = value.1;
        Self {
            sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
            nothing: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 37, 57>(chars))),
            keywords: FieldResult::from((
                Position::from((row, 43)),
                pluck::<100, 43, 57, 0>(chars),
            )),
        }
    }
}

impl FSpecLineContinuation {
    pub fn to_raw(&self) -> (usize, [char; 100]) {
        let start = match &self.sequence {
            FieldResult::Ok(fld) => fld.meta.span.start.row,
            FieldResult::Idk(fld) => fld.meta.span.start.row,
        };
        let txt = self.to_string();
        let chars = txt.chars().collect::<Vec<char>>();
        let msg = "Expect FSpecLineContinuation.to_string() to yield exactly 100 chars";
        let chars100: [char; 100] = chars.try_into().expect(msg);
        (start, chars100)
    }
}

impl PMixin for FSpecLineContinuation {
    fn span(&self) -> Span {
        let start = self.sequence.span();
        let end = self.keywords.span();
        Span::from((start, end))
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = vec![];
        out.append(&mut self.sequence.highlight());
        out.append(&mut self.form_type.highlight());
        out.append(&mut self.keywords.highlight());
        out
    }
}
