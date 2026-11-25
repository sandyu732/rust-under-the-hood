use lab_9_2_file_hasher::*;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    let root = PathBuf::from(std::env::args().nth(1).unwrap_or_else(|| ".".into()));
    let t = Instant::now();
    let pool = find_duplicates_pool(&root, 8);
    println!(
        "worker pool: {} duplicate groups in {:?}",
        pool.len(),
        t.elapsed()
    );
    let t = Instant::now();
    let ray = find_duplicates_rayon(&root);
    println!(
        "rayon:       {} duplicate groups in {:?}",
        ray.len(),
        t.elapsed()
    );
    for group in pool.iter().take(10) {
        println!("  {group:?}");
    }
}
