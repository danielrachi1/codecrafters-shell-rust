use rustyline::Helper;
use rustyline::completion::Completer;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use std::borrow::Cow;
use std::vec;

use crate::path_bins::PathBins;

pub struct ShellHelper;

impl ShellHelper {
    pub fn default() -> Self {
        ShellHelper
    }
}

impl Completer for ShellHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        // For now, only the command is replaced
        let prefix = line;

        if prefix == "ech" && pos == line.len() {
            return Ok((0, vec!["echo ".to_string()]));
        } else if prefix == "exi" && pos == line.len() {
            return Ok((0, vec!["exit ".to_string()]));
        }

        if let Some(path) = PathBins::new()?.0.iter().find(|bin| {
            bin.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with(prefix)
        }) {
            let completion = path.file_name().unwrap().to_string_lossy().to_string() + " ";
            Ok((0, vec![completion]))
        } else {
            Ok((0, vec![]))
        }
    }
}

impl Hinter for ShellHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Highlighter for ShellHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _kind: CmdKind) -> bool {
        false
    }
}

impl Validator for ShellHelper {}

impl Helper for ShellHelper {}
