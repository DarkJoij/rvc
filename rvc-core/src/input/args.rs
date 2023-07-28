use super::commands::Command;

use rvc::error;

use std::env::args;
use std::fmt::{Debug, Display, Formatter, Result};

const USAGE: &str = "rvc <command> [options] [-flags] [--para=meters]";

fn trim_flag(mut flag: String) -> Arg {
    flag.remove(0);
    Arg::Flag(flag)
}

fn split_parameter(parameter: String) -> Arg {
    let parts = parameter.split('=')
        .collect::<Vec<&str>>();

    if parts.len() != 2 && parameter.len() > 50 { // This might be fatal error.
        error!("Invalid parameter: {parameter}");
    }

    let (left, right) = (parts[0], parts[1]);
    let mut owned_left = left.to_owned();

    owned_left.remove(0);
    owned_left.remove(0); 

    Arg::Parameter(owned_left, right.to_owned())
}

pub fn get_args() -> ParsedArgs {
    let mut argv = args()
        .collect::<Vec<String>>();

    argv.remove(0);
    if argv.len() == 0 {
        error!("You have passed too few arguments. Use the template: {USAGE}");
    }

    let command_string = argv.remove(0);
    let mut argc = Vec::new();

    for arg in argv { 
        let parsed = if arg.starts_with("--") { split_parameter(arg) } 
                else if arg.starts_with("-") { trim_flag(arg) } 
                else { Arg::Base(arg) };

        argc.push(parsed);
    }

    ParsedArgs { 
        argc, 
        command: Command::from(command_string)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParsedArgs {
    argc: Vec<Arg>,
    command: Command
}

#[derive(Debug)]
pub enum Arg {
    Base(String),
    Flag(String),
    Parameter(String, String)
}

impl Display for Arg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Arg::Base(arg) => write!(f, "{arg}"),
            Arg::Flag(flag) => write!(f, "-{flag}"),
            Arg::Parameter(left, right) => write!(f, "--{left}={right}"),
        }
    }
}
