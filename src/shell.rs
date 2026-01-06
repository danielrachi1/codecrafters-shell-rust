use crate::input;
use crate::order::Order;

pub fn input(line: String) -> Option<Order> {
    if line.is_empty() {
        return None;
    }

    let parsed_input = input::parse_input(line);
    match parsed_input {
        Ok((command, args, output_conf)) => Some(Order::new(command, args, output_conf)),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}
