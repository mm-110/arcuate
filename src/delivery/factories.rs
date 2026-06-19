//! Factory functions that wire adapters into use cases.

use crate::application::documentation_extractor::DocumentationExtractor;
use crate::application::file_scanner::FileScanner;
use crate::infrastructure::markdown_writer::MarkdownWriter;
use crate::infrastructure::python_parser::PythonParser;

/// Builds a FileScanner pre-loaded with all supported language analyzers.
pub fn make_file_scanner() -> FileScanner {
    FileScanner::new(vec![Box::new(PythonParser)])
}

/// Builds a DocumentationExtractor wired with the markdown writer for both docs and index.
pub fn make_documentation_extractor() -> DocumentationExtractor {
    DocumentationExtractor::new(
        Box::new(MarkdownWriter),
        Box::new(MarkdownWriter),
    )
}
