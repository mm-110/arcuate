//! Layout structure that mirrors the source filesystem hierarchy.

use std::path::PathBuf;
use super::parsed_source_file::ParsedSourceFile;

/// An entry in the project layout, either a directory or a parsed source file.
#[derive(Debug, Clone)]
pub enum ProjectEntry {
    File {
        path: PathBuf,
        parsed_source_file: ParsedSourceFile,
    },
    Dir {
        path: PathBuf,
        child_nodes: Vec<ProjectEntry>,
    },
}

/// The complete layout of a scanned project, used by the writer to produce mirrored output.
#[derive(Debug, Clone)]
pub struct ProjectLayout {
    pub project_root_path: PathBuf,
    pub entries: Vec<ProjectEntry>,
}
