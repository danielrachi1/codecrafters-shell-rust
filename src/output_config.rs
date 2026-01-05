use crate::error::not_found::NotFound;
use crate::file_descriptor::FileDescriptor;
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
    pub fn new(args: &mut Vec<String>) -> Result<Self, NotFound> {
        if let Some(idx) = args
            .iter()
            .position(|arg| arg == ">" || arg == "1>" || arg == "2>")
        {
            let redirect_symbol = args.get(idx).unwrap().to_owned();
            let output_file = PathBuf::from(args.get(idx + 1).ok_or(NotFound::RedirectTarget)?);
            args.truncate(idx);
            Ok(OutputConfig::default().redirect(redirect_symbol.try_into()?, output_file)?)
        } else if let Some(idx) = args
            .iter()
            .position(|arg| arg == ">>" || arg == "1>>" || arg == "2>>")
        {
            let redirect_symbol = args.get(idx).unwrap().to_owned();
            let output_file = PathBuf::from(args.get(idx + 1).ok_or(NotFound::RedirectTarget)?);
            args.truncate(idx);
            Ok(OutputConfig::default().append(redirect_symbol.try_into()?, output_file)?)
        } else {
            Ok(OutputConfig::default())
        }
    }

    pub fn redirect(mut self, file_descriptor: FileDescriptor, path: PathBuf) -> io::Result<Self> {
        let file = Output::File(File::create(path)?);

        match file_descriptor {
            FileDescriptor::StdOut => self.stdout = file,
            FileDescriptor::StdErr => self.stderr = file,
        }

        Ok(self)
    }

    pub fn append(mut self, file_descriptor: FileDescriptor, path: PathBuf) -> io::Result<Self> {
        let file = Output::File(OpenOptions::new().append(true).create(true).open(path)?);

        match file_descriptor {
            FileDescriptor::StdOut => self.stdout = file,
            FileDescriptor::StdErr => self.stderr = file,
        }

        Ok(self)
    }
}
