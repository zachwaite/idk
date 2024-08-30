use crate::field::{
    CodeField, CommentField, ControlLevelField, DecimalsField, Factor1Field, Factor2Field,
    FieldResult, FormtypeField, IndicatorsField, NothingField, OperationField, ResultField,
    ResultLengthField,
};
use crate::meta::pluck_array3 as pluck;
use crate::meta::Position;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionalCSpecLine {
    pub nothing: FieldResult<NothingField>,
    pub form_type: FieldResult<FormtypeField>,
    pub control_level: FieldResult<ControlLevelField>,
    pub indicators: FieldResult<IndicatorsField>,
    pub factor1: FieldResult<Factor1Field>,
    pub operation: FieldResult<OperationField>,
    pub factor2: FieldResult<Factor2Field>,
    pub result: FieldResult<ResultField>,
    pub result_length: FieldResult<ResultLengthField>,
    pub decimals: FieldResult<DecimalsField>,
    pub resulting_indicators: FieldResult<IndicatorsField>,
    pub comments: FieldResult<CommentField>,
}

impl From<(usize, &[char; 100])> for TraditionalCSpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        let line = TraditionalCSpecLine {
            nothing: FieldResult::from((start, pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((start, pluck::<100, 5, 1, 94>(chars))),
            control_level: FieldResult::from((start, pluck::<100, 6, 2, 92>(chars))),
            indicators: FieldResult::from((start, pluck::<100, 8, 3, 89>(chars))),
            factor1: FieldResult::from((start, pluck::<100, 11, 13, 76>(chars))),
            operation: FieldResult::from((start, pluck::<100, 24, 10, 66>(chars))),
            factor2: FieldResult::from((start, pluck::<100, 34, 15, 51>(chars))),
            result: FieldResult::from((start, pluck::<100, 49, 13, 38>(chars))),
            result_length: FieldResult::from((start, pluck::<100, 62, 5, 33>(chars))),
            decimals: FieldResult::from((start, pluck::<100, 67, 2, 31>(chars))),
            resulting_indicators: FieldResult::from((start, pluck::<100, 69, 5, 26>(chars))),
            comments: FieldResult::from((start, pluck::<100, 74, 26, 0>(chars))),
        };
        line
    }
}

impl Display for TraditionalCSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.nothing.to_string());
        msg.push_str(&self.form_type.to_string());
        msg.push_str(&self.control_level.to_string());
        msg.push_str(&self.indicators.to_string());
        msg.push_str(&self.factor1.to_string());
        msg.push_str(&self.operation.to_string());
        msg.push_str(&self.factor2.to_string());
        msg.push_str(&self.result.to_string());
        msg.push_str(&self.result_length.to_string());
        msg.push_str(&self.decimals.to_string());
        msg.push_str(&self.resulting_indicators.to_string());
        msg.push_str(&self.comments.to_string());
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtF2CSpecLine {
    pub nothing: FieldResult<NothingField>,
    pub form_type: FieldResult<FormtypeField>,
    pub control_level: FieldResult<ControlLevelField>,
    pub indicators: FieldResult<IndicatorsField>,
    pub factor1: FieldResult<Factor1Field>,
    pub operation: FieldResult<OperationField>,
    pub factor2: FieldResult<Factor2Field>,
}

impl Display for ExtF2CSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.nothing.to_string());
        msg.push_str(&self.form_type.to_string());
        msg.push_str(&self.control_level.to_string());
        msg.push_str(&self.indicators.to_string());
        msg.push_str(&self.factor1.to_string());
        msg.push_str(&self.operation.to_string());
        msg.push_str(&self.factor2.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for ExtF2CSpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        let line = ExtF2CSpecLine {
            nothing: FieldResult::from((start, pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((start, pluck::<100, 5, 1, 94>(chars))),
            control_level: FieldResult::from((start, pluck::<100, 6, 2, 92>(chars))),
            indicators: FieldResult::from((start, pluck::<100, 8, 3, 89>(chars))),
            factor1: FieldResult::from((start, pluck::<100, 11, 13, 76>(chars))),
            operation: FieldResult::from((start, pluck::<100, 24, 10, 66>(chars))),
            factor2: FieldResult::from((start, pluck::<100, 34, 66, 0>(chars))),
        };
        line
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeCSpecLine {
    pub nothing: FieldResult<NothingField>,
    pub code: FieldResult<CodeField>,
}

impl Display for FreeCSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.nothing.to_string());
        msg.push_str(&self.code.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for FreeCSpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        let line = FreeCSpecLine {
            nothing: FieldResult::from((start, pluck::<100, 0, 7, 93>(chars))),
            code: FieldResult::from((start, pluck::<100, 7, 93, 0>(chars))),
        };
        line
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CSpecLine {
    Traditional(TraditionalCSpecLine),
    ExtF2(ExtF2CSpecLine),
    Free(FreeCSpecLine),
}

// not implementing from at this time because it would necessarily require
// the implementation perform some sort of peek which would be redundant with
// the peek in SpecLine::from()

impl Display for CSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Traditional(line) => write!(f, "{}", line.to_string()),
            Self::ExtF2(line) => write!(f, "{}", line.to_string()),
            Self::Free(line) => write!(f, "{}", line.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtF2CSpecLineContinuation {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeCSpecLineContinuation {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CSpecLineContinuation {
    ExtF2(ExtF2CSpecLineContinuation),
    Free(FreeCSpecLineContinuation),
}

impl Display for CSpecLineContinuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        write!(f, "{}", msg)
    }
}
