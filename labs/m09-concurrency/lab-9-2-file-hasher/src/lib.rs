#![allow(unused)] // TODO: delete this line once the lab is done
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
    todo!("lab-9-2-file-hasher: implement me")
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
    todo!("lab-9-2-file-hasher: implement me")
}

/// Version 2: the same in a few lines with rayon.
pub fn find_duplicates_rayon(root: &Path) -> Vec<Vec<PathBuf>> {
    todo!("lab-9-2-file-hasher: implement me")
}
