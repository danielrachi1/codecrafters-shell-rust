use std::{
    fs::File,
    io::{Stderr, Stdout, Write},
};

pub enum Output {
    StdOut(Stdout),
    StdErr(Stderr),
    File(File),
    Pipe(Vec<u8>),
}

impl Output {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Output::StdOut(_) | Output::StdErr(_) | Output::File(_) => Vec::new(),
            Output::Pipe(data) => data,
        }
    }
}


impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Output::StdOut(s) => s.write(buf),
            Output::StdErr(s) => s.write(buf),
            Output::File(f) => f.write(buf),
            Output::Pipe(v) => v.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Output::StdOut(s) => s.flush(),
            Output::StdErr(s) => s.flush(),
            Output::File(f) => f.flush(),
            Output::Pipe(v) => v.flush(),
        }
    }
}
