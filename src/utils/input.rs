use crate::BuiltinCommand;
use crate::InputError;

pub struct Input {
    pub command: BuiltinCommand,
    pub args: Vec<String>,
}

impl Input {
    pub fn new(buf: String) -> Result<Input, InputError> {
        let buf_tup = buf.split_once(" ").ok_or(InputError::EmptyCommand)?;
        let command = buf_tup.0.into();
        let args = split_args(buf_tup.1);
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

fn split_args(args: &str) -> Vec<String> {
    let mut quote_flag = false;
    args.split(|c: char| {
        if c == '\'' {
            quote_flag = !quote_flag;
            false
        } else {
            c.is_whitespace() & !quote_flag
        }
    })
    .filter(|arg| !arg.is_empty())
    .map(|arg| arg.replace("'", ""))
    .collect()
}
