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
    stdin: Option<Vec<u8>>,
}

impl<'a> Order<'a> {
    pub fn new(
        command: Command,
        args: Vec<String>,
        output_config: OutputConfig,
        editor: &'a mut Editor<ShellHelper, FileHistory>,
        stdin: Option<Vec<u8>>,
    ) -> Self {
        Order {
            command,
            args,
            output_config,
            editor,
            stdin,
        }
    }

    pub fn execute(self) -> (ControlFlow<()>, Option<Vec<u8>>) {
        let Self {
            command,
            args,
            output_config,
            editor,
            stdin,
        } = self;

        let result: Result<(ControlFlow<()>, OutputConfig), Box<dyn std::error::Error>> = match &command {
            Command::Builtin(BuiltinCommand::Exit) => Ok((runner::exit(editor), output_config)),
            Command::Builtin(BuiltinCommand::Echo) => Ok(runner::echo(&args, output_config)),
            Command::Builtin(BuiltinCommand::Type) => runner::r#type(&args, output_config)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>),
            Command::Builtin(BuiltinCommand::Pwd) => Ok(runner::pwd(output_config)),
            Command::Builtin(BuiltinCommand::Cd) => {
                runner::cd(&args)
                    .map(|cf| (cf, output_config))
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
            Command::Builtin(BuiltinCommand::History) => {
                runner::history(editor, output_config, &args)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
            Command::Executable(path) => Ok(runner::executable(path, &args, output_config, stdin)),
        };

        match result {
            Ok((control_flow, out_config)) => {
                // Extract captured output if we were piping
                let captured_output = out_config.stdout.into_bytes();
                let captured = if captured_output.is_empty() {
                    None
                } else {
                    Some(captured_output)
                };
                (control_flow, captured)
            }
            Err(err) => {
                eprintln!("{}: {}: {}", command, args.join(" "), err);
                (ControlFlow::Continue(()), None)
            }
        }
    }
}
