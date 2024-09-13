use super::comment_spec::CommentSpec;
use super::d_spec::DSpec;
use super::f_spec::FSpec;
use super::h_spec::HSpec;
use super::idk_spec::IdkSpec;
use super::{c_spec::CSpec, CompilerDirectiveSpec};
use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Spec {
    Idk(IdkSpec),
    Comment(CommentSpec),
    CompilerDirective(CompilerDirectiveSpec),
    H(HSpec),
    F(FSpec),
    D(DSpec),
    C(CSpec),
}
impl Spec {
    pub fn kind(&self) -> String {
        match self {
            Self::Idk(_) => "IdkSpec".to_string(),
            Self::Comment(_) => "CommentSpec".to_string(),
            Self::CompilerDirective(_) => "CompilerDirectiveSpec".to_string(),
            Self::H(_) => "HSpec".to_string(),
            Self::F(_) => "FSpec".to_string(),
            Self::D(_) => "DSpec".to_string(),
            Self::C(_) => "CSpec".to_string(),
        }
    }
    pub fn dspec(&self) -> Option<&DSpec> {
        match self {
            Self::D(spec) => Some(spec),
            _ => None,
        }
    }
}
impl PMixin for Spec {
    fn highlight(&self) -> Vec<(Span, String)> {
        match self {
            Spec::Idk(spec) => spec.highlight(),
            Spec::Comment(spec) => spec.highlight(),
            Spec::CompilerDirective(spec) => spec.highlight(),
            Spec::H(spec) => spec.highlight(),
            Spec::F(spec) => spec.highlight(),
            Spec::D(spec) => spec.highlight(),
            Spec::C(spec) => spec.highlight(),
        }
    }

    fn span(&self) -> Span {
        todo!()
    }
}
