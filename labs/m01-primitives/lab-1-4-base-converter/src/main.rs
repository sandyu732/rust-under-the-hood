fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match lab_1_4_base_converter::run(&args) {
        Ok(out) => println!("{out}"),
        Err(msg) => {
            eprintln!("error: {msg}");
            std::process::exit(1);
        }
    }
}
