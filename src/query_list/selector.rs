use std::fmt;

use crate::query_list::condition;

#[derive(Clone)]
pub struct Selector {
    path: Option<String>,
    system_conditions: Option<condition::Condition>,
    event_data_conditions: Option<condition::Condition>,
}

impl<'a> Selector {
    pub fn new(path: String) -> Selector {
        Selector {
            path: Some(path),
            system_conditions: None,
            event_data_conditions: None,
        }
    }

    pub fn system_conditions(&'a mut self, conditions: condition::Condition) -> &'a mut Self {
        self.system_conditions = Some(conditions);
        self
    }

    pub fn event_conditions(&'a mut self, conditions: condition::Condition) -> &'a mut Self {
        self.event_data_conditions = Some(conditions);
        self
    }

    pub fn build(&self) -> Selector {
        Selector {
            path: self.path.clone(),
            system_conditions: self.system_conditions.clone(),
            event_data_conditions: self.event_data_conditions.clone(),
        }
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.path {
            Some(ref path) => {
                let mut parts = Vec::new();
                write!(f, "<Select Path=\"{}\">\n", path)?;
                if let Some(ref conditions) = self.system_conditions {
                    parts.push(format!("*[System[{}]]", conditions))
                }
                if let Some(ref conditions) = self.event_data_conditions {
                    parts.push(format!("*[EventData[{}]]", conditions))
                }
                write!(f, "{}", parts.join("\nand\n"))?;
                write!(f, "\n</Select>")
            }
            None => write!(f, ""),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_level() {
        use crate::{Comparison, Condition, EventFilter, Selector};
        let selector = Selector::new("Application".to_owned())
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
