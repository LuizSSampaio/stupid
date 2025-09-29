use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about = "A stupid language compiler", long_about = None)]
struct Args {
    /// Path to the source file
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.file.display());
}
