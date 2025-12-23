use crate::InputError;
use std::ops::ControlFlow;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

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
                for arg in args {
                    print!("{} ", arg);
                }
                println!();
                ControlFlow::Continue(())
            }
            BuiltinCommand::Type => {
                for arg in args {
                    match BuiltinCommand::try_from(arg.as_str()) {
                        Ok(comm) => println!("{} is a shell builtin", comm),
                        Err(_) => {
                            let mut found = false;
                            for dir in std::env::var("PATH").unwrap_or_default().split(":") {
                                let path: PathBuf = [dir, arg.as_str()].iter().collect();
                                if let Ok(metadata) = path.metadata()
                                    && metadata.mode() & 0o111 != 0
                                {
                                    println!("{} is {}", arg, path.display());
                                    found = true;
                                    break;
                                }
                            }
                            if !found {
                                println!("{}: not found", arg);
                            }
                        }
                    }
                }
                ControlFlow::Continue(())
            }
        }
    }
}
