use lab_6_1_word_freq::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let path = args.next().ok_or("usage: word-freq <file> [n]")?;
    let n: usize = args.next().map(|s| s.parse()).transpose()?.unwrap_or(20);
    let text = std::fs::read_to_string(path)?;
    let freqs = word_frequencies(&text);
    for (word, count) in top_n(&freqs, n) {
        println!("{count:>8}  {word}");
    }
    Ok(())
}
