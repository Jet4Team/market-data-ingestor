use std::process;

fn main() {
    if let Err(e) = binance_ingestor::run() {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
