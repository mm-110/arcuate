//! Orchestrates filesystem scanning and source file analysis.

use std::path::Path;
use crate::domain::entities::parsed_source_file::ParsedSourceFile;
use crate::domain::entities::project_layout::{ProjectEntry, ProjectLayout};
use crate::domain::entities::source_file_analysis::SourceFileAnalysis;
use crate::domain::policies::exclusion_rules::ExclusionRules;
use crate::domain::ports::source_code_analyzer::SourceCodeAnalyzer;

pub struct FileScanner {
    analyzers: Vec<Box<dyn SourceCodeAnalyzer>>,
}

impl FileScanner {
    pub fn new(analyzers: Vec<Box<dyn SourceCodeAnalyzer>>) -> Self {
        Self { analyzers }
    }

    pub fn scan(&self, root: &Path, exclusion_rules: &ExclusionRules) -> ProjectLayout {
        ProjectLayout {
            project_root_path: root.to_path_buf(),
            entries: self.scan_dir(root, exclusion_rules),
        }
    }

    fn scan_dir(&self, dir: &Path, exclusion_rules: &ExclusionRules) -> Vec<ProjectEntry> {
        let mut entries: Vec<ProjectEntry> = Vec::new();

        let read_dir_iter = match std::fs::read_dir(dir) {
            Ok(iter) => iter,
            Err(_) => return entries,
        };

        for entry in read_dir_iter {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path: std::path::PathBuf = entry.path();

            if path.is_dir() {
                if exclusion_rules.excludes_dir(&path) {
                    continue;
                }
                entries.push(ProjectEntry::Dir {
                    path: path.clone(),
                    child_nodes: self.scan_dir(&path, exclusion_rules),
                });
            } else if let Some(parsed) = self.try_analyze(&path, exclusion_rules) {
                entries.push(ProjectEntry::File {
                    path,
                    parsed_source_file: parsed,
                });
            }
        }

        entries
    }

    fn try_analyze(&self, path: &Path, exclusion_rules: &ExclusionRules) -> Option<ParsedSourceFile> {
        if exclusion_rules.excludes_file(path) {
            return None;
        }
        let ext: &str = path.extension()?.to_str()?;
        let analyzer: &Box<dyn SourceCodeAnalyzer> = self.analyzers
            .iter()
            .find(|a: &&Box<dyn SourceCodeAnalyzer>| a.supports_extension(ext))?;
        let source_code: String = std::fs::read_to_string(path).ok()?;
        let analysis: SourceFileAnalysis = analyzer.analyze(&source_code).ok()?;
        Some(ParsedSourceFile {
            source_path: path.to_path_buf(),
            top_level_doc: analysis.top_level_doc,
            documented_constructs: analysis.documented_constructs,
        })
    }
}
