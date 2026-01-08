use rustyline::Editor;
use rustyline::history::History;
use rustyline::sqlite_history::SQLiteHistory;

use crate::builtin_command::BuiltinCommand;
use crate::error::not_found::NotFound;
use crate::output_config::OutputConfig;
use crate::path_bins::PathBins;
use crate::shell_helper::ShellHelper;
use rustyline::history::SearchDirection;
use std::env;
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
        if p == "~" { home } else { p.clone() }
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
    editor: &Editor<ShellHelper, SQLiteHistory>,
    mut output_config: OutputConfig,
) -> ControlFlow<()> {
    let history = editor.history();
    let len = history.len();

    for i in 0..len {
        if let Some(result) = history.get(i, SearchDirection::Forward).unwrap() {
            writeln!(output_config.stdout, "    {} {}", i + 1, result.entry).unwrap()
        }
    }
    ControlFlow::Continue(())
}
