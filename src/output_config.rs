use crate::error::not_found::NotFound;
use crate::output::Output;
use std::{
    fs::{File, OpenOptions},
    io,
    path::PathBuf,
};

pub struct OutputConfig {
    pub stdout: Output,
    pub stderr: Output,
}

impl Default for OutputConfig {
    fn default() -> Self {
        OutputConfig {
            stdout: Output::StdOut(io::stdout()),
            stderr: Output::StdErr(io::stderr()),
        }
    }
}

impl OutputConfig {
    pub fn new(symbol: &str, file_path: PathBuf) -> Result<Self, NotFound> {
        match symbol {
            ">" | "1>" => Ok(OutputConfig {
                stdout: Output::File(File::create(file_path)?),
                stderr: Output::StdErr(io::stderr()),
            }),

            "2>" => Ok(OutputConfig {
                stdout: Output::StdOut(io::stdout()),
                stderr: Output::File(File::create(file_path)?),
            }),
            ">>" | "1>>" => Ok(OutputConfig {
                stdout: Output::File(
                    OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(file_path)?,
                ),
                stderr: Output::StdErr(io::stderr()),
            }),
            "2>>" => Ok(OutputConfig {
                stdout: Output::StdOut(io::stdout()),
                stderr: Output::File(
                    OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(file_path)?,
                ),
            }),
            _ => Err(NotFound::OutputConfigSymbol),
        }
    }
}
