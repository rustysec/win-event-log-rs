use crate::query_list::Comparison;
use std::fmt;

#[derive(Clone)]
pub struct Level {
    level: u32,
    comparison: Comparison,
}

impl Level {
    pub fn new(level: u32, comparison: Comparison) -> Level {
        Level {
            level: level,
            comparison: comparison,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Level {} {}", self.comparison, self.level)
    }
}
