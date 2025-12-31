use crate::argument_parser::ArgumentParser;
use crate::command::Command;
use crate::error::command_not_found::CommandNotFound;
use std::io;

pub fn read_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

pub fn parse_input(input: String) -> Result<(Command, Vec<String>), CommandNotFound> {
    let parsed_input = ArgumentParser::new(input).parse();
    let (command_array, args_array) = parsed_input.split_at(1);
    let command_string = command_array[0].clone();
    let args = args_array.to_vec();
    let command = Command::try_from(command_string)?;

    Ok((command, args))
}
