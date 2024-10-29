use super::core::AST;
use crate::cst::{ParserException, CST};
use crate::field::IdkField;
use crate::line::{CSpecLine, HSpecLine, IdkSpecLine, SpecLine};
use crate::meta::{partition, Meta, Span};
use crate::spec::{
    CSpec, CompilerDirectiveSpec as CompilerDirective, DSpec, FSpec, HSpec, IdkSpec, Spec,
};
use crate::FieldResult;

#[derive(Debug)]
pub enum ParseError {
    EmptyInput,
    Unhandled,
}

fn try_hspec(input: &[SpecLine]) -> Option<(Spec, &[SpecLine])> {
    todo!()
}

fn hspec(input: &[SpecLine]) -> Result<(Spec, &[SpecLine]), ParseError> {
    try_hspec(&input).ok_or(ParseError::Unhandled)
}

fn try_dspec(input: &[SpecLine]) -> Option<(Spec, &[SpecLine])> {
    todo!()
}

fn dspec(input: &[SpecLine]) -> Result<(Spec, &[SpecLine]), ParseError> {
    try_hspec(&input).ok_or(ParseError::Unhandled)
}

fn ast(input: &mut [SpecLine]) -> Result<(Vec<Spec>, &[SpecLine]), ParseError> {
    let (keep, ignore) = partition(&mut input, |line| matches(line, SpecLine::HLine{})));
}
