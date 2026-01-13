use crate::ShellHelper;
use crate::command::Command;
use crate::input;
use crate::output_config::OutputConfig;
use rustyline::Editor;
use rustyline::history::FileHistory;

pub type Pipeline = Vec<(Command, Vec<String>, OutputConfig)>;

pub fn input(_rl: &mut Editor<ShellHelper, FileHistory>, line: String) -> Option<Pipeline> {
    if line.is_empty() {
        return None;
    }

    let parsed_input = input::parse_input(line);
    match parsed_input {
        Ok(inputs) => Some(inputs),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}
