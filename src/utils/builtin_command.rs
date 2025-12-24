use super::path_finder::PathFinder;
use crate::InputError;
use std::env::VarError;
use std::ops::ControlFlow;

#[derive(Debug)]
pub enum BuiltinCommand {
    Exit,
    Echo,
    Type,
}

impl TryFrom<&str> for BuiltinCommand {
    type Error = InputError;
    fn try_from(value: &str) -> Result<BuiltinCommand, Self::Error> {
        match value {
            "exit" => Ok(BuiltinCommand::Exit),
            "echo" => Ok(BuiltinCommand::Echo),
            "type" => Ok(BuiltinCommand::Type),
            _ => Err(InputError::CommandNotFound(value.to_string())),
        }
    }
}

impl std::fmt::Display for BuiltinCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinCommand::Exit => write!(f, "exit"),
            BuiltinCommand::Echo => write!(f, "echo"),
            BuiltinCommand::Type => write!(f, "type"),
        }
    }
}

impl BuiltinCommand {
    pub fn execute(self, args: Vec<String>) -> ControlFlow<()> {
        match self {
            BuiltinCommand::Exit => ControlFlow::Break(()),
            BuiltinCommand::Echo => {
                exec_echo(args);
                ControlFlow::Continue(())
            }
            BuiltinCommand::Type => {
                let _ = exec_type(args);
                ControlFlow::Continue(())
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
        match BuiltinCommand::try_from(arg.as_str()) {
            Ok(comm) => {
                println!("{} is a shell builtin", comm)
            }
            Err(_) => {
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
