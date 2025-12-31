use crate::builtin_command::BuiltinCommand;
use crate::path_finder::PathFinder;
use std::env;
use std::ops::ControlFlow;
use std::path::PathBuf;

pub fn exit() -> ControlFlow<()> {
    ControlFlow::Break(())
}

pub fn echo(args: Vec<String>) -> ControlFlow<()> {
    println!("{}", args.join(" "));
    ControlFlow::Continue(())
}

pub fn r#type(args: Vec<String>) -> ControlFlow<()> {
    for arg in args {
        match BuiltinCommand::try_from(arg.clone()) {
            Ok(_) => {
                println!("{} is a shell builtin", arg)
            }
            Err(_) => {
                let finder = PathFinder::new(arg.clone());
                match finder.find_executable() {
                    Some(path) => println!("{} is {}", arg, path.display()),
                    None => println!("{}: not found", arg),
                }
            }
        }
    }

    ControlFlow::Continue(())
}

pub fn pwd() -> ControlFlow<()> {
    let path = std::env::current_dir().unwrap();
    println!("{}", path.display());

    ControlFlow::Continue(())
}

pub fn cd(args: Vec<String>) -> ControlFlow<()> {
    let home = env::home_dir().unwrap().to_string_lossy().into();
    let path = if let Some(p) = args.first() {
        if p == "~" { home } else { p.clone() }
    } else {
        home
    };
    env::set_current_dir(&path).unwrap();

    ControlFlow::Continue(())
}

pub fn executable(path: PathBuf, args: Vec<String>) -> ControlFlow<()> {
    std::process::Command::new(path)
        .args(args)
        .status()
        .unwrap();

    ControlFlow::Continue(())
}
