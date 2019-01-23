use std::fmt;

#[derive(Clone)]
pub struct Provider {
    name: String,
}

impl Provider {
    pub fn new(name: String) -> Provider {
        Provider { name: name }
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Provider[@Name = '{}']", self.name)
    }
}
