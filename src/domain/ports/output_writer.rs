//! Contract that every markdown writer must satisfy.

use std::path::Path;
use crate::domain::entities::project_layout::ProjectLayout;
use super::output_writer_error::OutputWriterError;
use crate::domain::entities::extraction_report::ExtractionReport;

/// Interface for writing the mirrored markdown output from a scanned project layout.
pub trait OutputWriter {
    fn write(&self, layout: &ProjectLayout, output_root: &Path) -> Result<ExtractionReport, OutputWriterError>;
}
