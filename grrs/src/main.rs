/*
 * Rust CLI Book
 * https://rust-cli.github.io/book/
 *  
 */

// command line arguments parser library
use clap::Parser;

 /// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
 // Struct for command line arguments
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read. Type PathBuf
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();

    println!("pattern: {}, path: {}", args.pattern, args.path.display());
}
