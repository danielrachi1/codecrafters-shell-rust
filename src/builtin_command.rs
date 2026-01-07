#[derive(Debug)]
pub enum BuiltinCommand {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
    History,
}

impl TryFrom<String> for BuiltinCommand {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "exit" => Ok(BuiltinCommand::Exit),
            "echo" => Ok(BuiltinCommand::Echo),
            "type" => Ok(BuiltinCommand::Type),
            "pwd" => Ok(BuiltinCommand::Pwd),
            "cd" => Ok(BuiltinCommand::Cd),
            "history" => Ok(BuiltinCommand::History),
            _ => Err("Not a builtin command".to_string()),
        }
    }
}

impl std::fmt::Display for BuiltinCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinCommand::Exit => write!(f, "exit"),
            BuiltinCommand::Echo => write!(f, "echo"),
            BuiltinCommand::Type => write!(f, "type"),
            BuiltinCommand::Pwd => write!(f, "pwd"),
            BuiltinCommand::Cd => write!(f, "cd"),
            BuiltinCommand::History => write!(f, "history"),
        }
    }
}
