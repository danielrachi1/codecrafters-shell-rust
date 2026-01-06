use crate::output_config::OutputConfig;
use crate::runner;
use crate::{builtin_command::BuiltinCommand, command::Command};
use std::ops::ControlFlow;

pub struct Order {
    command: Command,
    args: Vec<String>,
    output_config: OutputConfig,
}

impl Order {
    pub fn new(command: Command, args: Vec<String>, output_config: OutputConfig) -> Self {
        Order {
            command,
            args,
            output_config,
        }
    }

    pub fn execute(self) -> ControlFlow<()> {
        let Self {
            command,
            args,
            output_config,
        } = self;

        let result = match &command {
            Command::Builtin(BuiltinCommand::Exit) => Ok(runner::exit()),
            Command::Builtin(BuiltinCommand::Echo) => Ok(runner::echo(&args, output_config)),
            Command::Builtin(BuiltinCommand::Type) => runner::r#type(&args, output_config),
            Command::Builtin(BuiltinCommand::Pwd) => Ok(runner::pwd(output_config)),
            Command::Builtin(BuiltinCommand::Cd) => runner::cd(&args),
            Command::Executable(path) => Ok(runner::executable(path, &args, output_config)),
        };

        match result {
            Ok(control_flow) => control_flow,
            Err(err) => {
                eprintln!("{}: {}: {}", command, args.join(" "), err);
                ControlFlow::Continue(())
            }
        }
    }
}
