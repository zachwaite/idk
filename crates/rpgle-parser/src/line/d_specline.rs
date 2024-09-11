use crate::field::{
    DKeywordsField, DatastructureTypeField, DatatypeField, DecimalsField, DefinitionTypeField,
    ExternalDescriptionField, FieldResult, FormtypeField, NameField, NothingField, POSField,
    ReservedField, SequenceField,
};
use crate::meta::pluck_array3 as pluck;
use crate::meta::{PMixin, Position, Span};
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
    pub keywords: FieldResult<DKeywordsField>,
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
        let chars = value.1;
        Self {
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
            to_length: FieldResult::from((
                Position::from((row, 32)),
                pluck::<100, 32, 7, 61>(chars),
            )),
            datatype: FieldResult::from((
                Position::from((row, 39)),
                pluck::<100, 39, 1, 60>(chars),
            )),
            decimals: FieldResult::from((
                Position::from((row, 40)),
                pluck::<100, 40, 2, 58>(chars),
            )),
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

impl PMixin for DSpecLine {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSpecLineContinuation {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub nothing: FieldResult<NothingField>,
    pub keywords: FieldResult<DKeywordsField>,
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
        let chars = value.1;
        Self {
            sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
            nothing: FieldResult::from((Position::from((row, 37)), pluck::<100, 6, 37, 57>(chars))),
            keywords: FieldResult::from((
                Position::from((row, 43)),
                pluck::<100, 43, 57, 0>(chars),
            )),
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

impl PMixin for DSpecLineContinuation {
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
