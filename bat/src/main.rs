use std::fs;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Path to the file
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf, 
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let content = fs::read_to_string(args.path)?; 

    println!("{}", content);
    Ok(())
}
