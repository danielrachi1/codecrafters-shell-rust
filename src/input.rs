use crate::argument_parser::ArgumentParser;
use crate::command::Command;
use crate::error::not_found::NotFound;
use crate::output_config::OutputConfig;

pub fn parse_input(input: String) -> Result<(Command, Vec<String>, OutputConfig), NotFound> {
    let parsed_input = ArgumentParser::new(input).parse();
    let (command_array, args_array) = parsed_input.split_at(1);
    let command_string = command_array[0].clone();

    let command = Command::try_from(command_string)?;
    let mut args = args_array;

    let output_config = if let Some((index, symbol)) = find_special_symbols(args) {
        let file_path = args
            .get(index + 1)
            .ok_or(NotFound::RedirectTargetFile)?
            .into();

        args = &args[0..index];

        OutputConfig::new(symbol, file_path)
    } else {
        Ok(OutputConfig::default())
    }?;

    Ok((command, args.to_vec(), output_config))
}

fn find_special_symbols(args: &[String]) -> Option<(usize, &String)> {
    let special_symbols = [">", "1>", "2>", ">>", "1>>", "2>>"];

    args.iter()
        .enumerate()
        .find(|(_index, arg)| special_symbols.contains(&arg.as_str()))
}
