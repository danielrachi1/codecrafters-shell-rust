use crate::BuiltinCommand;
use crate::InputError;

pub struct Input {
    pub command: BuiltinCommand,
    pub args: Vec<String>,
}

impl Input {
    pub fn new(buf: &mut str) -> Result<Input, InputError> {
        let mut buf_vec = buf.split_whitespace();
        let command = buf_vec.next().ok_or(InputError::EmptyCommand)?.into();
        Ok(Input {
            command,
            args: buf_vec.map(|arg| arg.to_string()).collect(),
        })
    }
}
