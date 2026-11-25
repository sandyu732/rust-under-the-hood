//! Lab 9.3 — A bounded three-stage pipeline with clean shutdown.
//!
//!   reader thread ──sync_channel(cap)──▶ parser thread ──sync_channel(cap)──▶ collector (caller)
//!
//! `sync_channel(cap)` blocks the sender when the buffer is full, which gives
//! you back-pressure: a fast reader can't run ahead and fill memory.
//! Shutdown needs no flags: when a stage finishes, it drops its `Sender`; the
//! next stage's `for msg in rx` loop then ends naturally.

use std::sync::mpsc::sync_channel;
use std::thread;

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    pub score: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub records: Vec<Record>,
    /// Lines that didn't parse as `name,score`.
    pub rejected: usize,
}

/// `"alice,42"` → `Some(Record)`. Anything else → `None`.
pub fn parse_record(line: &str) -> Option<Record> {
    // SOLUTION-START
    let (name, score) = line.split_once(',')?;
    let name = name.trim();
    if name.is_empty() {
        return None;
    }
    Some(Record {
        name: name.to_string(),
        score: score.trim().parse().ok()?,
    })
    // SOLUTION-END
}

/// Run the pipeline. Order of records must be preserved.
pub fn run_pipeline<I>(lines: I, capacity: usize) -> Report
where
    I: IntoIterator<Item = String> + Send + 'static,
    I::IntoIter: Send,
{
    // SOLUTION-START
    let (line_tx, line_rx) = sync_channel::<String>(capacity);
    let (rec_tx, rec_rx) = sync_channel::<Option<Record>>(capacity);

    let reader = thread::spawn(move || {
        for line in lines {
            if line_tx.send(line).is_err() {
                break; // downstream hung up
            }
        }
        // line_tx dropped here → parser's loop ends
    });

    let parser = thread::spawn(move || {
        for line in line_rx {
            if rec_tx.send(parse_record(&line)).is_err() {
                break;
            }
        }
        // rec_tx dropped here → collector's loop ends
    });

    let mut report = Report {
        records: Vec::new(),
        rejected: 0,
    };
    for parsed in rec_rx {
        match parsed {
            Some(r) => report.records.push(r),
            None => report.rejected += 1,
        }
    }
    reader.join().unwrap();
    parser.join().unwrap();
    report
    // SOLUTION-END
}
