use crate::builtin_command::BuiltinCommand;
use crate::output::Output;
use crate::path_finder::PathFinder;
use std::env;
use std::io::Write;
use std::ops::ControlFlow;
use std::path::PathBuf;

pub fn exit() -> ControlFlow<()> {
    ControlFlow::Break(())
}

pub fn echo(args: Vec<String>, mut output: Output) -> ControlFlow<()> {
    writeln!(output, "{}", args.join(" ")).unwrap();
    ControlFlow::Continue(())
}

pub fn r#type(args: Vec<String>, mut output: Output) -> ControlFlow<()> {
    for arg in args {
        match BuiltinCommand::try_from(arg.clone()) {
            Ok(_) => {
                writeln!(output, "{} is a shell builtin", arg).unwrap();
            }
            Err(_) => {
                let finder = PathFinder::new(arg.clone());
                match finder.find_executable() {
                    Some(path) => writeln!(output, "{} is {}", arg, path.display()).unwrap(),
                    None => eprintln!("{}: not found", arg),
                }
            }
        }
    }
    ControlFlow::Continue(())
}

pub fn pwd(mut output: Output) -> ControlFlow<()> {
    let path = std::env::current_dir().expect("couldn't access current working directory");
    writeln!(output, "{}", path.display()).unwrap();
    ControlFlow::Continue(())
}

pub fn cd(args: Vec<String>) -> ControlFlow<()> {
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
    env::set_current_dir(&path).expect("couldn't change current working dir");
    ControlFlow::Continue(())
}

pub fn executable(path: PathBuf, args: Vec<String>, mut output: Output) -> ControlFlow<()> {
    let command_out = std::process::Command::new(path)
        .args(args)
        .output()
        .unwrap();
    output.write_all(&command_out.stdout).unwrap();
    std::io::stderr().write_all(&command_out.stderr).unwrap();
    ControlFlow::Continue(())
}
