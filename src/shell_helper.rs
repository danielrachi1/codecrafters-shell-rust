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

    fn get_candidates(&self, prefix: &str) -> rustyline::Result<Vec<String>> {
        let mut candidates: Vec<String> = PathBins::new()?
            .0
            .iter()
            .filter_map(|bin| {
                bin.file_name().and_then(|name| {
                    let name_str = name.to_string_lossy();
                    if name_str.starts_with(prefix) {
                        Some(name_str.to_string())
                    } else {
                        None
                    }
                })
            })
            .collect();

        candidates.sort();
        Ok(candidates)
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
            return Ok((0, vec!["echo".to_string()]));
        } else if prefix == "exi" && pos == line.len() {
            return Ok((0, vec!["exit".to_string()]));
        }

        let candidates = self.get_candidates(prefix)?;
        Ok((0, candidates))
    }

    fn update(
        &self,
        line: &mut rustyline::line_buffer::LineBuffer,
        start: usize,
        elected: &str,
        cl: &mut rustyline::Changeset,
    ) {
        // Check if there are other candidates that start with the elected string
        let has_more_candidates = self
            .get_candidates(elected)
            .map(|candidates| candidates.iter().any(|c| c != elected))
            .unwrap_or(false);

        // If there are other candidates, it's a partial completion (no space)
        // Otherwise, it's a full completion (add space)
        let replacement = if has_more_candidates {
            elected.to_string()
        } else {
            format!("{} ", elected)
        };

        let end = line.pos();
        line.replace(start..end, &replacement, cl);
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
