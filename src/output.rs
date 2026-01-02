use std::{
    fs::File,
    io::{self, Stdout, Write},
    path::PathBuf,
};

pub enum Output {
    StdOut(Stdout),
    File(File),
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Output::StdOut(s) => s.write(buf),
            Output::File(f) => f.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Output::StdOut(s) => s.flush(),
            Output::File(f) => f.flush(),
        }
    }
}

impl Output {
    pub fn new(path_opt: &Option<PathBuf>) -> io::Result<Self> {
        match path_opt {
            Some(path) => Ok(Output::File(File::create(path)?)),
            None => Ok(Output::StdOut(io::stdout())),
        }
    }
}
