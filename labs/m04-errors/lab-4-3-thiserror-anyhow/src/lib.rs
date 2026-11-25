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

// SOLUTION-START
//~ // TODO: write each message with #[error("...")] (copy them from Lab 4.2's Display impl)
//~ // and mark the Io field with #[from] so `?` converts io::Error automatically.
//~ #[derive(Debug, thiserror::Error)]
//~ pub enum ConfigError {
//~     #[error("TODO")]
//~     Io(io::Error),
//~     #[error("TODO")]
//~     BadLine { line: usize },
//~     #[error("TODO")]
//~     Missing(&'static str),
//~     #[error("TODO")]
//~     UnknownKey { key: String, line: usize },
//~     #[error("TODO")]
//~     InvalidValue { key: String, value: String },
//~ }
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("could not read config: {0}")]
    Io(#[from] io::Error),
    #[error("line {line}: expected `key = value`")]
    BadLine { line: usize },
    #[error("missing required key `{0}`")]
    Missing(&'static str),
    #[error("line {line}: unknown key `{key}`")]
    UnknownKey { key: String, line: usize },
    #[error("invalid value `{value}` for `{key}`")]
    InvalidValue { key: String, value: String },
}
// SOLUTION-END

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

pub fn load_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    // SOLUTION-START
    parse_config(&fs::read_to_string(path)?)
    // SOLUTION-END
}

/// Application-level loader: wrap any error with the context
/// `"loading config from <path>"` (use `.with_context(|| format!(...))`).
pub fn load_for_app(path: &str) -> anyhow::Result<Config> {
    // SOLUTION-START
    load_config(path).with_context(|| format!("loading config from {path}"))
    // SOLUTION-END
}
