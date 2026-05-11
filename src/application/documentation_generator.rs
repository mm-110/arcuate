//! Use case that writes the mirrored docs and the project index from a scanned layout.

use std::path::Path;
use crate::domain::entities::project_layout::ProjectLayout;
use crate::domain::ports::index_writer::IndexWriter;
use crate::domain::ports::output_writer::OutputWriter;
use crate::domain::ports::output_writer_error::OutputWriterError;

/// Generates the full documentation output: per-file markdown and the global index.
pub struct DocumentationGenerator {
    output_writer: Box<dyn OutputWriter>,
    index_writer: Box<dyn IndexWriter>,
}

impl DocumentationGenerator {
    pub fn new(output_writer: Box<dyn OutputWriter>, index_writer: Box<dyn IndexWriter>) -> Self {
        Self { output_writer, index_writer }
    }

    pub fn generate(&self, layout: &ProjectLayout, output_root: &Path) -> Result<(), OutputWriterError> {
        self.output_writer.write(layout, output_root)?;
        self.index_writer.write_index(layout, output_root)?;
        Ok(())
    }
}
