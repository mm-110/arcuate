//! The semantic representation of a single source file.

use std::path::PathBuf;
use super::documented_construct::DocumentedConstruct;

/// All semantic information extracted from one source file by a `SourceCodeAnalyzer`.
#[derive(Debug, Clone)]
pub struct ParsedSourceFile {
    pub source_path: PathBuf,
    pub top_level_doc: Option<String>,
    pub documented_constructs: Vec<DocumentedConstruct>,
}
