//! Separate test binary on purpose: without an iterative `impl Drop for List`,
//! dropping a million nodes recurses a million levels deep and overflows the
//! stack, crashing this binary. Write the Drop impl in src/lib.rs to fix it.
use lab_8_2_linked_list::List;

#[test]
fn dropping_a_long_list_does_not_overflow_the_stack() {
    // Run on a thread with a small stack so the failure is reliable.
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let mut l = List::new();
            for i in 0..1_000_000 {
                l.push(i);
            }
            drop(l);
        })
        .unwrap()
        .join()
        .unwrap();
}
