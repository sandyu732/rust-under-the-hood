#![allow(unused)] // TODO: delete this line once the lab is done
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
    todo!("lab-9-3-pipeline: implement me")
}

/// Run the pipeline. Order of records must be preserved.
pub fn run_pipeline<I>(lines: I, capacity: usize) -> Report
where
    I: IntoIterator<Item = String> + Send + 'static,
    I::IntoIter: Send,
{
    todo!("lab-9-3-pipeline: implement me")
}
