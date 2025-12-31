use crate::{builtin_command::BuiltinCommand, command::Command, runner};
use std::ops::ControlFlow;

pub struct Order {
    command: Command,
    args: Vec<String>,
}

impl Order {
    pub fn new(command: Command, args: Vec<String>) -> Self {
        Order { command, args }
    }

    pub fn execute(self) -> ControlFlow<()> {
        match self.command {
            Command::Builtin(BuiltinCommand::Exit) => runner::exit(),
            Command::Builtin(BuiltinCommand::Echo) => runner::echo(self.args),
            Command::Builtin(BuiltinCommand::Type) => runner::r#type(self.args),
            Command::Builtin(BuiltinCommand::Pwd) => runner::pwd(),
            Command::Builtin(BuiltinCommand::Cd) => runner::cd(self.args),
            Command::Executable(path) => runner::executable(path, self.args),
        }
    }
}
