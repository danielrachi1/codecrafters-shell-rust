mod argument_parser;
mod builtin_command;
mod command;
mod error;
mod helper;
mod input;
mod order;
mod output;
mod output_config;
mod path_finder;
mod runner;
mod shell;

use crate::helper::ShellHelper;
use rustyline::{Config, Editor, Result};
use std::ops::ControlFlow;

fn main() -> Result<()> {
    let helper = ShellHelper::default();
    let config = Config::builder().auto_add_history(true).build();
    let history = rustyline::sqlite_history::SQLiteHistory::with_config(&config)?;
    let mut rl: Editor<ShellHelper, _> = Editor::with_history(config, history)?;
    rl.set_helper(Some(helper));

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
