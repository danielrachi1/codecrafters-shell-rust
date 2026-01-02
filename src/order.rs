use crate::error::input_error::InputError;
use crate::output::Output;
use crate::runner;
use crate::{builtin_command::BuiltinCommand, command::Command};
use std::ops::ControlFlow;
use std::path::PathBuf;

pub struct Order {
    command: Command,
    args: Vec<String>,
    output: Output,
}

impl Order {
    pub fn new(command: Command, mut args: Vec<String>) -> Result<Self, InputError> {
        let mut output_file = None;

        if let Some(idx) = args.iter().position(|arg| arg == ">" || arg == "1>") {
            output_file = Some(PathBuf::from(
                args.get(idx + 1)
                    .ok_or(InputError::RedirectTargetNotFound)?,
            ));
            args.truncate(idx);
        }

        let output = Output::new(&output_file)?;

        Ok(Order {
            command,
            args,
            output,
        })
    }

    pub fn execute(self) -> ControlFlow<()> {
        let Self {
            command,
            args,
            output,
        } = self;

        let result = match &command {
            Command::Builtin(BuiltinCommand::Exit) => Ok(runner::exit()),
            Command::Builtin(BuiltinCommand::Echo) => Ok(runner::echo(&args, output)),
            Command::Builtin(BuiltinCommand::Type) => Ok(runner::r#type(&args, output)),
            Command::Builtin(BuiltinCommand::Pwd) => Ok(runner::pwd(output)),
            Command::Builtin(BuiltinCommand::Cd) => runner::cd(&args),
            Command::Executable(path) => Ok(runner::executable(path, &args, output)),
        };

        if let Err(err) = result {
            eprintln!("{}: {}: {}", command, args.join(" "), err)
        }

        ControlFlow::Continue(())
    }
}
