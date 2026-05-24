//! Error types produced by source code analyzers during parsing.

use std::fmt;

/// Errors that a `SourceCodeAnalyzer` can return when analysing a source file.
#[allow(dead_code)]
#[derive(Debug)]
pub enum ParserError {
    SyntaxError { message: String, line: Option<usize> },
    UnsupportedLanguage(String),
    IoError(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::SyntaxError { message, line } => match line {
                Some(l) => write!(f, "Syntax error at line {l}: {message}"),
                None => write!(f, "Syntax error: {message}"),
            },
            ParserError::UnsupportedLanguage(ext) => write!(f, "Unsupported language: {ext}"),
            ParserError::IoError(msg) => write!(f, "IO error: {msg}"),
        }
    }
}

impl std::error::Error for ParserError {}
