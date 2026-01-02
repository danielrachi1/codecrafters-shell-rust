#[derive(Debug)]
pub enum InputError {
    CommandNotFound(String),
    RedirectTargetNotFound,
    Io(std::io::Error),
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::CommandNotFound(command) => write!(f, "{}: command not found", command),
            InputError::RedirectTargetNotFound => {
                write!(f, "syntax error near unexpected token `newline'")
            }
            InputError::Io(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for InputError {}

impl From<std::io::Error> for InputError {
    fn from(value: std::io::Error) -> Self {
        InputError::Io(value)
    }
}
