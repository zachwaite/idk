use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("({}, {})", self.row, self.col);
        write!(f, "{}", s)
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            row: value.0,
            col: value.1,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Eq for Position {}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.row < other.row {
            Some(Ordering::Less)
        } else if self.row == other.row {
            if self.col < other.col {
                Some(Ordering::Less)
            } else if self.col == other.col {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.row < other.row {
            Ordering::Less
        } else if self.row == other.row {
            if self.col < other.col {
                Ordering::Less
            } else if self.col == other.col {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Greater
        }
    }
}

impl Position {
    pub fn empty() -> Self {
        Self { row: 0, col: 0 }
    }

    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}
