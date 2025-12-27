use crate::error::ExecutionError;

use super::path_finder::PathFinder;
use std::env;
use std::io::ErrorKind;
use std::ops::ControlFlow;

#[derive(Debug)]
pub enum BuiltinCommand {
    Exit,
    Echo,
    Type,
    Run(String),
    Pwd,
    Cd,
}

impl From<&str> for BuiltinCommand {
    fn from(value: &str) -> BuiltinCommand {
        match value {
            "exit" => BuiltinCommand::Exit,
            "echo" => BuiltinCommand::Echo,
            "type" => BuiltinCommand::Type,
            "pwd" => BuiltinCommand::Pwd,
            "cd" => BuiltinCommand::Cd,
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
            BuiltinCommand::Pwd => write!(f, "pwd"),
            BuiltinCommand::Cd => write!(f, "cd"),
        }
    }
}

impl BuiltinCommand {
    pub fn execute(self, args: Vec<String>) -> Result<ControlFlow<()>, ExecutionError> {
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
            BuiltinCommand::Pwd => {
                exec_pwd()?;
                Ok(ControlFlow::Continue(()))
            }
            BuiltinCommand::Cd => {
                exec_cd(args)?;
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

fn exec_type(args: Vec<String>) -> Result<(), ExecutionError> {
    for arg in args {
        match BuiltinCommand::from(arg.as_str()) {
            BuiltinCommand::Exit
            | BuiltinCommand::Echo
            | BuiltinCommand::Type
            | BuiltinCommand::Pwd
            | BuiltinCommand::Cd => {
                println!("{} is a shell builtin", arg)
            }
            _ => {
                let finder =
                    PathFinder::new(arg.clone()).map_err(ExecutionError::CantReadEnvPath)?;
                match finder.find_executable() {
                    Some(path) => println!("{} is {}", arg, path.display()),
                    None => println!("{}: not found", arg),
                }
            }
        }
    }
    Ok(())
}

fn exec_run(program: String, args: Vec<String>) -> Result<(), ExecutionError> {
    // if none of this returns error, the program exists and has exec perms
    PathFinder::new(program.clone())
        .map_err(ExecutionError::CantReadEnvPath)?
        .find_executable()
        .ok_or(ExecutionError::CommandNotFound(program.clone()))?;

    std::process::Command::new(program)
        .args(args)
        .status()
        .map_err(ExecutionError::OsProcessError)?;

    Ok(())
}

fn exec_pwd() -> Result<(), ExecutionError> {
    let path = std::env::current_dir().map_err(ExecutionError::CurrentDirError)?;
    println!("{}", path.display());
    Ok(())
}

fn exec_cd(args: Vec<String>) -> Result<(), ExecutionError> {
    let path = if let Some(p) = args.first() {
        p.clone()
    } else {
        env::home_dir()
            .ok_or(ExecutionError::HomeDirFail)?
            .to_string_lossy()
            .into()
    };
    env::set_current_dir(&path).map_err(|err| {
        let error_message = match err.kind() {
            ErrorKind::NotFound => "No such file or directory".to_string(),
            _ => err.kind().to_string(),
        };
        ExecutionError::ChdirFail(path.clone(), error_message)
    })?;
    Ok(())
}
