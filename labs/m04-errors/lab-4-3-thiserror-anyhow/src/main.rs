//! `cargo run -p lab-4-3-thiserror-anyhow -- app.conf`
//! Try a missing file and a bad port to see anyhow's "Caused by:" chain.
fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "app.conf".to_string());
    let cfg = lab_4_3_thiserror_anyhow::load_for_app(&path)?;
    println!("{cfg:#?}");
    Ok(())
}
