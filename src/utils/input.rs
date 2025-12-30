use crate::BuiltinCommand;
use crate::InputError;
use crate::utils::argument_parser::*;

pub struct Input {
    pub command: BuiltinCommand,
    pub args: Vec<String>,
}

impl Input {
    pub fn new(buf: String) -> Result<Input, InputError> {
        let parsed_input = ArgumentParser::new(buf.trim().to_string()).parse();
        let (command, args) = parsed_input.split_at(1);
        Input {
            command: command[0].as_str().into(),
            args: args.to_vec(),
        }
        .check_args()
    }

    fn check_args(self) -> Result<Self, InputError> {
        match self.command {
            BuiltinCommand::Cd => {
                if self.args.len() > 1 {
                    Err(InputError::TooManyArguments)
                } else {
                    Ok(self)
                }
            }
            _ => Ok(self),
        }
    }
}
