// Adapts SpecLine to SrcLine
use crate::field::{
    CommentField, CompilerDirectiveField, ControlLevelField, DatastructureTypeField, DatatypeField,
    DecimalsField, DefinitionTypeField, DeviceField, EndfileField, ExternalDescriptionField,
    Factor1Field, FieldResult, FileAdditionField, FileDesignationField, FileFormatField,
    FileOrganizationField, FileSequenceField, FiletypeField, FormtypeField, IdkField,
    IndicatorsField, KeyLengthField, LimitsProcessingField, NameField, NothingField,
    OperationField, POSField, RawCodeField, RawFactor2Field, RawKeywordsField,
    RecordAddressTypeField, RecordLengthField, ReservedField, ResultField, ResultLengthField,
    SequenceField,
};
use crate::line::{CSpecLine, SpecLine};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

pub fn srcline_from_specline(specline: &SpecLine) -> Srcline {
    match specline {
        SpecLine::Idk(line) => Srcline::Idk {
            idk: line.idk.clone(),
        },
        SpecLine::Comment(line) => Srcline::Comment {
            sequence: line.sequence.clone(),
            form_type: line.form_type.clone(),
            comment: line.comment.clone(),
        },
        SpecLine::CompilerDirective(line) => Srcline::CompilerDirective {
            sequence: line.sequence.clone(),
            form_type: line.form_type.clone(),
            directive: line.directive.clone(),
        },
        SpecLine::HSpec(line) => Srcline::H {
            sequence: line.sequence.clone(),
            form_type: line.form_type.clone(),
            keywords: line.keywords.clone(),
        },
        SpecLine::FSpec(line) => Srcline::F {
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
            keywords: line.keywords.clone(),
        },
        SpecLine::FSpecContinuation(line) => Srcline::FCont {
            sequence: line.sequence.clone(),
            form_type: line.form_type.clone(),
            nothing: line.nothing.clone(),
            keywords: line.keywords.clone(),
        },
        SpecLine::DSpec(line) => Srcline::D {
            sequence: line.sequence.clone(),
            form_type: line.form_type.clone(),
            name: line.name.clone(),
            external_description: line.external_description.clone(),
            datastructure_type: line.datastructure_type.clone(),
            definition_type: line.definition_type.clone(),
            from_position: line.from_position.clone(),
            to_length: line.to_length.clone(),
            datatype: line.datatype.clone(),
            decimals: line.decimals.clone(),
            reserved: line.reserved.clone(),
            keywords: line.keywords.clone(),
        },
        SpecLine::DSpecContinuation(line) => Srcline::DCont {
            sequence: line.sequence.clone(),
            form_type: line.form_type.clone(),
            nothing: line.nothing.clone(),
            keywords: line.keywords.clone(),
        },
        SpecLine::CSpec(line) => match line {
            CSpecLine::Traditional(line) => Srcline::C(CSrcline::Traditional {
                nothing: line.nothing.clone(),
                form_type: line.form_type.clone(),
                control_level: line.control_level.clone(),
                indicators: line.indicators.clone(),
                factor1: line.factor1.clone(),
                operation: line.operation.clone(),
                factor2: line.factor2.clone(),
                result: line.result.clone(),
                result_length: line.result_length.clone(),
                decimals: line.decimals.clone(),
                resulting_indicators: line.resulting_indicators.clone(),
                comments: line.comments.clone(),
            }),
            CSpecLine::ExtF2(line) => Srcline::C(CSrcline::ExtF2 {
                nothing: line.nothing.clone(),
                form_type: line.form_type.clone(),
                control_level: line.control_level.clone(),
                indicators: line.indicators.clone(),
                factor1: line.factor1.clone(),
                operation: line.operation.clone(),
                factor2: line.factor2.clone(),
            }),
            CSpecLine::Free(line) => Srcline::C(CSrcline::Free {
                nothing: line.nothing.clone(),
                code: line.code.clone(),
            }),
        },
    }
}
