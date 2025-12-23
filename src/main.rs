#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input = input.split_whitespace();
        let command = input.next().unwrap();
        match command {
            "exit" => break,
            "echo" => {
                for arg in input {
                    print!("{} ", arg);
                }
                println!();
            }
            _ => {
                println!("{}: command not found", command)
            }
        }
    }
}
