use std::process;

fn main() {
    if let Err(e) = binance::run() {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
