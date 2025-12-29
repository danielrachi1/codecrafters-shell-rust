use crate::utils::argument_parser::*;
use crate::BuiltinCommand;
use crate::InputError;

pub struct Input {
    pub command: BuiltinCommand,
    pub args: Vec<String>,
}

impl Input {
    pub fn new(buf: String) -> Result<Input, InputError> {
        let buf_tup = buf.split_once(" ").unwrap_or((buf.trim(), ""));
        let command = buf_tup.0.into();
        let args = ArgumentParser::new(buf_tup.1.trim().to_string()).parse();
        Input { command, args }.check_args()
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
