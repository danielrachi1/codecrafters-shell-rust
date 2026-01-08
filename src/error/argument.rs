#[derive(Debug)]
pub enum ArgumentError {
    WrongType,
}

impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentError::WrongType => write!(f, "numeric argument required"),
        }
    }
}

impl std::error::Error for ArgumentError {}
