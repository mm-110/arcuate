//! Rules that control which directories and files are excluded during scanning.

/// Defines which paths the scanner should skip.
#[derive(Default)]
pub struct ExclusionRules {
    pub dirs: Vec<String>,
    pub files: Vec<String>,
    pub dir_starts_with_patterns: Vec<String>,
    pub file_starts_with_patterns: Vec<String>,
}

impl ExclusionRules {
    pub fn excludes_dir(&self, path: &std::path::Path) -> bool {
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            return false;
        };
        self.dirs.iter().any(|d| d == name)
            || self.dir_starts_with_patterns.iter().any(|p| name.starts_with(p.as_str()))
    }

    pub fn excludes_file(&self, path: &std::path::Path) -> bool {
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            return false;
        };
        self.files.iter().any(|f| f == name)
            || self.file_starts_with_patterns.iter().any(|p| name.starts_with(p.as_str()))
    }
}
