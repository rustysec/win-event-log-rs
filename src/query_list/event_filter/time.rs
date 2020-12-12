use crate::query_list::Comparison;
use std::fmt;
use std::time::SystemTime;

#[derive(Clone)]
pub struct TimeCreated {
    time: String,
    comparison: TimeComparison,
    time_diff: TimeDiff,
}

impl TimeCreated {
    pub fn new(time: &str, comparison: TimeComparison, time_diff: TimeDiff) -> Self {
        let time = match time_diff {
            TimeDiff::InLast => format!("{}", time),
            TimeDiff::InRange => format!("'{}'", time),
        };
        Self {
            time,
            comparison,
            time_diff,
        }
    }
}

impl fmt::Display for TimeCreated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TimeCreated[{} {} {}]",
            self.time_diff, self.comparison, self.time
        )
    }
}
#[derive(Clone)]
/// Comparison conditions supported by the Windows Event Log
pub enum TimeComparison {
    Equal,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl std::fmt::Display for TimeComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TimeComparison::Equal => write!(f, "="),
            TimeComparison::GreaterThan => write!(f, "&gt;"),
            TimeComparison::LessThan => write!(f, "&lt;"),
            TimeComparison::GreaterThanOrEqual => write!(f, "&gt;="),
            TimeComparison::LessThanOrEqual => write!(f, "&lt;="),
        }
    }
}
#[derive(Clone)]
/// Comparison conditions supported by the Windows Event Log
pub enum TimeDiff {
    InLast,
    InRange,
}

impl std::fmt::Display for TimeDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TimeDiff::InLast => write!(f, "timediff(@SystemTime)"),
            TimeDiff::InRange => write!(f, "@SystemTime"),
        }
    }
}
