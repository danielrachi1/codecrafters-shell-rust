use crate::builtin_command::BuiltinCommand;
use crate::error::not_found::NotFound;
use crate::path_finder::PathFinder;
use std::path::PathBuf;

pub enum Command {
    Builtin(BuiltinCommand),
    Executable(PathBuf),
}

impl TryFrom<String> for Command {
    type Error = NotFound;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok(builtin_command) = BuiltinCommand::try_from(value.clone()) {
            Ok(Self::Builtin(builtin_command))
        } else {
            let path = PathFinder::new(value.clone())
                .find_executable()
                .ok_or(NotFound::Command(value))?;
            Ok(Self::Executable(path))
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Builtin(builtin_command) => write!(f, "{}", builtin_command),
            Command::Executable(path) => write!(f, "{}", path.display()),
        }
    }
}
