//! Contract that every language analyzer must satisfy.

use super::parser_error::ParserError;
use crate::domain::entities::source_file_analysis::SourceFileAnalysis;

/// Interface for language-specific analyzers that extract documented constructs from source code.
pub trait SourceCodeAnalyzer {
    fn supports_extension(&self, ext: &str) -> bool;
    fn analyze(&self, source_code: &str) -> Result<SourceFileAnalysis, ParserError>;
}
