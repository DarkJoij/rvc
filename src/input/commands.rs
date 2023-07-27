use crate::error;

#[cfg(debug_assertions)]
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum Command {
    Test
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        match value.as_str() {
            "test" => Self::Test,
            _ => error!("Unrecognized command: {value}.")
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string_of = self.to_string();
        write!(f, "{}", string_of.to_lowercase())
    }
}
