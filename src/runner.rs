use rustyline::history::{FileHistory, History};
use rustyline::Editor;

use crate::builtin_command::BuiltinCommand;
use crate::error::argument::ArgumentError;
use crate::error::not_found::NotFound;
use crate::output_config::OutputConfig;
use crate::path_bins::PathBins;
use crate::shell_helper::ShellHelper;
use rustyline::history::SearchDirection;
use std::env;
use std::fs::File;
use std::io::Write;
use std::ops::ControlFlow;
use std::path::Path;

pub fn exit() -> ControlFlow<()> {
    ControlFlow::Break(())
}

pub fn echo(args: &[String], mut output_config: OutputConfig) -> ControlFlow<()> {
    writeln!(output_config.stdout, "{}", args.join(" ")).unwrap();
    ControlFlow::Continue(())
}

pub fn r#type(
    args: &Vec<String>,
    mut output_config: OutputConfig,
) -> Result<ControlFlow<()>, NotFound> {
    for arg in args {
        match BuiltinCommand::try_from(arg.clone()) {
            Ok(_) => {
                writeln!(output_config.stdout, "{} is a shell builtin", arg).unwrap();
            }
            Err(_) => {
                if let Some(path) = PathBins::new()?
                    .0
                    .iter()
                    .find(|bin| &bin.file_name().unwrap().to_string_lossy() == arg)
                {
                    writeln!(output_config.stdout, "{} is {}", arg, path.display()).unwrap()
                } else {
                    writeln!(output_config.stderr, "{}: not found", arg).unwrap()
                }
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}

pub fn pwd(mut output_config: OutputConfig) -> ControlFlow<()> {
    let path = std::env::current_dir().expect("couldn't access current working directory");
    writeln!(output_config.stdout, "{}", path.display()).unwrap();
    ControlFlow::Continue(())
}

pub fn cd(args: &[String]) -> Result<ControlFlow<()>, NotFound> {
    let home = env::home_dir()
        .expect("couldn't get path of current user's HOME directory")
        .to_string_lossy()
        .into();
    let path = if let Some(p) = args.first() {
        if p == "~" {
            home
        } else {
            p.clone()
        }
    } else {
        home
    };
    env::set_current_dir(&path)?;
    Ok(ControlFlow::Continue(()))
}

pub fn executable(
    path: &Path,
    args: &Vec<String>,
    mut output_config: OutputConfig,
) -> ControlFlow<()> {
    let command_out = std::process::Command::new(path.file_name().unwrap())
        .args(args)
        .output()
        .unwrap();
    output_config.stdout.write_all(&command_out.stdout).unwrap();
    output_config.stderr.write_all(&command_out.stderr).unwrap();
    ControlFlow::Continue(())
}

pub fn history(
    editor: &mut Editor<ShellHelper, FileHistory>,
    mut output_config: OutputConfig,
    args: &[String],
) -> Result<ControlFlow<()>, ArgumentError> {
    if args.contains(&"-r".to_string()) {
        let history_file_path = args.get(1).unwrap();
        editor.load_history(history_file_path).unwrap();
    } else if args.contains(&"-w".to_string()) {
        let history_file_path = args.get(1).unwrap();
        let mut history_file = File::create_new(history_file_path).unwrap();
        let history = editor.history();
        let len = history.len();
        for i in 0..len {
            if let Some(result) = history.get(i, SearchDirection::Forward).unwrap() {
                writeln!(history_file, "{}", result.entry).unwrap()
            }
        }
    } else {
        let history = editor.history();
        let len = history.len();

        let n: usize = match args.first() {
            Some(s) => s.parse().map_err(|_| ArgumentError::WrongType)?,
            None => len,
        };

        let start = len.saturating_sub(n);

        for i in start..len {
            if let Some(result) = history.get(i, SearchDirection::Forward).unwrap() {
                writeln!(output_config.stdout, "    {} {}", i + 1, result.entry).unwrap()
            }
        }
    };

    Ok(ControlFlow::Continue(()))
}
