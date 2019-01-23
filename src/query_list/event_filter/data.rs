use std::fmt;

#[derive(Clone)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: String) -> Name {
        Name { name: name }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Data[@Name = '{}']", self.name)
    }
}

#[derive(Clone)]
pub struct Value {
    value: String,
}

impl Value {
    pub fn new(value: String) -> Value {
        Value { value: value }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Data = '{}'", self.value)
    }
}
