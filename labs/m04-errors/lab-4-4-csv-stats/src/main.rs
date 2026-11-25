//! `cargo run -p lab-4-4-csv-stats -- data.csv price [--lenient]`
use lab_4_4_csv_stats::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = args
        .first()
        .ok_or("usage: csv-stats <file> <column> [--lenient]")?;
    let column = args.get(1).ok_or("missing <column>")?;
    let text = std::fs::read_to_string(path)?;
    let values = if args.iter().any(|a| a == "--lenient") {
        let (values, errors) = parse_lenient(&text, column)?;
        for e in &errors {
            eprintln!("skipped: {e}");
        }
        values
    } else {
        parse_strict(&text, column)?
    };
    match stats(&values) {
        Some(s) => println!("{s:#?}"),
        None => println!("no values"),
    }
    Ok(())
}
