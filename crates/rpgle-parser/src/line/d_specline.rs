use crate::field::{
    DatastructureTypeField, DatatypeField, DecimalsField, DefinitionTypeField,
    ExternalDescriptionField, FieldResult, FormtypeField, KeywordsField, NameField, NothingField,
    POSField, ReservedField, SequenceField,
};
use crate::meta::pluck_array3 as pluck;
use crate::meta::Position;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSpecLine {
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
    pub keywords: FieldResult<KeywordsField>,
}

impl Display for DSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.sequence.to_string());
        msg.push_str(&self.form_type.to_string());
        msg.push_str(&self.name.to_string());
        msg.push_str(&self.external_description.to_string());
        msg.push_str(&self.datastructure_type.to_string());
        msg.push_str(&self.definition_type.to_string());
        msg.push_str(&self.from_position.to_string());
        msg.push_str(&self.to_length.to_string());
        msg.push_str(&self.datatype.to_string());
        msg.push_str(&self.decimals.to_string());
        msg.push_str(&self.reserved.to_string());
        msg.push_str(&self.keywords.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for DSpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        Self {
            sequence: FieldResult::from((start, pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((start, pluck::<100, 5, 1, 94>(chars))),
            name: FieldResult::from((start, pluck::<100, 6, 15, 79>(chars))),
            external_description: FieldResult::from((start, pluck::<100, 21, 1, 78>(chars))),
            datastructure_type: FieldResult::from((start, pluck::<100, 22, 1, 77>(chars))),
            definition_type: FieldResult::from((start, pluck::<100, 23, 2, 75>(chars))),
            from_position: FieldResult::from((start, pluck::<100, 25, 7, 68>(chars))),
            to_length: FieldResult::from((start, pluck::<100, 32, 7, 61>(chars))),
            datatype: FieldResult::from((start, pluck::<100, 39, 1, 60>(chars))),
            decimals: FieldResult::from((start, pluck::<100, 40, 2, 58>(chars))),
            reserved: FieldResult::from((start, pluck::<100, 42, 1, 57>(chars))),
            keywords: FieldResult::from((start, pluck::<100, 43, 57, 0>(chars))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSpecLineContinuation {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub nothing: FieldResult<NothingField>,
    pub keywords: FieldResult<KeywordsField>,
}

impl Display for DSpecLineContinuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.sequence.to_string());
        msg.push_str(&self.form_type.to_string());
        msg.push_str(&self.nothing.to_string());
        msg.push_str(&self.keywords.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for DSpecLineContinuation {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        Self {
            sequence: FieldResult::from((start, pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((start, pluck::<100, 5, 1, 94>(chars))),
            nothing: FieldResult::from((start, pluck::<100, 6, 37, 57>(chars))),
            keywords: FieldResult::from((start, pluck::<100, 43, 57, 0>(chars))),
        }
    }
}

impl DSpecLineContinuation {
    pub fn to_raw(&self) -> (usize, [char; 100]) {
        let start = match &self.sequence {
            FieldResult::Ok(fld) => fld.meta.span.start.row,
            FieldResult::Idk(fld) => fld.meta.span.start.row,
        };
        let txt = self.to_string();
        let chars = txt.chars().collect::<Vec<char>>();
        let msg = "Expect DSpecLineContinuation.to_string() to yield exactly 100 chars";
        let chars100: [char; 100] = chars.try_into().expect(msg);
        (start, chars100)
    }
}
