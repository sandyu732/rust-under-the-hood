use lab_1_2_overflow::*;

fn main() {
    // Reading from args stops the compiler from folding the overflow at compile time.
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (a, b) = match (
        args.first().map(|s| s.parse::<u8>()),
        args.get(1).map(|s| s.parse::<u8>()),
    ) {
        (Some(Ok(a)), Some(Ok(b))) => (a, b),
        _ => {
            eprintln!("usage: lab-1-2-overflow <a: u8> <b: u8>   e.g. 250 10");
            std::process::exit(1);
        }
    };
    println!("checked     = {:?}", checked(a, b));
    println!("wrapping    = {}", wrapping(a, b));
    println!("saturating  = {}", saturating(a, b));
    println!("overflowing = {:?}", overflowing(a, b));
    println!(
        "plain `+`   = {}   (panics in debug, wraps in --release)",
        plain_add(a, b)
    );
}
