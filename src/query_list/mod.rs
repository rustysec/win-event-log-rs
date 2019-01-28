#[allow(dead_code)]
use std::fmt;

mod condition;
mod event_filter;
mod selector;
mod suppressor;

pub use self::condition::Condition;
pub use self::event_filter::EventFilter;
pub use self::selector::Selector;
pub use self::suppressor::Suppressor;

#[derive(Clone)]
pub enum Comparison {
    Equal,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl std::fmt::Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Comparison::Equal => write!(f, "="),
            Comparison::GreaterThan => write!(f, ">"),
            Comparison::LessThan => write!(f, "<"),
            Comparison::GreaterThanOrEqual => write!(f, ">="),
            Comparison::LessThanOrEqual => write!(f, "<="),
        }
    }
}

#[derive(Clone)]
pub struct QueryList {
    queries: Vec<Query>,
}

impl<'a> QueryList {
    pub fn new() -> QueryList {
        QueryList {
            queries: Vec::new(),
        }
    }

    pub fn with_query(&'a mut self, query: Query) -> &'a mut Self {
        self.queries.push(query);
        self
    }

    pub fn build(&self) -> QueryList {
        QueryList {
            queries: self.queries.clone(),
        }
    }
}

impl fmt::Display for QueryList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut index = 0;
        write!(f, "<QueryList>")?;
        for query in (*self.queries).iter() {
            write!(f, "\n<Query Id=\"{}\">\n", index)?;
            write!(f, "{}", query)?;
            write!(f, "</Query>")?;
            index += 1;
        }
        write!(f, "\n</QueryList>")
    }
}

impl Into<String> for QueryList {
    fn into(self) -> String {
        self.to_string()
    }
}

#[derive(Clone)]
pub struct Query {
    items: Vec<SelectOrSuppress>,
}

impl<'a> Query {
    pub fn new() -> Query {
        Query { items: Vec::new() }
    }

    pub fn select(&'a mut self, selector: selector::Selector) -> &'a mut Self {
        self.items.push(SelectOrSuppress::Select(selector));
        self
    }

    pub fn suppress(&'a mut self, suppressor: Suppressor) -> &'a mut Self {
        self.items.push(SelectOrSuppress::Suppress(suppressor));
        self
    }

    pub fn query(&self) -> Query {
        Query {
            items: self.items.clone(),
        }
    }
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for item in (*self.items).iter() {
            write!(f, "{}\n", item)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
enum SelectOrSuppress {
    Select(Selector),
    Suppress(Suppressor),
}

impl std::fmt::Display for SelectOrSuppress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SelectOrSuppress::Select(ref s) => write!(f, "{}", s),
            SelectOrSuppress::Suppress(ref s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_query() {
        use crate::prelude::*;
        let list = QueryList::new()
            .with_query(
                Query::new()
                    .select(
                        Selector::new("Application".to_owned())
                            .system_conditions(Condition::filter(EventFilter::level(
                                1,
                                Comparison::Equal,
                            )))
                            .build(),
                    )
                    .query(),
            )
            .build();

        assert_eq!(
            &list.to_string(),
            r#"<QueryList>
<Query Id="0">
<Select Path="Application">
*[System[(Level = 1)]]
</Select>
</Query>
</QueryList>"#
        );
    }

    #[test]
    fn simple_or_query() {
        use crate::{Comparison, Condition, EventFilter, Query, QueryList, Selector};
        let conditions = vec![
            Condition::filter(EventFilter::level(1, Comparison::Equal)),
            Condition::filter(EventFilter::level(4, Comparison::GreaterThanOrEqual)),
        ];
        let list = QueryList::new()
            .with_query(
                Query::new()
                    .select(
                        Selector::new("Application".to_owned())
                            .system_conditions(Condition::or(conditions))
                            .build(),
                    )
                    .query(),
            )
            .build();
        assert_eq!(
            &list.to_string(),
            r#"<QueryList>
<Query Id="0">
<Select Path="Application">
*[System[((Level = 1) or (Level >= 4))]]
</Select>
</Query>
</QueryList>"#
        );
    }

    #[test]
    fn system_and_event_query() {
        use crate::{Comparison, Condition, EventFilter, Query, QueryList, Selector};
        let system_conditions = vec![
            Condition::filter(EventFilter::level(0, Comparison::Equal)),
            Condition::filter(EventFilter::level(4, Comparison::GreaterThanOrEqual)),
        ];
        let event_conditions = vec![
            Condition::filter(EventFilter::name("TargetUserName".to_owned())),
            Condition::filter(EventFilter::value("SYSTEM".to_owned())),
        ];
        let list = QueryList::new()
            .with_query(
                Query::new()
                    .select(
                        Selector::new("Security".to_owned())
                            .system_conditions(Condition::or(system_conditions))
                            .event_conditions(Condition::and(event_conditions))
                            .build(),
                    )
                    .query(),
            )
            .build();

        assert_eq!(
            &list.to_string(),
            r#"<QueryList>
<Query Id="0">
<Select Path="Security">
*[System[((Level = 0) or (Level >= 4))]]
and
*[EventData[((Data[@Name = 'TargetUserName']) and (Data = 'SYSTEM'))]]
</Select>
</Query>
</QueryList>"#
        );
    }
}
