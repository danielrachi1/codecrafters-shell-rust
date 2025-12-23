#[allow(unused_imports)]
use std::io::{self, Write};

enum Command {
    Exit,
    Echo,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value {
            "exit" => Command::Exit,
            "echo" => Command::Echo,
            _ => panic!("Unknown command"),
        }
    }
}

struct Input<'a> {
    command: Command,
    args: Vec<&'a str>,
}

impl Input<'static> {
    fn new(buf: &mut str) -> Input<'_> {
        let mut buf_vec = buf.split_whitespace();
        Input {
            command: buf_vec.nth(0).unwrap().into(),
            args: buf_vec.collect(),
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let input = Input::new(&mut buf);
        match input.command {
            Command::Exit => break,
            Command::Echo => {
                for arg in input.args {
                    print!("{} ", arg);
                }
                println!();
            }
        }
    }
}
