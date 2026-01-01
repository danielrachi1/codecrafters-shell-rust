mod argument_parser;
mod builtin_command;
mod command;
mod error;
mod input;
mod order;
mod path_finder;
mod runner;
mod shell;

use std::{io::Write, ops::ControlFlow};

fn main() {
    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();

        let Some(order) = shell::input() else {
            continue;
        };

        match order.execute() {
            ControlFlow::Continue(_) => continue,
            ControlFlow::Break(_) => break,
        }
    }
}
