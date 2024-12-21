mod commands;

use commands::init;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let args = &args[1..];

    match args.get(1).map(String::as_str) {
        Some("init") => {
            let target_dir = args.get(2).map_or(".", String::as_str);
            if let Err(err) = init::init(target_dir) {
                eprintln!("{}", err);
            }
        }
        _ => {
            eprintln!("Usage: mygit <command>");
            eprintln!("Commands:");
            eprintln!("  init [path]   Initialize a new mygit repository in the specified path");
        }
    }
}
