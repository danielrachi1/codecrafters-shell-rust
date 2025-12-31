#[derive(Debug)]
pub struct CommandNotFound(pub String);

impl std::fmt::Display for CommandNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: command not found", self.0)
    }
}

impl std::error::Error for CommandNotFound {}
