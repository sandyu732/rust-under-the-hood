#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 9.4 — A thread-safe memoising cache.
//!
//! `get_or_insert_with` must NOT hold the write lock while running `f`:
//! `f` can be slow (a DB call), and it may even read the cache itself —
//! which would deadlock. Pattern: read-lock check → compute unlocked →
//! write-lock insert (another thread may have won the race: keep the first value).

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;

pub struct Cache<K, V> {
    inner: RwLock<HashMap<K, V>>,
}

impl<K: Eq + Hash + Clone, V: Clone> Cache<K, V> {
    pub fn new() -> Self {
        Cache {
            inner: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        todo!("lab-9-4-concurrent-cache: implement me")
    }

    pub fn insert(&self, key: K, value: V) {
        todo!("lab-9-4-concurrent-cache: implement me")
    }

    pub fn len(&self) -> usize {
        todo!("lab-9-4-concurrent-cache: implement me")
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the cached value, or compute it with `f`, store it and return it.
    /// If two threads race, both may run `f`, but every caller gets the SAME
    /// stored value (the first one inserted).
    pub fn get_or_insert_with(&self, key: K, f: impl FnOnce() -> V) -> V {
        todo!("lab-9-4-concurrent-cache: implement me")
    }
}

impl<K: Eq + Hash + Clone, V: Clone> Default for Cache<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
