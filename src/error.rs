use std::{env::VarError, io};

#[derive(thiserror::Error, Debug)]
pub enum InputError {
    #[error("provided command is an empty string")]
    EmptyCommand,
    #[error("too many arguments")]
    TooManyArguments,
}

#[derive(thiserror::Error, Debug)]
pub enum ExecutionError {
    #[error("{0}: command not found")]
    CommandNotFound(String),
    #[error("error reading PATH environment variable: {0}")]
    CantReadEnvPath(VarError),
    #[error("error getting the current directory path: {0}")]
    CurrentDirError(io::Error),
    #[error("failed to get home dir")]
    HomeDirFail,
    #[error("cd: {0}: {1}")]
    ChdirFail(String, String),
    #[error("failed to execute OS process: {0}")]
    OsProcessError(io::Error),
}
