use capstone_a_rgrep::*;
use std::fs;

fn fixture() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    let p = d.path();
    fs::create_dir_all(p.join("src/nested")).unwrap();
    fs::create_dir_all(p.join("target")).unwrap();
    fs::create_dir_all(p.join(".hidden")).unwrap();
    fs::write(p.join(".gitignore"), "target/\n*.log\n").unwrap();
    fs::write(
        p.join("src/main.rs"),
        "fn main() {\n    println!(\"Hello\");\n}\n",
    )
    .unwrap();
    fs::write(
        p.join("src/nested/lib.rs"),
        "pub fn helper() {}\n// hello again\n",
    )
    .unwrap();
    fs::write(p.join("target/out.rs"), "fn ignored() {}\n").unwrap();
    fs::write(p.join("debug.log"), "fn also_ignored\n").unwrap();
    fs::write(p.join(".hidden/secret.rs"), "fn secret() {}\n").unwrap();
    fs::write(p.join("image.bin"), b"fn \x00binary").unwrap();
    d
}

#[test]
fn searches_text() {
    let re = build_regex("o+", &Options::default()).unwrap();
    assert_eq!(
        search_text(&re, "foo\nbar\nboo"),
        vec![(1, "foo"), (3, "boo")]
    );
}

#[test]
fn walks_respecting_gitignore_hidden_and_binary() {
    let d = fixture();
    let re = build_regex(r"fn \w+", &Options::default()).unwrap();
    let found: Vec<String> = search_path(d.path(), &re, &Options::default())
        .iter()
        .map(|m| {
            format!(
                "{}:{}",
                m.path.strip_prefix(d.path()).unwrap().display(),
                m.line_no
            )
        })
        .collect();
    assert_eq!(found, ["src/main.rs:1", "src/nested/lib.rs:1"]);
}

#[test]
fn hidden_and_case_options() {
    let d = fixture();
    let opts = Options {
        ignore_case: true,
        hidden: true,
    };
    let re = build_regex("HELLO|secret", &opts).unwrap();
    let hits = search_path(d.path(), &re, &opts);
    assert_eq!(hits.len(), 3, "{hits:#?}");
    assert!(hits.iter().any(|m| m.path.ends_with(".hidden/secret.rs")));
}

#[test]
fn binary_detection() {
    assert!(is_binary(b"abc\0def"));
    assert!(!is_binary(b"plain text"));
    assert!(!is_binary(b""));
}

#[test]
fn highlights_every_match() {
    let re = build_regex("ab", &Options::default()).unwrap();
    assert_eq!(
        highlight("xabyab", &re),
        format!("x{HIGHLIGHT_START}ab{HIGHLIGHT_END}y{HIGHLIGHT_START}ab{HIGHLIGHT_END}")
    );
}
