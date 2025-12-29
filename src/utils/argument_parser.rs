enum State {
    Normal,
    SingleQuote,
    DoubleQuote,
    Escaped,
}

pub struct ArgumentParser {
    input: String,
    mode: State,
    current_word: String,
    result: Vec<String>,
}

impl ArgumentParser {
    pub fn new(input: String) -> ArgumentParser {
        ArgumentParser {
            input,
            mode: State::Normal,
            current_word: String::new(),
            result: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<String> {
        for c in self.input.chars() {
            match self.mode {
                State::Normal => match c {
                    ' ' => {
                        if !self.current_word.is_empty() {
                            self.result.push(self.current_word.clone());
                            self.current_word = String::new();
                        } else {
                            continue;
                        }
                    }
                    '\'' => self.mode = State::SingleQuote,
                    '\"' => self.mode = State::DoubleQuote,
                    '\\' => self.mode = State::Escaped,
                    _ => self.current_word.push(c),
                },
                State::SingleQuote => {
                    if c == '\'' {
                        self.mode = State::Normal
                    } else {
                        self.current_word.push(c);
                    }
                }
                State::DoubleQuote => {
                    if c == '"' {
                        self.mode = State::Normal;
                    } else {
                        self.current_word.push(c);
                    }
                }
                State::Escaped => {
                    self.current_word.push(c);
                    self.mode = State::Normal;
                }
            }
        }
        if !self.current_word.is_empty() {
            self.result.push(self.current_word.clone());
        }
        self.result.clone()
    }
}
