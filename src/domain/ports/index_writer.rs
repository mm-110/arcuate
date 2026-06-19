//! Contract for writing the project index.

use std::path::Path;
use crate::domain::entities::project_layout::ProjectLayout;
use crate::domain::entities::extraction_report::ExtractionReport;
use super::output_writer_error::OutputWriterError;

/// Interface for writing the INDEX.md entry point for a scanned project.
pub trait IndexWriter {
    fn write_index(&self, layout: &ProjectLayout, output_root: &Path) -> Result<ExtractionReport, OutputWriterError>;
}
