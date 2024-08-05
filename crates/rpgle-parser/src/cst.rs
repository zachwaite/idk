use crate::meta::Diagnostic;
use crate::spec::Spec;
use std::fmt::Display;

pub struct CST {
    pub specs: Vec<Spec>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Display for CST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "CST!")
    }
}

impl From<&str> for CST {
    fn from(txt: &str) -> Self {
        Self {
            specs: vec![],
            diagnostics: vec![],
        }
    }
}

// type L = [char; 100];
//
// let speclines = raw.split('\n')
//                    .map(|txt| { pad100chars(txt) })?
//                    .enumerate()
//                    .map(|i, chars| {SpecLine.from(i, chars)}) // type: L
//                    .collect::<Vec<SpecLine>>()

// let specs = speclines
//                .iter()
//                .fold(|acc, cur| { evolve(acc, cur) })
//                .collect::<Vec<Spec>>()

// impl From<(usize, L)> for DSpecLine {
//     fn from(value: (usize, L)) -> Self {
//         sequence: SequenceField::from(i, L::from_slice(chars[0..6])),
//         formtype: FormtypeField::from(i, L::from_slice(chars[6..7])),
//         ...
//      }
// }
