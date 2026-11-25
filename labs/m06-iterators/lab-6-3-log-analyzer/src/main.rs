use lab_6_3_log_analyzer::*;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("gen") => {
            let n = args
                .get(1)
                .map(|s| s.parse())
                .transpose()?
                .unwrap_or(100_000);
            print!("{}", generate(n));
        }
        Some("analyze") => {
            let path = args.get(1).ok_or("usage: analyze <file>")?;
            let start = Instant::now();
            let text = std::fs::read_to_string(path)?;
            let (lines, bad): (Vec<_>, Vec<_>) =
                text.lines().map(parse_line).partition(Result::is_ok);
            let lines: Vec<LogLine> = lines.into_iter().map(Result::unwrap).collect();
            let mut levels: Vec<_> = count_by_level(&lines).into_iter().collect();
            levels.sort();
            println!("lines: {} ok, {} bad", lines.len(), bad.len());
            println!("by level: {levels:?}");
            println!("slowest: {:#?}", slowest_endpoints(&lines, 5));
            println!("p95: {:#?}", p95_by_endpoint(&lines));
            println!("took {:?}", start.elapsed());
        }
        _ => eprintln!("usage: log-analyzer gen <n> | analyze <file>"),
    }
    Ok(())
}
