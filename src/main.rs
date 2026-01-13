mod argument_parser;
mod builtin_command;
mod command;
mod error;
mod input;
mod order;
mod output;
mod output_config;
mod path_bins;
mod runner;
mod shell;
mod shell_helper;

use crate::order::Order;
use crate::shell_helper::ShellHelper;
use rustyline::{Config, Editor, Result, history::FileHistory};
use std::{env, ops::ControlFlow};

fn main() -> Result<()> {
    let helper = ShellHelper::default();
    let config = Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .auto_add_history(true)
        .build();
    let history = FileHistory::with_config(&config);
    let mut rl: Editor<ShellHelper, FileHistory> = Editor::with_history(config, history)?;
    rl.set_helper(Some(helper));

    if let Ok(history_file_path) = env::var("HISTFILE") {
        rl.load_history(&history_file_path).unwrap();
    }

    loop {
        let line = rl.readline("$ ")?;

        let Some(pipeline) = shell::input(&mut rl, line) else {
            continue;
        };

        let mut should_exit = false;

        // Check if all commands in the pipeline are executable (external commands)
        // If so, we can use real OS pipes for true concurrent execution
        let all_executable = pipeline
            .iter()
            .all(|(cmd, _, _)| matches!(cmd, crate::command::Command::Executable(_)));

        let pipeline_len = pipeline.len();

        if all_executable && pipeline_len > 1 {
            // Use OS pipes for external commands
            use std::process::{Command, Stdio};
            use std::os::unix::io::{FromRawFd, IntoRawFd};

            let mut children = Vec::new();

            for (index, (command, args, _output_config)) in pipeline.into_iter().enumerate() {
                let crate::command::Command::Executable(path) = command else {
                    unreachable!()
                };

                let mut cmd = Command::new(path.file_name().unwrap());
                cmd.args(&args);

                // Setup stdin from previous command or inherit
                if index == 0 {
                    cmd.stdin(Stdio::inherit());
                } else {
                    cmd.stdin(Stdio::piped());
                }

                // Setup stdout: pipe to next command or use configured output
                let is_last = index == pipeline_len - 1;
                if is_last {
                    // Last command: use real stdout (or configured output if file redirect)
                    cmd.stdout(Stdio::inherit());
                } else {
                    // Intermediate command: create pipe to next command
                    cmd.stdout(Stdio::piped());
                }

                cmd.stderr(Stdio::inherit());

                children.push(cmd);
            }

            // Spawn all processes and connect their pipes
            let mut processes = Vec::new();
            let mut prev_stdout: Option<std::process::ChildStdout> = None;

            for (index, mut cmd) in children.into_iter().enumerate() {
                // Connect stdin to previous command's stdout
                if let Some(stdout) = prev_stdout.take() {
                    unsafe {
                        let raw_fd = stdout.into_raw_fd();
                        cmd.stdin(Stdio::from_raw_fd(raw_fd));
                    }
                }

                let mut child = cmd.spawn().unwrap();

                // Save stdout for next command
                if index < pipeline_len - 1 {
                    prev_stdout = child.stdout.take();
                }

                processes.push(child);
            }

            // Wait for all children to complete
            for mut child in processes {
                child.wait().unwrap();
            }
        } else {
            // Fallback: sequential execution for builtins or single commands
            let pipeline_len = pipeline.len();
            let mut stdin_data: Option<Vec<u8>> = None;

            for (index, (command, args, mut output_config)) in pipeline.into_iter().enumerate() {
                // If this is not the last command in the pipeline, capture its output
                let is_last = index == pipeline_len - 1;
                if !is_last {
                    // Replace stdout with a pipe to capture output
                    output_config.stdout = crate::output::Output::Pipe(Vec::new());
                }

                let order = Order::new(command, args, output_config, &mut rl, stdin_data);
                match order.execute() {
                    (ControlFlow::Continue(_), captured_output) => {
                        // Pass the captured output to the next command
                        stdin_data = captured_output;
                    }
                    (ControlFlow::Break(_), _) => {
                        should_exit = true;
                        break;
                    }
                }
            }
        }

        if should_exit {
            break;
        }
    }

    if let Ok(history_file_path) = env::var("HISTFILE") {
        rl.save_history(&history_file_path).unwrap();
    }

    Ok(())
}
