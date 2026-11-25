#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 4.4 — CSV stats with fail-fast and lenient modes.
//!
//! The first line is a header. Pick the column by name. Line numbers in
//! errors are 1-based and count the header as line 1.
//!
//! Strict mode: the trick is `.collect::<Result<Vec<f64>, CsvError>>()` — an
//! iterator of Results collects into a Result of Vec, stopping at the first Err.

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CsvError {
    #[error("empty input")]
    Empty,
    #[error("no column named `{0}`")]
    MissingColumn(String),
    #[error("line {line}: row has too few fields")]
    ShortRow { line: usize },
    #[error("line {line}: `{value}` is not a number")]
    BadValue { line: usize, value: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub count: usize,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
}

/// Fail on the first bad row. Blank lines are skipped.
pub fn parse_strict(text: &str, column: &str) -> Result<Vec<f64>, CsvError> {
    todo!("lab-4-4-csv-stats: implement me")
}

/// Keep going: return the good values and every row error.
/// A missing column / empty input is still a hard error.
pub fn parse_lenient(text: &str, column: &str) -> Result<(Vec<f64>, Vec<CsvError>), CsvError> {
    todo!("lab-4-4-csv-stats: implement me")
}

/// `None` for an empty slice. Median of an even count is the mean of the middle two.
pub fn stats(values: &[f64]) -> Option<Stats> {
    todo!("lab-4-4-csv-stats: implement me")
}
