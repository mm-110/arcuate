//! Markdown writer that mirrors a project layout into .md files.
use std::path::Path;
use crate::domain::entities::documented_construct::DocumentedConstruct;
use crate::domain::entities::extraction_report::ExtractionReport;
use crate::domain::entities::parsed_source_file::ParsedSourceFile;
use crate::domain::entities::project_layout::{ ProjectLayout, ProjectEntry };
use crate::domain::ports::index_writer::IndexWriter;
use crate::domain::ports::output_writer::OutputWriter;
use crate::domain::ports::output_writer_error::OutputWriterError;

pub struct MarkdownWriter;

impl MarkdownWriter {
    fn render_markdown(&self, file: &ParsedSourceFile) -> String {
        let file_name = file.source_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        let title = match file.top_level_doc.as_deref().and_then(|d| d.lines().find(|l| !l.trim().is_empty())) {
            Some(first_line) => format!("# {file_name} — {first_line}\n\n"),
            None => format!("# {file_name}\n\n"),
        };

        let mut out = title;
        for construct in &file.documented_constructs {
            self.render_construct(&mut out, construct, 2);
        }
        out
    }

    fn render_construct(&self, out: &mut String, construct: &DocumentedConstruct, level: usize) {
        let heading = "#".repeat(level);

        if let Some(sig) = &construct.signature {
            out.push_str(&format!("{heading} `{sig}`\n\n"));
        } else {
            out.push_str(&format!("{heading} `{}`\n\n", construct.name));
        }

        if let Some(doc) = &construct.docstring {
            out.push_str(&format!("> {doc}\n\n"));
        }

        for nested in &construct.nested_constructs {
            self.render_construct(out, nested, level + 1);
        }
    }

    fn write_entries(&self, entries: &[ProjectEntry], project_root_path: &Path, output_root: &Path) -> Result<(usize, usize), OutputWriterError> {
        let mut files_written = 0;
        let mut total_chars = 0;

        for entry in entries {
            match entry {
                ProjectEntry::File { path, parsed_source_file } => {
                    let relative_path = path.strip_prefix(project_root_path).unwrap();
                    let output_path = output_root.join(relative_path).with_extension("md");
                    if let Some(parent) = output_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    let content = self.render_markdown(parsed_source_file);
                    total_chars += content.len();
                    std::fs::write(&output_path, content)?;
                    files_written += 1;
                }
                ProjectEntry::Dir { path, child_nodes } => {
                    let relative_path = path.strip_prefix(project_root_path).unwrap();
                    let output_path = output_root.join(relative_path);
                    std::fs::create_dir_all(&output_path)?;
                    let (sub_files, sub_chars) = self.write_entries(child_nodes, project_root_path, output_root)?;
                    files_written += sub_files;
                    total_chars += sub_chars;
                }
            }
        }

        Ok((files_written, total_chars))
    }

    fn render_index(&self, layout: &ProjectLayout) -> String {
        let mut out = String::from("# Index\n\n");
        self.render_index_entries(&mut out, &layout.entries, &layout.project_root_path, 0);
        out
    }

    fn render_index_entries(&self, out: &mut String, entries: &[ProjectEntry], project_root: &Path, depth: usize) {
        let indent = "  ".repeat(depth);

        let mut dirs: Vec<&ProjectEntry> = entries.iter()
            .filter(|e| matches!(e, ProjectEntry::Dir { .. }))
            .collect();
        dirs.sort_by_key(|e| match e {
            ProjectEntry::Dir { path, .. } => path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
            _ => String::new(),
        });

        let mut files: Vec<&ProjectEntry> = entries.iter()
            .filter(|e| match e {
                ProjectEntry::File { path, .. } => path.file_name().map(|n| n != "__init__.py").unwrap_or(true),
                _ => false,
            })
            .collect();
        files.sort_by_key(|e| match e {
            ProjectEntry::File { path, .. } => path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
            _ => String::new(),
        });

        for entry in dirs {
            if let ProjectEntry::Dir { path, child_nodes } = entry {
                let relative = path.strip_prefix(project_root).unwrap();
                let dir_doc = init_doc(child_nodes)
                    .map(|d| format!(" — {d}"))
                    .unwrap_or_default();
                out.push_str(&format!("{indent}- **{}/**{dir_doc}\n", relative.to_string_lossy()));
                self.render_index_entries(out, child_nodes, project_root, depth + 1);
            }
        }

        for entry in files {
            if let ProjectEntry::File { path, parsed_source_file } = entry {
                let relative = path.strip_prefix(project_root).unwrap();
                let md_path = relative.with_extension("md").to_string_lossy().into_owned();
                let file_name = relative.file_name().unwrap_or_default().to_string_lossy().into_owned();
                let file_doc = parsed_source_file.top_level_doc.as_deref()
                    .map(|d| d.lines().filter(|l| !l.trim().is_empty()).collect::<Vec<_>>().join(" "))
                    .filter(|s| !s.is_empty())
                    .map(|d| format!(" — {d}"))
                    .unwrap_or_default();
                out.push_str(&format!("{indent}- [{file_name}]({md_path}){file_doc} · [source]({})\n", path.display()));
            }
        }
    }
}

/// Extracts all non-empty lines of the docstring from an `__init__.py` within the given entries.
fn init_doc(entries: &[ProjectEntry]) -> Option<String> {
    entries.iter().find_map(|e| match e {
        ProjectEntry::File { path, parsed_source_file }
            if path.file_name().map(|n| n == "__init__.py").unwrap_or(false) =>
        {
            let lines: Vec<&str> = parsed_source_file.top_level_doc.as_deref()?
                .lines()
                .filter(|l| !l.trim().is_empty())
                .collect();
            if lines.is_empty() { None } else { Some(lines.join(" ")) }
        }
        _ => None,
    })
}

impl OutputWriter for MarkdownWriter {
    fn write(&self, layout: &ProjectLayout, output_root: &Path) -> Result<ExtractionReport, OutputWriterError> {
        let (files_written, total_chars) = self.write_entries(&layout.entries, &layout.project_root_path, output_root)?;
        Ok(ExtractionReport {
            output_path: output_root.to_path_buf(),
            files_written,
            total_chars,
        })
    }
}

impl IndexWriter for MarkdownWriter {
    fn write_index(&self, layout: &ProjectLayout, output_root: &Path) -> Result<ExtractionReport, OutputWriterError> {
        let content = self.render_index(layout);
        std::fs::create_dir_all(output_root)?;
        std::fs::write(output_root.join("INDEX.md"), &content)?;
        Ok(ExtractionReport {
            output_path: output_root.to_path_buf(),
            files_written: 1,
            total_chars: content.len(),
        })
    }
}
