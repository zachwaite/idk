// use super::c_spec::CSpec;
// use super::compilerdirective_spec::CompilerDirectiveSpec;
// use super::d_spec::DSpec;
// use super::f_spec::FSpec;
// use super::free_spec::FreeSpec;
// use super::fullfree_spec::FullFreeSpec;
// use super::h_spec::HSpec;
// use super::i_spec::ISpec;
// use super::idk_spec::IdkSpec;
// use super::o_spec::OSpec;
// use super::p_spec::PSpec;
use super::comment_spec::CommentSpec;
use super::idk_spec::IdkSpec;
use crate::line::SpecLine;
use std::fmt::Display;
//
// pub enum Spec {
//     FullFree(FullFreeSpec),
//     CompilerDirective(CompilerDirectiveSpec),
//     Idk(IdkSpec),
//     Free(FreeSpec),
//     FSpec(FSpec),
//     DSpec(DSpec),
//     CSpec(CSpec),
//     OSpec(OSpec),
//     PSpec(PSpec),
//     HSpec(HSpec),
//     ISpec(ISpec),
// }
//
pub enum Spec {
    Idk(IdkSpec),
    Comment(CommentSpec),
}

impl Display for Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Idk(spec) => spec.to_string(),
            Self::Comment(spec) => format!("{}", spec),
            // Self::FullFree(spec) => format!("{}", spec),
            // Self::CompilerDirective(spec) => format!("{}", spec),
            // Self::Free(spec) => format!("{}", spec),
            // Self::FSpec(spec) => format!("{}", spec),
            // Self::DSpec(spec) => format!("{}", spec),
            // Self::CSpec(spec) => format!("{}", spec),
            // Self::OSpec(spec) => format!("{}", spec),
            // Self::PSpec(spec) => format!("{}", spec),
            // Self::HSpec(spec) => format!("{}", spec),
            // Self::ISpec(spec) => format!("{}", spec),
        };
        write!(f, "{}", msg)
    }
}

impl Spec {
    pub fn kind(&self) -> String {
        match self {
            Self::Idk(_) => "IdkSpec".to_string(),
            Self::Comment(_) => "CommentSpec".to_string(),
        }
    }
}
