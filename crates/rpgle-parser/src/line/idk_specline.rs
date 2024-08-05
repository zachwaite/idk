use crate::{field::IdkField, meta::Position};
use std::fmt::Display;

pub struct IdkSpecLine {
    idk: IdkField,
}

impl Display for IdkSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = "TODO: IdkSpec";
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for IdkSpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        let txt = chars.iter().collect::<String>();
        let idk = IdkField::from((start, txt.as_str()));
        Self { idk }
    }
}
