use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

/// (a) Mutating a plain local from several threads.
pub fn a_plain_counter() -> i32 {
    let mut count = 0;
    let handles: Vec<_> = (0..4).map(|_| thread::spawn(|| count += 1)).collect();
    for h in handles {
        h.join().unwrap();
    }
    count
}

/// (b) Sharing an `Rc` across threads.
pub fn b_rc_across_threads() -> usize {
    let data = Rc::new(vec![1, 2, 3]);
    let d = Rc::clone(&data);
    thread::spawn(move || d.len()).join().unwrap()
}

/// (c) Sharing a `RefCell` across threads (even inside an Arc).
pub fn c_refcell_across_threads() -> i32 {
    let cell = Arc::new(RefCell::new(0));
    let c = Arc::clone(&cell);
    thread::spawn(move || *c.borrow_mut() += 1).join().unwrap();
    *cell.borrow()
}
