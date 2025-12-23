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

        let input = match Input::new(&mut buf) {
            Ok(input) => input,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        if input.command.execute(input.args).is_break() {
            break;
        }
    }
}
