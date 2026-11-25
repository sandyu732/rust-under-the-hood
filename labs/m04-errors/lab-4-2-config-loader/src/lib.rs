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
        // SOLUTION-START
        match self {
            ConfigError::Io(e) => write!(f, "could not read config: {e}"),
            ConfigError::BadLine { line } => write!(f, "line {line}: expected `key = value`"),
            ConfigError::Missing(key) => write!(f, "missing required key `{key}`"),
            ConfigError::UnknownKey { key, line } => write!(f, "line {line}: unknown key `{key}`"),
            ConfigError::InvalidValue { key, value } => {
                write!(f, "invalid value `{value}` for `{key}`")
            }
        }
        // SOLUTION-END
    }
}

impl Error for ConfigError {
    /// Return the underlying io::Error for the `Io` variant, else `None`.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // SOLUTION-START
        match self {
            ConfigError::Io(e) => Some(e),
            _ => None,
        }
        // SOLUTION-END
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        // SOLUTION-START
        ConfigError::Io(e)
        // SOLUTION-END
    }
}

pub fn parse_config(text: &str) -> Result<Config, ConfigError> {
    // SOLUTION-START
    let (mut host, mut port, mut workers) = (None, None, None);
    for (idx, raw) in text.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or(ConfigError::BadLine { line: line_no })?;
        let (key, value) = (key.trim(), value.trim());
        let invalid = || ConfigError::InvalidValue {
            key: key.to_string(),
            value: value.to_string(),
        };
        match key {
            "host" if !value.is_empty() => host = Some(value.to_string()),
            "host" => return Err(invalid()),
            "port" => port = Some(value.parse::<u16>().map_err(|_| invalid())?),
            "workers" => workers = Some(value.parse::<usize>().map_err(|_| invalid())?),
            _ => {
                return Err(ConfigError::UnknownKey {
                    key: key.to_string(),
                    line: line_no,
                });
            }
        }
    }
    Ok(Config {
        host: host.ok_or(ConfigError::Missing("host"))?,
        port: port.ok_or(ConfigError::Missing("port"))?,
        workers: workers.unwrap_or(4),
    })
    // SOLUTION-END
}

/// Read the file and parse it. Two lines with `?` is enough.
pub fn load_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    // SOLUTION-START
    let text = fs::read_to_string(path)?;
    parse_config(&text)
    // SOLUTION-END
}
