mod application;
mod delivery;
mod domain;
mod infrastructure;

use std::path::PathBuf;
use std::process;
use crate::delivery::run::run;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let output_root = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("arcuate-docs")
    };

    if let Err(e) = run(&output_root) {
        eprintln!("Error: {e}");
        process::exit(1);
    }

    println!("Done. Output written to {}", output_root.display());
}
