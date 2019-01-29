use std::fmt;

#[derive(Clone)]
pub struct Computer {
    name: String,
}

impl Computer {
    pub fn new<T: Into<String>>(name: T) -> Computer {
        Computer { name: name.into() }
    }
}

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "System[@Computer = '{}']", self.name)
    }
}
