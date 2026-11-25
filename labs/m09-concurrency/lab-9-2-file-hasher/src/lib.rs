//! Lab 9.2 — Find duplicate files in parallel, two ways.
//!
//!   cargo run --release -p lab-9-2-file-hasher -- ~/Downloads
//!
//! Version 1: a hand-made worker pool. Jobs go over one `mpsc` channel; since
//! `Receiver` can't be cloned, workers share it as `Arc<Mutex<Receiver<_>>>`.
//! Results come back over a second channel.
//! Version 2: `rayon`'s `par_iter()`. Compare the line counts.

use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/// Lower-case hex SHA-256 of the file's contents.
pub fn hash_file(path: &Path) -> io::Result<String> {
    // SOLUTION-START
    let bytes = std::fs::read(path)?;
    Ok(hex::encode(Sha256::digest(&bytes)))
    // SOLUTION-END
}

/// Every regular file under `root`, recursively (given).
pub fn list_files(root: &Path) -> Vec<PathBuf> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect()
}

/// Given: group `(path, hash)` pairs into duplicate sets, deterministically
/// ordered (paths sorted inside each group, groups sorted by first path).
pub fn group_duplicates(hashes: impl IntoIterator<Item = (PathBuf, String)>) -> Vec<Vec<PathBuf>> {
    let mut by_hash: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for (path, hash) in hashes {
        by_hash.entry(hash).or_default().push(path);
    }
    let mut groups: Vec<Vec<PathBuf>> = by_hash.into_values().filter(|g| g.len() > 1).collect();
    for g in &mut groups {
        g.sort();
    }
    groups.sort();
    groups
}

/// Version 1: `workers` threads pulling paths from a shared job channel.
/// Unreadable files are skipped.
pub fn find_duplicates_pool(root: &Path, workers: usize) -> Vec<Vec<PathBuf>> {
    // SOLUTION-START
    let (job_tx, job_rx) = mpsc::channel::<PathBuf>();
    let (result_tx, result_rx) = mpsc::channel::<(PathBuf, String)>();
    let job_rx = Arc::new(Mutex::new(job_rx));

    let handles: Vec<_> = (0..workers.max(1))
        .map(|_| {
            let rx = Arc::clone(&job_rx);
            let tx = result_tx.clone();
            thread::spawn(move || {
                loop {
                    // Hold the lock only while receiving, not while hashing.
                    let job = rx.lock().unwrap().recv();
                    let Ok(path) = job else { break }; // channel closed: no more work
                    if let Ok(hash) = hash_file(&path) {
                        let _ = tx.send((path, hash));
                    }
                }
            })
        })
        .collect();
    drop(result_tx); // only workers hold senders now

    for path in list_files(root) {
        job_tx.send(path).unwrap();
    }
    drop(job_tx); // closing the job channel tells workers to stop

    let groups = group_duplicates(result_rx); // Receiver is an iterator until all senders drop
    for h in handles {
        h.join().unwrap();
    }
    groups
    // SOLUTION-END
}

/// Version 2: the same in a few lines with rayon.
pub fn find_duplicates_rayon(root: &Path) -> Vec<Vec<PathBuf>> {
    // SOLUTION-START
    let hashes: Vec<(PathBuf, String)> = list_files(root)
        .into_par_iter()
        .filter_map(|p| hash_file(&p).ok().map(|h| (p, h)))
        .collect();
    group_duplicates(hashes)
    // SOLUTION-END
}
