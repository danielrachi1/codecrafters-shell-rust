use std::io::Write;

use crate::input;
use crate::order::Order;
use crate::output::Output;

pub fn input() -> Option<Order> {
    let input_string = input::read_input();
    if input_string.is_empty() {
        None
    } else {
        let (command, args) = input::parse_input(input_string);
        Some(Order::new(command, args))
    }
}

pub fn output(output: Output, content: String) {
    match output {
        Output::StdOut => {
            print!("{}", content);
            std::io::stdout().flush().unwrap();
        }
        Output::StdErr => eprint!("{}", content),
    }
}
