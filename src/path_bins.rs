use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::{env, io, path::PathBuf};

pub struct PathBins(pub Vec<PathBuf>);

impl PathBins {
    pub fn new() -> Result<Self, io::Error> {
        let path_string = std::env::var_os("PATH").expect("PATH enviroment variable must be set");
        let mut executables = Vec::new();

        env::split_paths(&path_string).for_each(|dir| {
            if let Ok(entries) = dir.read_dir() {
                entries
                    .filter_map(|entry| entry.ok())
                    .filter(|file| has_exec_permissions(&file.path()).unwrap_or(false))
                    .for_each(|bin| {
                        executables.push(bin.path());
                    });
            }
        });

        Ok(PathBins(executables))
    }
}

fn has_exec_permissions(path: &Path) -> Result<bool, io::Error> {
    Ok(path.metadata()?.mode() & 0o111 != 0)
}
