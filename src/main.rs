mod commands;

use commands::init;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("init") => {
            if let Err(err) = init::init() {
                eprintln!("Error: {}", err);
            }
        }
        _ => eprintln!("Usage: mygit <command>"),
    }
}
