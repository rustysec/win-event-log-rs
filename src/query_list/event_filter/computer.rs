use std::fmt;

#[derive(Clone)]
pub struct Computer {
    name: String,
}

impl Computer {
    pub fn new(name: String) -> Computer {
        Computer { name: name }
    }
}

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Security[@Computer = '{}']", self.name)
    }
}
