mod argument_parser;
mod builtin_command;
mod command;
mod error;
mod input;
mod order;
mod output;
mod output_config;
mod path_bins;
mod runner;
mod shell;
mod shell_helper;

use crate::shell_helper::ShellHelper;
use rustyline::{Config, Editor, Result, sqlite_history::SQLiteHistory};
use std::ops::ControlFlow;

fn main() -> Result<()> {
    let helper = ShellHelper::default();
    let config = Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .auto_add_history(true)
        .build();
    let history = rustyline::sqlite_history::SQLiteHistory::with_config(&config)?;
    let mut rl: Editor<ShellHelper, SQLiteHistory> = Editor::with_history(config, history)?;
    rl.set_helper(Some(helper));

    loop {
        let line = rl.readline("$ ")?;

        let Some(order) = shell::input(&rl, line) else {
            continue;
        };

        match order.execute() {
            ControlFlow::Continue(_) => continue,
            ControlFlow::Break(_) => break Ok(()),
        }
    }
}
