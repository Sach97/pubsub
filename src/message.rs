use std::{
    fmt::{Display, Error, Formatter},
    result::Result,
};

#[derive(Clone)]
pub struct Message {
    body: String,
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.body)
    }
}

impl Message {
    pub fn new(body: &str) -> Message {
        Message {
            body: String::from(body),
        }
    }
}
