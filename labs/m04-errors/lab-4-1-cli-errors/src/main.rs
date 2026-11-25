use std::error::Error;

// Returning Result from main: an Err prints "Error: ..." and exits with code 1.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("{}", lab_4_1_cli_errors::run(&args)?);
    Ok(())
}
