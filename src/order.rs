use crate::error::not_found::NotFound;
use crate::output_config::OutputConfig;
use crate::runner;
use crate::{builtin_command::BuiltinCommand, command::Command};
use std::ops::ControlFlow;
use std::path::PathBuf;

pub struct Order {
    command: Command,
    args: Vec<String>,
    output_config: OutputConfig,
}

impl Order {
    pub fn new(command: Command, mut args: Vec<String>) -> Result<Self, NotFound> {
        let output_config = if let Some(idx) = args
            .iter()
            .position(|arg| arg == ">" || arg == "1>" || arg == "2>")
        {
            let redirect_symbol = args.get(idx).unwrap().to_owned();
            let output_file = PathBuf::from(args.get(idx + 1).ok_or(NotFound::RedirectTarget)?);
            args.truncate(idx);
            OutputConfig::default().redirect(redirect_symbol.try_into()?, output_file)?
        } else if let Some(idx) = args
            .iter()
            .position(|arg| arg == ">>" || arg == "1>>" || arg == "2>>")
        {
            let redirect_symbol = args.get(idx).unwrap().to_owned();
            let output_file = PathBuf::from(args.get(idx + 1).ok_or(NotFound::RedirectTarget)?);
            args.truncate(idx);
            OutputConfig::default().append(redirect_symbol.try_into()?, output_file)?
        } else {
            OutputConfig::default()
        };

        Ok(Order {
            command,
            args,
            output_config,
        })
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
            Command::Builtin(BuiltinCommand::Type) => Ok(runner::r#type(&args, output_config)),
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
