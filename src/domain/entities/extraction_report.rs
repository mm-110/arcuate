//! Summary of a documentation extraction run.

use std::path::PathBuf;

/// Statistics produced after writing the full documentation output.
pub struct ExtractionReport {
    pub output_path: PathBuf,
    pub files_written: usize,
    pub total_chars: usize,
}