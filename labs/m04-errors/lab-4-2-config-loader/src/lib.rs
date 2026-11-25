#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 4.2 — Config loader with a hand-written error type.
//!
//! File format:
//! ```text
//! # comments and blank lines are ignored
//! host = example.com
//! port = 8080
//! workers = 8        # optional, defaults to 4
//! ```
//! Implement `Display`, `std::error::Error` and `From<io::Error>` for
//! `ConfigError` yourself (no crates) — Lab 4.3 shows the shortcut.

use std::{error::Error, fmt, fs, io, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    /// A non-comment line without `=`. `line` is 1-based.
    BadLine {
        line: usize,
    },
    /// A required key is missing.
    Missing(&'static str),
    /// A key we don't know.
    UnknownKey {
        key: String,
        line: usize,
    },
    /// A value that doesn't parse for its key (e.g. port = abc).
    InvalidValue {
        key: String,
        value: String,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("lab-4-2-config-loader: implement me")
    }
}

impl Error for ConfigError {
    /// Return the underlying io::Error for the `Io` variant, else `None`.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!("lab-4-2-config-loader: implement me")
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        todo!("lab-4-2-config-loader: implement me")
    }
}

pub fn parse_config(text: &str) -> Result<Config, ConfigError> {
    todo!("lab-4-2-config-loader: implement me")
}

/// Read the file and parse it. Two lines with `?` is enough.
pub fn load_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    todo!("lab-4-2-config-loader: implement me")
}
