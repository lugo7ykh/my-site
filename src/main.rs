use std::process;

fn main() {
    if let Err(e) = my_site::run() {
        eprintln!("app error: {}", e);
        process::exit(1);
    };
}
