mod argument_parser;
mod builtin_command;
mod command;
mod error;
mod input;
mod order;
mod output;
mod output_config;
mod path_finder;
mod runner;
mod shell;

use rustyline::{DefaultEditor, Result};
use std::ops::ControlFlow;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let line = rl.readline("$ ")?;

        let Some(order) = shell::input(line) else {
            continue;
        };

        match order.execute() {
            ControlFlow::Continue(_) => continue,
            ControlFlow::Break(_) => break Ok(()),
        }
    }
}
