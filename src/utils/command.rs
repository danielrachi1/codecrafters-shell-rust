use crate::InputError;
use std::ops::ControlFlow;

#[derive(Debug)]
pub enum Command {
    Exit,
    Echo,
    Type,
}

impl TryFrom<&str> for Command {
    type Error = InputError;
    fn try_from(value: &str) -> Result<Command, Self::Error> {
        match value {
            "exit" => Ok(Command::Exit),
            "echo" => Ok(Command::Echo),
            "type" => Ok(Command::Type),
            _ => Err(InputError::CommandNotFound(value.to_string())),
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Exit => write!(f, "exit"),
            Command::Echo => write!(f, "echo"),
            Command::Type => write!(f, "type"),
        }
    }
}

impl Command {
    pub fn execute(self, args: Vec<String>) -> ControlFlow<()> {
        match self {
            Command::Exit => ControlFlow::Break(()),
            Command::Echo => {
                for arg in args {
                    print!("{} ", arg);
                }
                println!();
                ControlFlow::Continue(())
            }
            Command::Type => {
                for arg in args {
                    let comm_res: Result<Command, InputError> = arg.as_str().try_into();
                    match comm_res {
                        Ok(comm) => println!("{} is a shell builtin", comm),
                        Err(_) => println!("{}: not found", arg),
                    }
                }
                ControlFlow::Continue(())
            }
        }
    }
}
