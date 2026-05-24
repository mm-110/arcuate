//! The atomic unit of semantic information extracted from source code.

use super::definition_kind::DefinitionKind;

/// A single documented construct extracted from a source file.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DocumentedConstruct {
    pub name: String,
    pub kind: DefinitionKind,
    pub signature: Option<String>,
    pub docstring: Option<String>,
    pub source_line_range: Option<(usize, usize)>,
    pub nested_constructs: Vec<DocumentedConstruct>,
}
