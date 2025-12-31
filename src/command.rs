use crate::builtin_command::BuiltinCommand;
use crate::error::command_not_found::CommandNotFound;
use crate::path_finder::PathFinder;
use std::path::PathBuf;

pub enum Command {
    Builtin(BuiltinCommand),
    Executable(PathBuf),
}

impl TryFrom<String> for Command {
    type Error = CommandNotFound;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok(builtin_command) = BuiltinCommand::try_from(value.clone()) {
            Ok(Self::Builtin(builtin_command))
        } else {
            let path = PathFinder::new(value.clone())
                .find_executable()
                .ok_or(CommandNotFound(value))?;
            Ok(Self::Executable(path))
        }
    }
}
