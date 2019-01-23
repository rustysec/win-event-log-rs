use crate::query_list::event_filter::EventFilter;
use std::fmt;

#[derive(Clone)]
pub enum Condition {
    ConditionItem(EventFilter),
    ConditionOr(Vec<Condition>),
    ConditionAnd(Vec<Condition>),
}

impl Condition {
    pub fn filter(filter: EventFilter) -> Condition {
        Condition::ConditionItem(filter)
    }

    pub fn or(filters: Vec<Condition>) -> Condition {
        Condition::ConditionOr(filters)
    }

    pub fn and(filters: Vec<Condition>) -> Condition {
        Condition::ConditionAnd(filters)
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Condition::ConditionItem(item) => write!(f, "({})", item),
            Condition::ConditionAnd(items) => write!(
                f,
                "({})",
                items
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" and ")
            ),
            Condition::ConditionOr(items) => write!(
                f,
                "({})",
                items
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" or ")
            ),
        }
    }
}
