use crate::field::{
    DeviceField, EndfileField, FieldResult, FileAdditionField, FileDesignationField,
    FileFormatField, FileOrganizationField, FileSequenceField, FiletypeField, FormtypeField,
    IdkField, KeyLengthField, KeywordsField, LimitsProcessingField, NameField,
    RecordAddressTypeField, RecordLengthField, ReservedField, SequenceField,
};
use crate::meta::pluck_array3 as pluck;
use crate::meta::{Meta, Position};
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
    pub keywords: FieldResult<KeywordsField>,
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
        let start = Position::from((row, 0));
        let chars = value.1;
        Self {
            sequence: FieldResult::from((start, pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((start, pluck::<100, 5, 1, 94>(chars))),
            name: FieldResult::from((start, pluck::<100, 6, 10, 84>(chars))),
            filetype: FieldResult::from((start, pluck::<100, 16, 1, 83>(chars))),
            file_designation: FieldResult::from((start, pluck::<100, 17, 1, 82>(chars))),
            endfile: FieldResult::from((start, pluck::<100, 18, 1, 81>(chars))),
            file_addition: FieldResult::from((start, pluck::<100, 19, 1, 80>(chars))),
            file_sequence: FieldResult::from((start, pluck::<100, 20, 1, 79>(chars))),
            file_format: FieldResult::from((start, pluck::<100, 21, 1, 78>(chars))),
            record_length: FieldResult::from((start, pluck::<100, 22, 5, 73>(chars))),
            limits_processing: FieldResult::from((start, pluck::<100, 27, 1, 72>(chars))),
            keylength: FieldResult::from((start, pluck::<100, 28, 5, 67>(chars))),
            record_address_type: FieldResult::from((start, pluck::<100, 33, 1, 66>(chars))),
            file_organization: FieldResult::from((start, pluck::<100, 34, 1, 65>(chars))),
            device: FieldResult::from((start, pluck::<100, 35, 7, 58>(chars))),
            reserved: FieldResult::from((start, pluck::<100, 42, 1, 57>(chars))),
            keywords: FieldResult::from((start, pluck::<100, 43, 57, 0>(chars))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSpecLineContinuation {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub keywords: FieldResult<KeywordsField>,
}

impl Display for FSpecLineContinuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.sequence.to_string());
        msg.push_str(&self.form_type.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for FSpecLineContinuation {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        Self {
            sequence: FieldResult::from((start, pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((start, pluck::<100, 5, 1, 94>(chars))),
            keywords: FieldResult::from((start, pluck::<100, 6, 94, 0>(chars))),
        }
    }
}
