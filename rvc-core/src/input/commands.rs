use crate::utils::SnakeCased;

use rvc::error;

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum Command {
    Test,
    LongTestCommand
}

impl Command {
    fn all_commands() -> [Command; 2] {
        [
            Command::Test,
            Command::LongTestCommand
        ]
    }
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        for command in Self::all_commands() {
            let snake_cased = SnakeCased(command.to_string());

            if value == snake_cased.to_string() {
                return command;
            }
        }

        error!("Unrecognized command: {value}.")
        
        /* 
        match value.as_str() {
            "test" => Command::Test,
            "long_test_command" => Command::LongTestCommand,
            _ => error!("Unrecognized command: {value}.")
        }
        */
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", SnakeCased(format!("{self:?}")))
    }
}
