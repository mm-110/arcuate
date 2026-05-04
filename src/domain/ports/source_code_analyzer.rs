//! Contract that every language analyzer must satisfy.

use crate::domain::entities::documented_construct::DocumentedConstruct;
use super::parser_error::ParserError;

/// Interface for language-specific analyzers that extract documented constructs from source code.
pub trait SourceCodeAnalyzer {
    fn supports_extension(&self, ext: &str) -> bool;
    fn analyze(&self, source_code: &str) -> Result<Vec<DocumentedConstruct>, ParserError>;
}
