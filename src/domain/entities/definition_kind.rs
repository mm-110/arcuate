//! Classifies the kind of construct a semantic entity represents.

/// The category of a documented construct within a source file.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionKind {
    /// The file itself; may carry a module-level docstring.
    Module,
    /// A class or struct that contains methods and attributes.
    Class,
    /// A standalone function or a method inside a class.
    Function,
    /// A relevant global constant or configuration value.
    Constant,
}
