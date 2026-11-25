#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 4.3 — The same config loader, using the ecosystem.
//!
//! * Library code: a typed error with `thiserror` (callers can match on it).
//! * Application code: `anyhow::Result` + `.context(...)` (just report it).
//!
//! Rewrite Lab 4.2's error using `#[derive(thiserror::Error)]`,
//! `#[error("...")]` and `#[from]`. Compare how many lines you saved.

use anyhow::Context;
use std::{fs, io, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

// TODO: write each message with #[error("...")] (copy them from Lab 4.2's Display impl)
// and mark the Io field with #[from] so `?` converts io::Error automatically.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("TODO")]
    Io(io::Error),
    #[error("TODO")]
    BadLine { line: usize },
    #[error("TODO")]
    Missing(&'static str),
    #[error("TODO")]
    UnknownKey { key: String, line: usize },
    #[error("TODO")]
    InvalidValue { key: String, value: String },
}

pub fn parse_config(text: &str) -> Result<Config, ConfigError> {
    todo!("lab-4-3-thiserror-anyhow: implement me")
}

pub fn load_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    todo!("lab-4-3-thiserror-anyhow: implement me")
}

/// Application-level loader: wrap any error with the context
/// `"loading config from <path>"` (use `.with_context(|| format!(...))`).
pub fn load_for_app(path: &str) -> anyhow::Result<Config> {
    todo!("lab-4-3-thiserror-anyhow: implement me")
}
