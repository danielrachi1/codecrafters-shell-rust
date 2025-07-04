#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input_words = parse_input(&input);

        match input_words[0] {
            "exit 0" => break,
            "echo" => println!("{}", input_words[1..].join(" ")),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split(" ").collect::<Vec<&str>>()
}
