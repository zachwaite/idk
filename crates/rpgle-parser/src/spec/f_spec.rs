use crate::field::{
    DeviceField, EndfileField, FKeywordsField, FieldResult, FileAdditionField,
    FileDesignationField, FileFormatField, FileOrganizationField, FileSequenceField, FiletypeField,
    FormtypeField, KeyLengthField, LimitsProcessingField, NameField, RecordAddressTypeField,
    RecordLengthField, ReservedField, SequenceField,
};
use crate::free::tokenize_fspec_kw;
use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::{FSpecLine, FSpecLineContinuation};

#[derive(Debug, Serialize, Deserialize)]
pub struct FSpec {
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

impl FSpec {
    pub fn highlight(&self) -> Vec<(Span, String)> {
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

impl From<(&FSpecLine, Vec<&FSpecLineContinuation>)> for FSpec {
    fn from(value: (&FSpecLine, Vec<&FSpecLineContinuation>)) -> Self {
        let line = value.0;
        let continuations = value.1;

        let tokens = tokenize_fspec_kw(line, continuations);
        let kwfield = FKeywordsField { tokens };

        Self {
            sequence: line.sequence.clone(),
            form_type: line.form_type.clone(),
            name: line.name.clone(),
            filetype: line.filetype.clone(),
            file_designation: line.file_designation.clone(),
            endfile: line.endfile.clone(),
            file_addition: line.file_addition.clone(),
            file_sequence: line.file_sequence.clone(),
            file_format: line.file_format.clone(),
            record_length: line.record_length.clone(),
            limits_processing: line.limits_processing.clone(),
            keylength: line.keylength.clone(),
            record_address_type: line.record_address_type.clone(),
            file_organization: line.file_organization.clone(),
            device: line.device.clone(),
            reserved: line.reserved.clone(),
            keywords: FieldResult::Ok(kwfield),
        }
    }
}
