use super::core::AST;
use crate::cst::{ParserException, CST};
use crate::field::IdkField;
use crate::line::{CSpecLine, HSpecLine, IdkSpecLine, SpecLine};
use crate::meta::{Meta, Span};
use crate::spec::{
    CSpec, CompilerDirectiveSpec as CompilerDirective, DSpec, FSpec, HSpec, IdkSpec, Spec,
};
use crate::FieldResult;
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::multi::many0;
use nom::IResult;

#[derive(Debug)]
pub enum ParserException {
    EmptyInput,
}

// fn hspec<'a>(head: &'a SpecLine, rest: &'a [SpecLine]) -> IResult<&'a [SpecLine], HSpec> {
//     todo!()
// }
//
// fn dspec<'a>(head: &'a SpecLine, rest: &'a [SpecLine]) -> IResult<&'a [SpecLine], DSpec> {
//     todo!()
// }
//
// fn fspec<'a>(head: &'a SpecLine, rest: &'a [SpecLine]) -> IResult<&'a [SpecLine], FSpec> {
//     todo!()
// }
//
// fn cspec<'a>(head: &'a SpecLine, rest: &'a [SpecLine]) -> IResult<&'a [SpecLine], CSpec> {
//     todo!()
// }
//
// fn compiler_directive<'a>(
//     head: &'a SpecLine,
//     rest: &'a [SpecLine],
// ) -> IResult<&'a [SpecLine], CompilerDirective> {
//     todo!()
// }
//
// fn idk<'a>(head: &'a SpecLine, rest: &'a [SpecLine]) -> IResult<&'a [SpecLine], Spec> {
//     todo!()
// }

fn dummy<'a>(_: &'a SpecLine, rest: &'a [SpecLine]) -> IResult<&'a [SpecLine], Spec> {
    let meta = Meta {
        span: Span::empty(),
        text: "".to_string(),
    };
    let fld = IdkField {
        value: "".to_string(),
        meta,
    };
    let frs = FieldResult::Idk(fld);
    let specline = IdkSpecLine { idk: frs };
    let idkspec = IdkSpec { line: specline };
    Ok((rest, Spec::Idk(idkspec)))
}

fn spec(input: &[SpecLine]) -> IResult<&[SpecLine], Spec> {
    let (head, rest) = input
        .split_first()
        .ok_or(Err(ParserException::EmptyInput))?;
    // alt((hspec, dspec, fspec, cspec, compiler_directive, idk))(head, rest)
    dummy(head, rest)
}

fn ast(input: &[SpecLine]) -> IResult<&[SpecLine], Spec> {
    many0(spec)
}

pub fn parse_ast(cst: &CST) -> AST {
    AST { specs: vec![] }
}
