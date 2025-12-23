#[allow(unused_imports)]
use std::io::{self, Write};

mod error;
mod utils;

use error::InputError;
use utils::command::Command;
use utils::input::Input;

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
