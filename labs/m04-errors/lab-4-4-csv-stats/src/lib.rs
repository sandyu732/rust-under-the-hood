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

// SOLUTION-ONLY-START
fn column_index(text: &str, column: &str) -> Result<usize, CsvError> {
    let header = text.lines().next().ok_or(CsvError::Empty)?;
    header
        .split(',')
        .position(|h| h.trim() == column)
        .ok_or_else(|| CsvError::MissingColumn(column.to_string()))
}

fn parse_row(line_no: usize, row: &str, idx: usize) -> Result<f64, CsvError> {
    let field = row
        .split(',')
        .nth(idx)
        .ok_or(CsvError::ShortRow { line: line_no })?
        .trim();
    field.parse().map_err(|_| CsvError::BadValue {
        line: line_no,
        value: field.to_string(),
    })
}

fn data_rows(text: &str) -> impl Iterator<Item = (usize, &str)> {
    text.lines()
        .enumerate()
        .skip(1)
        .map(|(i, l)| (i + 1, l))
        .filter(|(_, l)| !l.trim().is_empty())
}
// SOLUTION-ONLY-END

/// Fail on the first bad row. Blank lines are skipped.
pub fn parse_strict(text: &str, column: &str) -> Result<Vec<f64>, CsvError> {
    // SOLUTION-START
    let idx = column_index(text, column)?;
    data_rows(text)
        .map(|(n, row)| parse_row(n, row, idx))
        .collect()
    // SOLUTION-END
}

/// Keep going: return the good values and every row error.
/// A missing column / empty input is still a hard error.
pub fn parse_lenient(text: &str, column: &str) -> Result<(Vec<f64>, Vec<CsvError>), CsvError> {
    // SOLUTION-START
    let idx = column_index(text, column)?;
    let (ok, bad): (Vec<_>, Vec<_>) = data_rows(text)
        .map(|(n, row)| parse_row(n, row, idx))
        .partition(Result::is_ok);
    Ok((
        ok.into_iter().map(Result::unwrap).collect(),
        bad.into_iter().map(|r| r.unwrap_err()).collect(),
    ))
    // SOLUTION-END
}

/// `None` for an empty slice. Median of an even count is the mean of the middle two.
pub fn stats(values: &[f64]) -> Option<Stats> {
    // SOLUTION-START
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    let median = if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    };
    Some(Stats {
        count: n,
        min: sorted[0],
        max: sorted[n - 1],
        mean: sorted.iter().sum::<f64>() / n as f64,
        median,
    })
    // SOLUTION-END
}
