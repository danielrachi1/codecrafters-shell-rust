#[allow(unused_imports)]
use std::io::{self, Write};

#[derive(thiserror::Error, Debug)]
enum InputError {
    #[error("{0}: command not found")]
    CommandNotFound(String),
}

#[derive(Debug)]
enum Command {
    Exit,
    Echo,
    Type,
}

impl TryFrom<&str> for Command {
    type Error = InputError;
    fn try_from(value: &str) -> Result<Command, Self::Error> {
        match value {
            "exit" => Ok(Command::Exit),
            "echo" => Ok(Command::Echo),
            "type" => Ok(Command::Type),
            _ => Err(InputError::CommandNotFound(value.to_string())),
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Exit => write!(f, "exit"),
            Command::Echo => write!(f, "echo"),
            Command::Type => write!(f, "type"),
        }
    }
}

struct Input {
    command: Command,
    args: Vec<String>,
}

impl Input {
    fn new(buf: &mut str) -> Result<Input, InputError> {
        let mut buf_vec = buf.split_whitespace();
        Ok(Input {
            command: buf_vec.nth(0).unwrap().try_into()?,
            args: buf_vec.map(|arg| arg.to_string()).collect(),
        })
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let input_res = Input::new(&mut buf);

        match input_res {
            Ok(input) => match input.command {
                Command::Exit => break,
                Command::Echo => {
                    for arg in input.args {
                        print!("{} ", arg);
                    }
                    println!();
                }
                Command::Type => {
                    for arg in input.args {
                        let comm_res: Result<Command, InputError> = arg.as_str().try_into();
                        match comm_res {
                            Ok(comm) => println!("{} is a shell builtin", comm),
                            Err(_) => println!("{}: not found", arg),
                        }
                    }
                }
            },
            Err(err) => println!("{}", err),
        }
    }
}
