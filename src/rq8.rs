mod application;
mod delivery;
mod domain;
mod infrastructure;

use std::path::PathBuf;
use std::process;
use crate::delivery::mappers::cli_mapper::{exclusion_rules_from_args, flag_value, validate_args};
use crate::delivery::run::run;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("Usage: rq8 [--input-dir <path>] [--output-dir <path>] [exclusion flags]");
        println!();
        println!("Options:");
        println!("  --input-dir              Root of the project to scan (default: current directory)");
        println!("  --output-dir             Where to write the extracted Markdown (default: <input-dir>/<input-dir-name>_rq8_docs)");
        println!("  --exclude-dirs           Comma-separated directory names to exclude (e.g. target,node_modules)");
        println!("  --exclude-files          Comma-separated file names to exclude (e.g. __init__.py)");
        println!("  --exclude-dir-pattern    Comma-separated prefixes: exclude dirs whose name starts with these (e.g. .,test)");
        println!("  --exclude-file-pattern   Comma-separated prefixes: exclude files whose name starts with these (e.g. .,test_)");
        println!("  --exclude-hidden         Exclude all files and directories starting with .");
        process::exit(0);
    }

    if let Err(e) = validate_args(&args) {
        eprintln!("Error: {e}");
        eprintln!("Run 'rq8 --help' for usage.");
        process::exit(1);
    }

    let input_dir = flag_value(&args, "--input-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("cannot read current directory"));

    let output_dir = flag_value(&args, "--output-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let folder_name = input_dir.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("project");
            input_dir.join(format!("{folder_name}_rq8_docs"))
        });

    let exclusion_rules = exclusion_rules_from_args(&args);

    match run(&input_dir, &output_dir, exclusion_rules) {
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
        Ok(report) => {
            println!("Output: {}", report.output_path.display());
            println!("Files:  {}", report.files_written);
            println!("Chars:  {}", report.total_chars);
            println!("Tokens: ~{}", report.total_chars / 4);
        }
    }
}
