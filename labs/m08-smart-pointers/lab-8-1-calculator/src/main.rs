use lab_8_1_calculator::*;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    println!("calc — type an expression, Ctrl-D to quit");
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        if stdin.lock().read_line(&mut line)? == 0 {
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        match calculate(&line) {
            Ok(v) => println!("{v}"),
            Err(e) => println!("error: {e}"),
        }
    }
    Ok(())
}
