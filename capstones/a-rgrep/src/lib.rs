//! Capstone A — `rgrep`, a ripgrep-lite.
//!
//!   cargo run --release -p capstone-a-rgrep -- 'fn \w+' labs/ -i
//!
//! Requirements (the assessment checks these):
//! * regex search, optional case-insensitivity
//! * recursive walk that respects `.gitignore` and skips hidden files unless asked
//!   (the `ignore` crate's `WalkBuilder` does both — set `.require_git(false)`
//!   so `.gitignore` works outside a git repo)
//! * binary files (a NUL byte in the first 8 KiB) are skipped
//! * files searched in parallel (rayon); output sorted by path then line
//! * ANSI highlighting of matches
//!
//! Stretch: count mode, `--glob`, context lines (`-C 2`), benchmark vs
//! `grep -r` with `hyperfine`.

use rayon::prelude::*;
use regex::{Regex, RegexBuilder};
use std::path::{Path, PathBuf};

pub const HIGHLIGHT_START: &str = "\x1b[1;31m";
pub const HIGHLIGHT_END: &str = "\x1b[0m";

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub ignore_case: bool,
    pub hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    pub path: PathBuf,
    /// 1-based
    pub line_no: usize,
    pub line: String,
}

pub fn build_regex(pattern: &str, opts: &Options) -> Result<Regex, regex::Error> {
    // SOLUTION-START
    RegexBuilder::new(pattern)
        .case_insensitive(opts.ignore_case)
        .build()
    // SOLUTION-END
}

/// Matching lines of `text` as (1-based line number, line).
pub fn search_text<'t>(re: &Regex, text: &'t str) -> Vec<(usize, &'t str)> {
    // SOLUTION-START
    text.lines()
        .enumerate()
        .filter(|(_, l)| re.is_match(l))
        .map(|(i, l)| (i + 1, l))
        .collect()
    // SOLUTION-END
}

/// A NUL byte in the first 8 KiB means "binary".
pub fn is_binary(bytes: &[u8]) -> bool {
    // SOLUTION-START
    bytes[..bytes.len().min(8192)].contains(&0)
    // SOLUTION-END
}

/// Walk `root`, search every text file in parallel, return sorted matches.
pub fn search_path(root: &Path, re: &Regex, opts: &Options) -> Vec<Match> {
    // SOLUTION-START
    let files: Vec<PathBuf> = ignore::WalkBuilder::new(root)
        .hidden(!opts.hidden)
        .require_git(false)
        .build()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_some_and(|t| t.is_file()))
        .map(|e| e.into_path())
        .collect();
    let mut matches: Vec<Match> = files
        .par_iter()
        .flat_map_iter(|path| {
            let bytes = std::fs::read(path).unwrap_or_default();
            if is_binary(&bytes) {
                return Vec::new();
            }
            let text = String::from_utf8_lossy(&bytes);
            search_text(re, &text)
                .into_iter()
                .map(|(line_no, line)| Match {
                    path: path.clone(),
                    line_no,
                    line: line.to_string(),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    matches.sort_by(|a, b| a.path.cmp(&b.path).then(a.line_no.cmp(&b.line_no)));
    matches
    // SOLUTION-END
}

/// Wrap every match in `HIGHLIGHT_START` / `HIGHLIGHT_END`.
pub fn highlight(line: &str, re: &Regex) -> String {
    // SOLUTION-START
    re.replace_all(line, |caps: &regex::Captures| {
        format!("{HIGHLIGHT_START}{}{HIGHLIGHT_END}", &caps[0])
    })
    .into_owned()
    // SOLUTION-END
}
