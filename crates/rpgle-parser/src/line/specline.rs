use super::idk_specline::IdkSpecLine;

enum SpecLine {
    Idk(IdkSpecLine),
}

impl From<(usize, &[char; 100])> for SpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let idx = value.0;
        let chars = value.1;
        let p6 = chars[6];
        match p6 {
            _ => {
                let line = IdkSpecLine::from((idx, chars));
                SpecLine::Idk(line)
            }
        }
    }
}
