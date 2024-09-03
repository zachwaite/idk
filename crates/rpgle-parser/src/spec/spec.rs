use super::c_spec::CSpec;
use super::comment_spec::CommentSpec;
use super::d_spec::DSpec;
use super::f_spec::FSpec;
use super::h_spec::HSpec;
use super::idk_spec::IdkSpec;
use crate::field::Field;
use crate::meta::Span;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub enum Spec {
    Idk(IdkSpec),
    Comment(CommentSpec),
    H(HSpec),
    F(FSpec),
    D(DSpec),
    C(CSpec),
}

impl Display for Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Idk(spec) => spec.to_string(),
            Self::Comment(spec) => spec.to_string(),
            Self::H(spec) => spec.to_string(),
            Self::F(spec) => spec.to_string(),
            Self::D(spec) => spec.to_string(),
            Self::C(spec) => spec.to_string(),
        };
        write!(f, "{}", msg)
    }
}

impl Spec {
    pub fn kind(&self) -> String {
        match self {
            Self::Idk(_) => "IdkSpec".to_string(),
            Self::Comment(_) => "CommentSpec".to_string(),
            Self::H(_) => "HSpec".to_string(),
            Self::F(_) => "FSpec".to_string(),
            Self::D(_) => "DSpec".to_string(),
            Self::C(_) => "CSpec".to_string(),
        }
    }
}

impl Field for Spec {
    fn highlight(&self) -> Vec<(Span, String)> {
        match self {
            Spec::Idk(spec) => spec.highlight(),
            Spec::Comment(spec) => spec.highlight(),
            Spec::H(spec) => spec.highlight(),
            Spec::F(spec) => spec.highlight(),
            Spec::D(spec) => spec.highlight(),
            Spec::C(spec) => spec.highlight(),
        }
    }

    fn span(&self) -> Span {
        match self {
            Spec::Idk(spec) => spec.span(),
            Spec::Comment(spec) => spec.span(),
            Spec::H(spec) => spec.span(),
            Spec::F(spec) => spec.span(),
            Spec::D(spec) => spec.span(),
            Spec::C(spec) => spec.span(),
        }
    }
}
