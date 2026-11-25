#![allow(unused)] // TODO: delete this line once the lab is done
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
    todo!("a-rgrep: implement me")
}

/// Matching lines of `text` as (1-based line number, line).
pub fn search_text<'t>(re: &Regex, text: &'t str) -> Vec<(usize, &'t str)> {
    todo!("a-rgrep: implement me")
}

/// A NUL byte in the first 8 KiB means "binary".
pub fn is_binary(bytes: &[u8]) -> bool {
    todo!("a-rgrep: implement me")
}

/// Walk `root`, search every text file in parallel, return sorted matches.
pub fn search_path(root: &Path, re: &Regex, opts: &Options) -> Vec<Match> {
    todo!("a-rgrep: implement me")
}

/// Wrap every match in `HIGHLIGHT_START` / `HIGHLIGHT_END`.
pub fn highlight(line: &str, re: &Regex) -> String {
    todo!("a-rgrep: implement me")
}
