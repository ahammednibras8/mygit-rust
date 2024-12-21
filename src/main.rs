mod commands; // Declare the `commands` module

use commands::init; // Import the `init` module

fn main() {
    println!("Hello, from main!");

    // Call the init function
    init::init();
}