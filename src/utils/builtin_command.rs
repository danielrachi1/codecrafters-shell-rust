use crate::error::InputError;

use super::path_finder::PathFinder;
use std::env::VarError;
use std::ops::ControlFlow;

#[derive(Debug)]
pub enum BuiltinCommand {
    Exit,
    Echo,
    Type,
    Run(String),
}

impl From<&str> for BuiltinCommand {
    fn from(value: &str) -> BuiltinCommand {
        match value {
            "exit" => BuiltinCommand::Exit,
            "echo" => BuiltinCommand::Echo,
            "type" => BuiltinCommand::Type,
            _ => BuiltinCommand::Run(value.to_string()),
        }
    }
}

impl std::fmt::Display for BuiltinCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinCommand::Exit => write!(f, "exit"),
            BuiltinCommand::Echo => write!(f, "echo"),
            BuiltinCommand::Type => write!(f, "type"),
            BuiltinCommand::Run(program) => write!(f, "run: {}", program),
        }
    }
}

impl BuiltinCommand {
    pub fn execute(self, args: Vec<String>) -> Result<ControlFlow<()>, InputError> {
        match self {
            BuiltinCommand::Exit => Ok(ControlFlow::Break(())),
            BuiltinCommand::Echo => {
                exec_echo(args);
                Ok(ControlFlow::Continue(()))
            }
            BuiltinCommand::Type => {
                exec_type(args)?;
                Ok(ControlFlow::Continue(()))
            }
            BuiltinCommand::Run(program) => {
                exec_run(program, args)?;
                Ok(ControlFlow::Continue(()))
            }
        }
    }
}

fn exec_echo(args: Vec<String>) {
    for arg in args {
        print!("{} ", arg);
    }
    println!();
}

fn exec_type(args: Vec<String>) -> Result<(), VarError> {
    for arg in args {
        match BuiltinCommand::from(arg.as_str()) {
            BuiltinCommand::Exit | BuiltinCommand::Echo | BuiltinCommand::Type => {
                println!("{} is a shell builtin", arg)
            }
            _ => {
                let finder = PathFinder::new(arg.clone())?;
                match finder.find_executable() {
                    Some(path) => println!("{} is {}", arg, path.display()),
                    None => println!("{}: not found", arg),
                }
            }
        }
    }
    Ok(())
}

fn exec_run(program: String, args: Vec<String>) -> Result<(), InputError> {
    // if none of this returns error, the program exists and has exec perms
    PathFinder::new(program.clone())?
        .find_executable()
        .ok_or(InputError::CommandNotFound(program.clone()))?;

    std::process::Command::new(program).args(args).status()?;

    Ok(())
}
