#[derive(Debug)]
pub enum NotFound {
    Command(String),
    RedirectTarget,
    Io(String),
}

impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotFound::Command(command) => write!(f, "{}: command not found", command),
            NotFound::RedirectTarget => {
                write!(f, "syntax error near unexpected token `newline'")
            }
            NotFound::Io(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for NotFound {}

impl From<std::io::Error> for NotFound {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => NotFound::Io("No such file or directory".to_string()),
            _ => panic!("Unhandled IO error kind"),
        }
    }
}
