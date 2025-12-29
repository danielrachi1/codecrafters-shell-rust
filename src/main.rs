#[allow(unused_imports)]
use std::io::{self, Write};

mod error;
mod utils;

use error::InputError;
use utils::builtin_command::BuiltinCommand;
use utils::input::Input;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut mut_buf = String::new();
        io::stdin().read_line(&mut mut_buf).unwrap();

        let buf = mut_buf;

        if buf.trim().is_empty() {
            continue;
        }

        let input = match Input::new(buf) {
            Ok(input) => input,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        match input.command.execute(input.args) {
            Ok(exec) => {
                if exec.is_break() {
                    break;
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}
