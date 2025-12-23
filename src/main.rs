#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.split_whitespace().next().unwrap();
        if command == "exit" {
            break;
        } else {
            println!(
                "{}: command not found",
                input.split_whitespace().next().unwrap()
            );
        }
    }
}
