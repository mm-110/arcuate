//! Entry point logic: orchestrates use cases and handles top-level errors.

use std::path::Path;
use crate::delivery::factories::{make_documentation_extractor, make_file_scanner};
use crate::domain::entities::extraction_report::ExtractionReport;
use crate::domain::policies::exclusion_rules::ExclusionRules;

pub fn run(source_root: &Path, output_root: &Path, exclusion_rules: ExclusionRules) -> Result<ExtractionReport, Box<dyn std::error::Error>> {
    let scanner = make_file_scanner();
    let layout = scanner.scan(source_root, &exclusion_rules);

    let extractor = make_documentation_extractor();
    Ok(extractor.extract(&layout, output_root)?)
}
