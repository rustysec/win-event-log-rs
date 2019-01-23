use crate::Condition;
use std::fmt;

#[derive(Clone)]
pub struct Suppressor {
    path: Option<String>,
    conditions: Option<Condition>,
}

impl Suppressor {
    pub fn new(path: String) -> Suppressor {
        Suppressor {
            path: Some(path),
            conditions: None,
        }
    }
}

impl fmt::Display for Suppressor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.path {
            Some(ref path) => {
                let mut parts = Vec::new();
                parts.push(format!("<Suppress Path=\"{}\">", path));
                if let Some(ref conditions) = self.conditions {
                    parts.push(format!("*[System[{}]]", conditions))
                }
                parts.push(format!("</Suppress>"));
                write!(f, "{}", parts.join("\n"))
            }
            None => write!(f, ""),
        }
    }
}
