use std::fmt;

mod condition;
mod event_filter;
mod query_item;

pub use self::{
    condition::Condition,
    event_filter::EventFilter,
    query_item::{QueryItem, QueryItemType},
};

#[derive(Clone)]
/// Comparison conditions supported by the Windows Event Log
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

impl Default for QueryList {
    fn default() -> Self {
        Self {
            queries: Vec::new(),
        }
    }
}

impl<'a> QueryList {
    /// Create a new `QueryList`
    pub fn new() -> QueryList {
        Self::default()
    }

    /// Add a `Query` to a `QueryList`
    pub fn with_query(&'a mut self, query: Query) -> &'a mut Self {
        self.queries.push(query);
        self
    }

    /// Prepare `QueryList` for use
    pub fn build(&self) -> QueryList {
        QueryList {
            queries: self.queries.clone(),
        }
    }
}

impl fmt::Display for QueryList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<QueryList>")?;
        for (index, query) in (*self.queries).iter().enumerate() {
            write!(f, "\n<Query Id=\"{}\">\n", index)?;
            write!(f, "{}", query)?;
            write!(f, "</Query>")?;
        }
        write!(f, "\n</QueryList>")
    }
}

impl From<QueryList> for String {
    fn from(ql: QueryList) -> Self {
        ql.to_string()
    }
}

#[derive(Clone)]
pub struct Query {
    items: Vec<QueryItem>,
}

impl Default for Query {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl<'a> Query {
    /// Create a new `Query`
    pub fn new() -> Query {
        Self::default()
    }

    /// Add `QueryItem` to `Query`
    pub fn item(&'a mut self, item: QueryItem) -> &'a mut Self {
        self.items.push(item);
        self
    }

    /// Produce a `Query` from the builder
    pub fn query(&self) -> Query {
        Query {
            items: self.items.clone(),
        }
    }
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for item in (*self.items).iter() {
            writeln!(f, "{}", item)?;
        }
        Ok(())
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
                    .item(
                        QueryItem::new(QueryItemType::Selector, "Application".to_owned())
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
        use crate::prelude::*;
        let conditions = vec![
            Condition::filter(EventFilter::level(1, Comparison::Equal)),
            Condition::filter(EventFilter::level(4, Comparison::GreaterThanOrEqual)),
        ];
        let list = QueryList::new()
            .with_query(
                Query::new()
                    .item(
                        QueryItem::new(QueryItemType::Selector, "Application".to_owned())
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
        use crate::prelude::*;
        let system_conditions = vec![
            Condition::filter(EventFilter::level(0, Comparison::Equal)),
            Condition::filter(EventFilter::level(4, Comparison::GreaterThanOrEqual)),
        ];
        let event_conditions = Condition::filter(EventFilter::event_data(
            "TargetUserName".to_owned(),
            "SYSTEM".to_owned(),
        ));
        let list = QueryList::new()
            .with_query(
                Query::new()
                    .item(
                        QueryItem::new(QueryItemType::Selector, "Security".to_owned())
                            .system_conditions(Condition::or(system_conditions))
                            .event_conditions(event_conditions)
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
*[EventData[((Data[@Name = 'TargetUserName'] and Data = 'SYSTEM'))]]
</Select>
</Query>
</QueryList>"#
        );
    }
}
