use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::{
    env::{self, VarError},
    io,
    path::PathBuf,
};

pub struct PathFinder {
    candidates: Vec<PathBuf>,
}

impl PathFinder {
    pub fn new(executable: String) -> Result<PathFinder, VarError> {
        let path_string = std::env::var("PATH")?;
        let candidates = env::split_paths(&path_string)
            .map(|dir| {
                let mut path = dir.clone();
                path.push(&executable);
                path
            })
            .collect();

        Ok(PathFinder { candidates })
    }

    pub fn find_executable(self) -> Option<PathBuf> {
        self.candidates
            .into_iter()
            .find(|candidate| has_exec_permissions(candidate).unwrap_or(false))
    }
}

fn has_exec_permissions(path: &Path) -> Result<bool, io::Error> {
    Ok(path.metadata()?.mode() & 0o111 != 0)
}
