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
    overwrite: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        OutputConfig {
            stdout: Output::StdOut(io::stdout()),
            stderr: Output::StdErr(io::stderr()),
            overwrite: false,
        }
    }
}

impl OutputConfig {
    pub fn redirect(mut self, file_descriptor: FileDescriptor, path: PathBuf) -> io::Result<Self> {
        let file = Output::File(File::create(path)?);

        match file_descriptor {
            FileDescriptor::StdOut => self.stdout = file,
            FileDescriptor::StdErr => self.stderr = file,
        }

        self.overwrite = true;

        Ok(self)
    }

    pub fn append(mut self, file_descriptor: FileDescriptor, path: PathBuf) -> io::Result<Self> {
        let file = Output::File(OpenOptions::new().append(true).create(true).open(path)?);

        match file_descriptor {
            FileDescriptor::StdOut => self.stdout = file,
            FileDescriptor::StdErr => self.stderr = file,
        }

        self.overwrite = false;

        Ok(self)
    }
}
