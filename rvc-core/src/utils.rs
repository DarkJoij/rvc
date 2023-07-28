use std::fmt::{Display, Formatter, Result};

pub struct SnakeCased(pub String);

impl Display for SnakeCased {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut chars = self.0.chars();
        match chars.next() {
            Some(letter) => {
                let letter = letter.to_lowercase();
                write!(f, "{letter}")?;
            }
            None => return Ok(()),
        }

        for ch in chars {
            if ch.is_uppercase() {
                let ch = ch.to_lowercase();
                write!(f, "_{ch}")?;
            } else {
                write!(f, "{ch}")?;
            }
        }

        Ok(())
    }
}
