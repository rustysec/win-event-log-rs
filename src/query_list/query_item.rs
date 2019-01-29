use std::fmt;

use crate::query_list::condition;

#[derive(Clone)]
pub enum QueryItemType {
    Suppressor,
    Selector,
}

impl fmt::Display for QueryItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryItemType::Suppressor => write!(f, "Suppress"),
            QueryItemType::Selector => write!(f, "Select"),
        }
    }
}

#[derive(Clone)]
pub struct QueryItem {
    query_item_type: QueryItemType,
    path: Option<String>,
    system_conditions: Option<condition::Condition>,
    event_data_conditions: Option<condition::Condition>,
}

impl<'a> QueryItem {
    pub fn new<T: Into<String>>(item_type: QueryItemType, path: T) -> QueryItem {
        QueryItem {
            query_item_type: item_type,
            path: Some(path.into()),
            system_conditions: None,
            event_data_conditions: None,
        }
    }

    pub fn selector<T: Into<String>>(path: T) -> QueryItem {
        QueryItem::new(QueryItemType::Selector, path)
    }

    pub fn suppressor<T: Into<String>>(path: T) -> QueryItem {
        QueryItem::new(QueryItemType::Suppressor, path)
    }

    pub fn system_conditions(&'a mut self, conditions: condition::Condition) -> &'a mut Self {
        self.system_conditions = Some(conditions);
        self
    }

    pub fn event_conditions(&'a mut self, conditions: condition::Condition) -> &'a mut Self {
        self.event_data_conditions = Some(conditions);
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl fmt::Display for QueryItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.path {
            Some(ref path) => {
                let mut parts = Vec::new();
                write!(f, "<{} Path=\"{}\">\n", self.query_item_type, path)?;
                if let Some(ref conditions) = self.system_conditions {
                    parts.push(format!("*[System[{}]]", conditions))
                }
                if let Some(ref conditions) = self.event_data_conditions {
                    parts.push(format!("*[EventData[{}]]", conditions))
                }
                write!(f, "{}", parts.join("\nand\n"))?;
                write!(f, "\n</{}>", self.query_item_type)
            }
            None => write!(f, ""),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_level() {
        use crate::prelude::*;
        let selector = QueryItem::new(QueryItemType::Selector, "Application")
            .system_conditions(Condition::filter(EventFilter::level(1, Comparison::Equal)))
            .build();
        assert_eq!(
            r#"<Select Path="Application">
*[System[(Level = 1)]]
</Select>"#,
            &selector.to_string()
        );
    }
}
