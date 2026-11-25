use capstone_a_rgrep::*;
use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

/// Search files recursively for a regex.
#[derive(Parser)]
#[command(name = "rgrep", version)]
struct Cli {
    /// Regular expression to search for
    pattern: String,
    /// Directory or file to search
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Case-insensitive search
    #[arg(short, long)]
    ignore_case: bool,
    /// Include hidden files and directories
    #[arg(long)]
    hidden: bool,
    /// Only print the number of matching lines
    #[arg(short, long)]
    count: bool,
    /// Disable colour
    #[arg(long)]
    no_color: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let opts = Options {
        ignore_case: cli.ignore_case,
        hidden: cli.hidden,
    };
    let re = match build_regex(&cli.pattern, &opts) {
        Ok(re) => re,
        Err(e) => {
            eprintln!("rgrep: invalid pattern: {e}");
            return ExitCode::from(2);
        }
    };
    let matches = search_path(&cli.path, &re, &opts);
    if cli.count {
        println!("{}", matches.len());
    } else {
        for m in &matches {
            let line = if cli.no_color {
                m.line.clone()
            } else {
                highlight(&m.line, &re)
            };
            println!("{}:{}:{}", m.path.display(), m.line_no, line);
        }
    }
    if matches.is_empty() {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
