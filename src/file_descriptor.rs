use crate::error::not_found::NotFound;

pub enum FileDescriptor {
    StdOut,
    StdErr,
}

impl TryFrom<String> for FileDescriptor {
    type Error = NotFound;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "1>" | ">" | "1>>" | ">>" => Ok(FileDescriptor::StdOut),
            "2>" | "2>>" => Ok(FileDescriptor::StdErr),
            _ => Err(NotFound::FileDescriptor),
        }
    }
}
