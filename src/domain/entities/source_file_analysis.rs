//! The raw semantic output produced by analyzing a source file.

use super::documented_construct::DocumentedConstruct;

/// Semantic information extracted from source code, before being associated with a file path.
pub struct SourceFileAnalysis {
    pub top_level_doc: Option<String>,
    pub documented_constructs: Vec<DocumentedConstruct>,
}