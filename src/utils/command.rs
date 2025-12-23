use crate::InputError;

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
