use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Driver {
    pub lastname: String,
    pub firstname: String,
}

impl Display for Driver {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "lastname: {}, firstname: {}",
            self.lastname, self.firstname
        )
    }
}
