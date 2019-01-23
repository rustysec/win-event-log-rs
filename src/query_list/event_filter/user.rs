use std::fmt;

#[derive(Clone)]
pub struct User {
    sid: String,
}

impl User {
    pub fn new(sid: String) -> User {
        User { sid: sid }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Security[@UserID = '{}']", self.sid)
    }
}
