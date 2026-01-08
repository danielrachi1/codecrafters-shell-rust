use rustyline::Editor;
use rustyline::history::FileHistory;

use crate::output_config::OutputConfig;
use crate::runner;
use crate::shell_helper::ShellHelper;
use crate::{builtin_command::BuiltinCommand, command::Command};
use std::ops::ControlFlow;

pub struct Order<'a> {
    command: Command,
    args: Vec<String>,
    output_config: OutputConfig,
    editor: &'a mut Editor<ShellHelper, FileHistory>,
}

impl<'a> Order<'a> {
    pub fn new(
        command: Command,
        args: Vec<String>,
        output_config: OutputConfig,
        editor: &'a mut Editor<ShellHelper, FileHistory>,
    ) -> Self {
        Order {
            command,
            args,
            output_config,
            editor,
        }
    }

    pub fn execute(self) -> ControlFlow<()> {
        let Self {
            command,
            args,
            output_config,
            editor,
        } = self;

        let result: Result<ControlFlow<()>, Box<dyn std::error::Error>> = match &command {
            Command::Builtin(BuiltinCommand::Exit) => Ok(runner::exit()),
            Command::Builtin(BuiltinCommand::Echo) => Ok(runner::echo(&args, output_config)),
            Command::Builtin(BuiltinCommand::Type) => runner::r#type(&args, output_config)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>),
            Command::Builtin(BuiltinCommand::Pwd) => Ok(runner::pwd(output_config)),
            Command::Builtin(BuiltinCommand::Cd) => {
                runner::cd(&args).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
            Command::Builtin(BuiltinCommand::History) => {
                runner::history(editor, output_config, &args)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
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
