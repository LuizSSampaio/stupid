use std::{fs::File, io::Read, path::PathBuf};

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

fn run(path: PathBuf) {
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", display, why),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(why) => panic!("couldn't read {}: {}", display, why),
    }
}
