use crate::ShellHelper;
use crate::input;
use crate::order::Order;
use rustyline::Editor;
use rustyline::history::FileHistory;

pub fn input(rl: &mut Editor<ShellHelper, FileHistory>, line: String) -> Option<Order<'_>> {
    if line.is_empty() {
        return None;
    }

    let parsed_input = input::parse_input(line);
    match parsed_input {
        Ok((command, args, output_conf)) => Some(Order::new(command, args, output_conf, rl)),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}
