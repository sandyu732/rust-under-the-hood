use lab_9_2_file_hasher::*;
use std::fs;

fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path();
    fs::create_dir_all(p.join("sub/deeper")).unwrap();
    fs::write(p.join("a.txt"), "same").unwrap();
    fs::write(p.join("sub/b.txt"), "same").unwrap();
    fs::write(p.join("sub/deeper/c.txt"), "same").unwrap();
    fs::write(p.join("unique.txt"), "different").unwrap();
    fs::write(p.join("x.bin"), [0u8; 1024]).unwrap();
    fs::write(p.join("sub/y.bin"), [0u8; 1024]).unwrap();
    for i in 0..50 {
        fs::write(p.join(format!("noise-{i}.txt")), format!("file {i}")).unwrap();
    }
    dir
}

#[test]
fn hashes_known_content() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("abc");
    fs::write(&f, "abc").unwrap();
    assert_eq!(
        hash_file(&f).unwrap(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert!(hash_file(&dir.path().join("missing")).is_err());
}

#[test]
fn pool_finds_duplicates() {
    let dir = fixture();
    let groups = find_duplicates_pool(dir.path(), 4);
    assert_eq!(groups.len(), 2);
    let sizes: Vec<usize> = groups.iter().map(Vec::len).collect();
    assert!(sizes.contains(&3) && sizes.contains(&2), "got {groups:?}");
}

#[test]
fn rayon_matches_pool() {
    let dir = fixture();
    assert_eq!(
        find_duplicates_rayon(dir.path()),
        find_duplicates_pool(dir.path(), 3)
    );
}

#[test]
fn single_worker_and_empty_dir() {
    let dir = fixture();
    assert_eq!(find_duplicates_pool(dir.path(), 1).len(), 2);
    let empty = tempfile::tempdir().unwrap();
    assert!(find_duplicates_pool(empty.path(), 4).is_empty());
}
