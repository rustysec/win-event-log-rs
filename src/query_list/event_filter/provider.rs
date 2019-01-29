use std::fmt;

#[derive(Clone)]
pub struct Provider {
    name: String,
}

impl Provider {
    pub fn new<T: Into<String>>(name: T) -> Provider {
        Provider { name: name.into() }
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Provider[@Name = '{}']", self.name)
    }
}
