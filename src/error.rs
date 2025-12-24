use std::env::VarError;
use std::io;

#[derive(thiserror::Error, Debug)]
pub enum InputError {
    #[error("{0}: command not found")]
    CommandNotFound(String),
    #[error("provided command is an empty string")]
    EmptyCommand,
    #[error("error reading PATH environment variable: {0}")]
    CantReadEnvPath(VarError),
    #[error("error executing command: {0}")]
    CommandExecutionError(io::Error),
}

impl From<VarError> for InputError {
    fn from(value: VarError) -> Self {
        InputError::CantReadEnvPath(value)
    }
}

impl From<io::Error> for InputError {
    fn from(value: io::Error) -> Self {
        InputError::CommandExecutionError(value)
    }
}
