//! Use case that writes the mirrored docs and the project index from a scanned layout.

use std::path::Path;
use crate::domain::entities::extraction_report::ExtractionReport;
use crate::domain::entities::project_layout::ProjectLayout;
use crate::domain::ports::index_writer::IndexWriter;
use crate::domain::ports::output_writer::OutputWriter;
use crate::domain::ports::output_writer_error::OutputWriterError;

/// Extracts and writes the full documentation output: per-file markdown and the global index.
pub struct DocumentationExtractor {
    output_writer: Box<dyn OutputWriter>,
    index_writer: Box<dyn IndexWriter>,
}

impl DocumentationExtractor {
    pub fn new(output_writer: Box<dyn OutputWriter>, index_writer: Box<dyn IndexWriter>) -> Self {
        Self { output_writer, index_writer }
    }

    pub fn extract(&self, layout: &ProjectLayout, output_root: &Path) -> Result<ExtractionReport, OutputWriterError> {
        let doc_report = self.output_writer.write(layout, output_root)?;
        let index_report = self.index_writer.write_index(layout, output_root)?;
        Ok(ExtractionReport {
            output_path: output_root.to_path_buf(),
            files_written: doc_report.files_written + index_report.files_written,
            total_chars: doc_report.total_chars + index_report.total_chars,
        })
    }
}
