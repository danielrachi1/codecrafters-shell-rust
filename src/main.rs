#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let builtins = ["exit", "echo", "type"];

        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input_words = parse_input(&input);

        match input_words[0] {
            "exit" => break,
            "echo" => println!("{}", input_words[1..].join(" ")),
            "type" => {
                if builtins.contains(&input_words[1]) {
                    println!("{} is a shell builtin", input_words[1])
                } else {
                    println!("{}: not found", input_words[1])
                }
            }
            _ => println!("{}: command not found", input.trim()),
        }
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split(" ").collect::<Vec<&str>>()
}
