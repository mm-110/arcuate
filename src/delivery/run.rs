//! Entry point logic: orchestrates use cases and handles top-level errors.

use std::path::Path;
use crate::delivery::factories::{make_documentation_generator, make_file_scanner};

pub fn run(output_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source_root = std::env::current_dir()?;

    let scanner = make_file_scanner();
    let layout = scanner.scan(&source_root);

    let generator = make_documentation_generator();
    generator.generate(&layout, output_root)?;

    Ok(())
}
