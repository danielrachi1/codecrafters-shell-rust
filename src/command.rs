use crate::builtin_command::BuiltinCommand;
use crate::path_finder::PathFinder;
use std::path::PathBuf;

pub enum Command {
    Builtin(BuiltinCommand),
    Executable(PathBuf),
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        if let Ok(builtin_command) = BuiltinCommand::try_from(value.clone()) {
            Self::Builtin(builtin_command)
        } else {
            let path = PathFinder::new(value).unwrap().find_executable().unwrap();
            Self::Executable(path)
        }
    }
}
