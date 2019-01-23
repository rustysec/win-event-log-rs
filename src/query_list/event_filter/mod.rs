use crate::query_list::Comparison;
use std::fmt;

pub mod computer;
pub mod data;
pub mod event;
pub mod level;
pub mod provider;
pub mod user;

#[derive(Clone)]
pub enum SystemFilter {
    Computer(computer::Computer),
    EventID(event::Event),
    Level(level::Level),
    Provider(provider::Provider),
    User(user::User),
}

#[derive(Clone)]
pub enum EventDataFilter {
    Name(data::Name),
    Value(data::Value),
}

#[derive(Clone)]
pub enum EventFilter {
    System(SystemFilter),
    EventData(EventDataFilter),
}

impl EventFilter {
    pub fn computer(name: String) -> EventFilter {
        EventFilter::System(SystemFilter::Computer(computer::Computer::new(name)))
    }

    pub fn event(id: u32) -> EventFilter {
        EventFilter::System(SystemFilter::EventID(event::Event::new(id)))
    }

    pub fn level(level: u32, comparison: Comparison) -> EventFilter {
        EventFilter::System(SystemFilter::Level(level::Level::new(level, comparison)))
    }

    pub fn provider(name: String) -> EventFilter {
        EventFilter::System(SystemFilter::Provider(provider::Provider::new(name)))
    }

    pub fn user(sid: String) -> EventFilter {
        EventFilter::System(SystemFilter::User(user::User::new(sid)))
    }

    pub fn name(name: String) -> EventFilter {
        EventFilter::EventData(EventDataFilter::Name(data::Name::new(name)))
    }

    pub fn value(value: String) -> EventFilter {
        EventFilter::EventData(EventDataFilter::Value(data::Value::new(value)))
    }
}

impl fmt::Display for EventFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventFilter::System(item) => write!(f, "{}", item),
            EventFilter::EventData(item) => write!(f, "{}", item),
        }
    }
}

impl fmt::Display for SystemFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SystemFilter::Computer(item) => write!(f, "{}", item),
            SystemFilter::EventID(item) => write!(f, "{}", item),
            SystemFilter::Level(item) => write!(f, "{}", item),
            SystemFilter::Provider(item) => write!(f, "{}", item),
            SystemFilter::User(item) => write!(f, "{}", item),
        }
    }
}

impl fmt::Display for EventDataFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventDataFilter::Name(item) => write!(f, "{}", item),
            EventDataFilter::Value(item) => write!(f, "{}", item),
        }
    }
}
