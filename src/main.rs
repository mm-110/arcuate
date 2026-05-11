mod application;
mod delivery;
mod domain;
mod infrastructure;

use std::path::PathBuf;
use std::process;
use crate::delivery::run::run;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let input_dir = parse_flag(&args, "--input-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("cannot read current directory"));

    let output_dir = parse_flag(&args, "--output-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| input_dir.join("arcuate-docs"));

    if let Err(e) = run(&input_dir, &output_dir) {
        eprintln!("Error: {e}");
        process::exit(1);
    }

    println!("Done. Output written to {}", output_dir.display());
}

/// Returns the value of `--flag <value>` or `--flag=<value>` from the args list.
fn parse_flag(args: &[String], flag: &str) -> Option<String> {
    let prefix = format!("{flag}=");
    for (i, arg) in args.iter().enumerate() {
        if let Some(value) = arg.strip_prefix(&prefix) {
            return Some(value.to_string());
        }
        if arg == flag {
            return args.get(i + 1).cloned();
        }
    }
    None
}
