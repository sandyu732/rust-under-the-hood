use lab_6_4_zero_cost::*;
use std::hint::black_box;
use std::time::Instant;

fn time(label: &str, f: impl Fn(&[u64]) -> u64, data: &[u64]) {
    let start = Instant::now();
    let mut result = 0;
    for _ in 0..5 {
        result = f(black_box(data));
    }
    println!(
        "{label:<8} {:>10.2?} per run  (result {result})",
        start.elapsed() / 5
    );
}

fn main() {
    if cfg!(debug_assertions) {
        eprintln!("warning: debug build — run with --release for meaningful numbers");
    }
    let data: Vec<u64> = (0..50_000_000u64).map(|x| x % 1000).collect();
    time("index", sum_odd_squares_index, &data);
    time("for", sum_odd_squares_for, &data);
    time("iter", sum_odd_squares_iter, &data);
}
