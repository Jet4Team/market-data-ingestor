use std::process;

fn main() {
    if let Err(e) = l2_ingestor::run() {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
