use crate::input;
use crate::order::Order;

pub fn input() -> Option<Order> {
    let input_string = input::read_input();

    if input_string.is_empty() {
        return None;
    }

    let parsed_input = input::parse_input(input_string);
    match parsed_input {
        Ok((command, args)) => match Order::new(command, args) {
            Ok(order) => Some(order),
            Err(err) => {
                println!("{}", err);
                None
            }
        },
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}
