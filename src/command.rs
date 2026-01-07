use crate::builtin_command::BuiltinCommand;
use crate::error::not_found::NotFound;
use crate::path_bins::PathBins;
use std::path::PathBuf;

pub enum Command {
    Builtin(BuiltinCommand),
    Executable(PathBuf),
}

impl TryFrom<String> for Command {
    type Error = NotFound;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok(builtin_command) = BuiltinCommand::try_from(value.clone()) {
            return Ok(Self::Builtin(builtin_command));
        }
        if let Some(path) = PathBins::new()?
            .0
            .iter()
            .find(|bin| bin.file_name().unwrap().to_string_lossy() == value)
        {
            Ok(Self::Executable(path.into()))
        } else {
            Err(NotFound::Command(value))
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
