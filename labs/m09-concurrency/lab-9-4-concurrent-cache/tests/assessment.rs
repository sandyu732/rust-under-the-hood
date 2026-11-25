use lab_9_4_concurrent_cache::Cache;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn basic_operations() {
    let c = Cache::new();
    assert!(c.is_empty());
    c.insert("a", 1);
    assert_eq!(c.get(&"a"), Some(1));
    assert_eq!(c.get(&"b"), None);
    assert_eq!(c.get_or_insert_with("b", || 2), 2);
    assert_eq!(c.get_or_insert_with("b", || 99), 2, "existing value wins");
    assert_eq!(c.len(), 2);
}

#[test]
fn hammered_from_sixteen_threads() {
    for _ in 0..20 {
        let cache: Cache<u32, String> = Cache::new();
        let calls = AtomicUsize::new(0);
        let results: Vec<Vec<String>> = thread::scope(|s| {
            let hs: Vec<_> = (0..16)
                .map(|_| {
                    s.spawn(|| {
                        (0..50u32)
                            .map(|k| {
                                cache.get_or_insert_with(k, || {
                                    calls.fetch_add(1, Ordering::Relaxed);
                                    format!("v{k}")
                                })
                            })
                            .collect()
                    })
                })
                .collect();
            hs.into_iter().map(|h| h.join().unwrap()).collect()
        });
        assert_eq!(cache.len(), 50);
        for r in &results {
            assert_eq!(r[7], "v7");
        }
        assert!(calls.load(Ordering::Relaxed) >= 50);
    }
}

#[test]
fn does_not_hold_the_lock_while_computing() {
    // `f` reads the cache. If get_or_insert_with held the write lock during
    // `f`, this would deadlock; we detect that with a timeout.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let cache: Cache<&str, i32> = Cache::new();
        cache.insert("base", 10);
        let v = cache.get_or_insert_with("derived", || cache.get(&"base").unwrap() * 2);
        tx.send(v).unwrap();
    });
    let v = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("deadlock: lock held while computing");
    assert_eq!(v, 20);
}
