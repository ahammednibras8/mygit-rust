mod commands;

use commands::{add, init};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    //Remove program name
    let args = &args[1..];

    match args.get(0).map(String::as_str) {
        //mygit init:
        Some("init") => {
            let target_dir = args.get(1).map_or(".", String::as_str);
            if let Err(err) = init::init(target_dir) {
                eprintln!("Error: {}", err);
            }
        }

        //mygit add:
        Some("add") => {
            if let Some(file_path) = args.get(1) {
                if let Err(err) = add::add(file_path) {
                    eprintln!("Error: {}", err);
                }
            } else {
                eprintln!("Usage: mygit add <file>");
            }
        }

        //Unknown command:
        _ => {
            eprintln!("Usage: mygit <command>");
            eprintln!("Commands:");
            eprintln!("  init [path]   Initialize a new mygit repository in the specified path");
            eprintln!("  add <file>    Add a file to the staging area");
        }
    }
}
