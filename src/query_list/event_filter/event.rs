use std::fmt;

#[derive(Clone)]
pub struct Event {
    id: u32,
}

impl Event {
    pub fn new(id: u32) -> Event {
        Event { id: id }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventID = {}", self.id)
    }
}
