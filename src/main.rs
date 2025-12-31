mod argument_parser;
mod builtin_command;
mod command;
mod input;
mod order;
mod output;
mod path_finder;
mod runner;
mod shell;

use std::ops::ControlFlow;

use output::Output;

fn main() {
    loop {
        shell::output(Output::StdOut, "$ ".to_string());

        let Some(order) = shell::input() else {
            continue;
        };

        match order.execute() {
            ControlFlow::Continue(_) => continue,
            ControlFlow::Break(_) => break,
        }
    }
}
