use crate::field::{
    DKeywordsField, DatastructureTypeField, DatatypeField, DecimalsField, DefinitionTypeField,
    ExternalDescriptionField, FieldResult, FormtypeField, NameField, POSField, ReservedField,
    SequenceField,
};
use crate::free::tokenize_dspec_kw;
use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};

use crate::line::{DSpecLine, DSpecLineContinuation};

#[derive(Debug, Serialize, Deserialize)]
pub struct DSpec {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub name: FieldResult<NameField>,
    pub external_description: FieldResult<ExternalDescriptionField>,
    pub datastructure_type: FieldResult<DatastructureTypeField>,
    pub definition_type: FieldResult<DefinitionTypeField>,
    pub from_position: FieldResult<POSField>,
    pub to_length: FieldResult<POSField>,
    pub datatype: FieldResult<DatatypeField>,
    pub decimals: FieldResult<DecimalsField>,
    pub reserved: FieldResult<ReservedField>,
    pub keywords: FieldResult<DKeywordsField>,
}

impl DSpec {
    pub fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = vec![];
        out.append(&mut self.sequence.highlight());
        out.append(&mut self.form_type.highlight());
        out.append(&mut self.name.highlight());
        out.append(&mut self.external_description.highlight());
        out.append(&mut self.datastructure_type.highlight());
        out.append(&mut self.definition_type.highlight());
        out.append(&mut self.from_position.highlight());
        out.append(&mut self.to_length.highlight());
        out.append(&mut self.datatype.highlight());
        out.append(&mut self.decimals.highlight());
        out.append(&mut self.reserved.highlight());
        out.append(&mut self.keywords.highlight());
        out
    }
}

impl From<(&DSpecLine, Vec<&DSpecLineContinuation>)> for DSpec {
    fn from(value: (&DSpecLine, Vec<&DSpecLineContinuation>)) -> Self {
        let line = value.0;
        let continuations = value.1;

        let tokens = tokenize_dspec_kw(line, continuations);
        let kwfield = DKeywordsField { tokens };

        Self {
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
            keywords: FieldResult::Ok(kwfield),
        }
    }
}
